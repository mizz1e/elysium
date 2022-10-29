//#![deny(warnings)]
#![allow(dead_code)]
#![feature(abi_thiscall)]
#![feature(arbitrary_self_types)]
#![feature(bound_map)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(mem_copy_fn)]
#![feature(iter_intersperse)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_sub_ptr)]
#![feature(result_option_inspect)]
#![feature(sync_unsafe_cell)]
#![feature(strict_provenance)]

use error::Error;
use state::{CreateMove, DrawModel, FrameStageNotify, OverrideView, PollEvent, SwapWindow};

use std::borrow::Cow;
use std::ffi::CStr;
use std::time::Instant;
use std::{env, mem, ptr, thread};

use elysium_framework::Framework;
use elysium_sdk::material;
use elysium_sdk::material::Materials;
use elysium_sdk::networked;
use elysium_sdk::{Interface, InterfaceKind, LibraryKind, Vars, Vdf};

pub use options::Options;
pub use state::State;

mod error;
mod options;
mod ui;

pub mod anti_aim;
pub mod assets;
pub mod entity;
pub mod hooks;
pub mod launcher;
pub mod library;
pub mod pattern;
pub mod state;
pub mod util;

const fn const_cstr(string: &'static str) -> Cow<'static, CStr> {
    unsafe { Cow::Borrowed(CStr::from_bytes_with_nul_unchecked(string.as_bytes())) }
}

fn main() {
    env_logger::init();

    if let Err(error) = run() {
        log::error!("failed to even run: {error}");
    }
}

fn run() -> Result<(), Error> {
    // CLI options, duh!
    let options = Options::parse();

    // Why even try to continue if you don't even have the game?
    // Also, we *really* need this path for below.
    let csgo_dir = util::determine_csgo_dir().ok_or(Error::NoCsgo)?;

    // Automatically append "LD_LIBRARY_PATH" otherwise CSGO can't find any libraries!
    util::check_linker_path(&csgo_dir)?;

    // X11 `DISPLAY` sanity check as CSGO prefers to segmentation fault.
    //
    // In the future, this shouldn't be needed anymore (what v2 aims to implement).
    env::var_os("DISPLAY").ok_or(Error::NoDisplay)?;

    // Spawn a seperate thread in order to hook functions in modules.
    thread::spawn(setup);

    // Actually launch the game (bin/linux64/launcher_client.so).
    let mut framework = Framework::new();

    let tier0 = unsafe { libloading::Library::new("libtier0_client.so").unwrap() };

    unsafe {
        let command_line: unsafe extern "C" fn() -> Ptr<'static, CommandLine> =
            *tier0.get(b"CommandLine\0").unwrap();

        let mut args = vec!["-game", "csgo"];

        if options.vulkan {
            args.push("-vulkan");
        } else {
            args.push("-opengl");
        }

        command_line().parse(args);

        framework.load("client_client.so")?;
        framework.load("engine_client.so")?;
        framework.load("filesystem_stdio_client.so")?;
        framework.load("libvstdlib_client.so")?;
        framework.load("materialsystem_client.so")?;
        framework.load("studiorender_client.so")?;
    }

    let mut cvar_query =
        unsafe { framework.new_interface::<()>("engine_client.so", "VCvarQuery001")? };

    let mut filesystem =
        unsafe { framework.new_interface::<()>("filesystem_stdio_client.so", "VFileSystem017")? };

    let mut cvar =
        unsafe { framework.new_interface::<()>("libvstdlib_client.so", "VEngineCvar007")? };

    let materialsystem =
        unsafe { framework.new_interface::<()>("materialsystem_client.so", "VMaterialSystem080")? };

    unsafe {
        framework.link(
            PtrMut::clone(&cvar),
            &[("VCvarQuery001", PtrMut::clone(&cvar_query))],
        )?;
    }

    unsafe {
        let module = link::load_module("client_client.so").unwrap();
        let bytes = module.bytes();
        let opcode = &pattern::VDF_FROM_BYTES.find(bytes).unwrap().1[..5];

        log::trace!("vdf from_bytes = {opcode:02X?}");

        let ip = opcode.as_ptr().byte_add(1);
        let reladdr = ip.cast::<i32>().read() as isize;
        let absaddr = ip.byte_add(4).byte_offset(reladdr);

        Vdf::set_from_bytes(mem::transmute(absaddr));
    }

    unsafe {
        let materialsystem = PtrMut::clone(&materialsystem).cast::<MaterialSystem>();
        let kind = materialsystem.shader_api_kind();

        log::trace!("Shader API: {kind:?}");

        let materials: &mut Materials = std::mem::transmute(materialsystem);

        // load what we need
        materials.init();

        unsafe {
            materials.hook_create(hooks::create_material);
            materials.hook_find(hooks::find_material);
        }
    }

    launcher::launch(options)?;

    Ok(())
}

use libloading::Library;

fn glx() -> Option<()> {
    log::trace!("finding glx proc address");

    let glx = unsafe { Library::new("libGLX.so").ok()? };
    let proc_addr = unsafe { *glx.get(b"glXGetProcAddress\0").ok()? };

    log::trace!("found glx proc address");

    State::get().proc_address = proc_addr;

    Some(())
}

fn sdl() -> Option<()> {
    log::trace!("finding sdl symbols");

    let glx = unsafe { Library::new("libSDL2-2.0.so.0").ok()? };
    let poll_event = unsafe { *glx.get(b"SDL_PollEvent\0").ok()? };
    let swap_window = unsafe { *glx.get(b"SDL_GL_SwapWindow\0").ok()? };

    log::trace!("found sdl symbols, finding original methods...");

    let swap_window = unsafe { elysium_mem::next_abs_addr_mut_ptr::<SwapWindow>(swap_window)? };
    let poll_event = unsafe { elysium_mem::next_abs_addr_mut_ptr::<PollEvent>(poll_event)? };

    log::trace!("found sdl original methods, hooking...");

    let state = State::get();

    state.hooks.swap_window = Some(unsafe { swap_window.replace(hooks::swap_window) });
    state.hooks.poll_event = Some(unsafe { poll_event.replace(hooks::poll_event) });

    Some(())
}

fn setup() -> Result<(), Error> {
    let now = Instant::now();

    // Wait for SDL to load before hooking.
    //util::sleep_until(util::is_sdl_loaded);

    log::trace!("sdl loaded. took {:?}", now.elapsed());

    let state = State::get();

    // TODO: Clean this mess up.
    use std::collections::HashSet;
    state.world = Some(HashSet::new());
    state.blur = Some(HashSet::new());
    state.blur_static = Some(HashSet::new());
    state.init_time = Some(std::time::Instant::now());

    glx();
    sdl();

    util::sleep_until(util::is_browser_loaded);

    log::trace!("server browser loaded. took {:?}", now.elapsed());

    unsafe {
        let interfaces = library::load_interfaces();
        let state = State::get();

        state.interfaces = Some(interfaces);

        let interfaces = state.interfaces.as_mut().unwrap_unchecked();

        let show = interfaces.game_console.show_address();

        unsafe extern "C" fn show_hook(this: *const ()) {
            println!("show console");
        }

        elysium_mem::unprotect(show, |show, prot| {
            show.replace(show_hook as *const ());
            prot
        });

        let console = &mut interfaces.console;
        let client = &interfaces.client;
        let model_render = &interfaces.model_render;
        let materials = &interfaces.materials;

        let globals = &mut *(client.globals() as *mut _);
        let input = &mut *(client.input() as *mut _);

        console.write(format_args!("welcome to elysium\n")).unwrap();

        let vars = Vars::from_loader(|kind| {
            let name = kind.name();
            let var = console.var::<_, i32>(name);

            match var {
                Some(var) => {
                    let pointer = var as *const _ as _;

                    log::info!("convar {name} found at {pointer:?}");

                    pointer
                }
                None => {
                    log::info!("convar {name} missing :warning:");

                    ptr::null_mut()
                }
            }
        });

        state.globals = Some(globals);
        state.input = Some(input);
        state.vars = vars.ok();

        networked::init(&client);

        let address = client.create_move_address().cast::<CreateMove>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.create_move = Some(address.replace(hooks::create_move));
            log::info!("hooked clientmode createmove");
            prot
        });

        let address = model_render.draw_model_address().cast::<DrawModel>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.draw_model = Some(address.replace(hooks::draw_model));
            log::info!("hooked modelrender dme");
            prot
        });

        let address = client
            .frame_stage_notify_address()
            .cast::<FrameStageNotify>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.frame_stage_notify = Some(address.replace(hooks::frame_stage_notify));
            log::info!("hooked client fsn");
            prot
        });

        let address = client.override_view_address().cast::<OverrideView>();

        elysium_mem::unprotect(address, |address, prot| {
            state.hooks.override_view = Some(address.replace(hooks::override_view));
            log::info!("hooked clientmode overrideview");
            prot
        });
    }

    Ok(())
}

#[repr(C)]
pub struct CommandLineVTable {
    _pad0: MaybeUninit<[unsafe extern "C" fn(); 1]>,
    parse: unsafe extern "C" fn(
        command_line: Ptr<'_, CommandLine>,
        len: ffi::c_int,
        args: *const *const ffi::c_char,
    ),
}

#[repr(C)]
pub struct CommandLine {
    vtable: &'static CommandLineVTable,
}

impl CommandLine {
    pub unsafe fn parse<I, S>(self: Ptr<'_, CommandLine>, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args = args
            .into_iter()
            .map(|arg| std::ffi::CString::new(arg.as_ref()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .chain(Some(std::ptr::null()))
            .collect::<Vec<_>>();

        let len = args.len() - 1;

        unsafe {
            (self.vtable.parse)(self, len as ffi::c_int, args.as_ptr());
        }
    }
}

use elysium_ptr::{Ptr, PtrMut};
use std::ffi;
use std::mem::MaybeUninit;

type Factory = unsafe extern "C" fn(name: *const ffi::c_char, result: *mut i32) -> *mut u8;

unsafe extern "C" fn factory(_name: *const ffi::c_char, _result: *mut i32) -> *mut u8 {
    println!("factory called");

    std::ptr::null_mut()
}

unsafe extern "C" fn create_proxy(
    _material_proxy: Ptr<'_, MaterialProxyFactory>,
    _proxy_name: *const ffi::c_char,
) -> *mut u8 {
    println!("create proxy called");

    std::ptr::null_mut()
}

unsafe extern "C" fn delete_proxy(_material_proxy: Ptr<'_, MaterialProxyFactory>, _proxy: *mut u8) {
    println!("delete proxy called");
}

#[repr(C)]
struct MaterialProxyFactoryVTable {
    create_proxy: unsafe extern "C" fn(
        material_proxy: Ptr<'_, MaterialProxyFactory>,
        proxy_name: *const ffi::c_char,
    ) -> *mut u8,
    delete_proxy:
        unsafe extern "C" fn(material_proxy: Ptr<'_, MaterialProxyFactory>, proxy: *mut u8),
    get_factory: Factory,
}

#[repr(C)]
struct MaterialProxyFactory {
    vtable: &'static MaterialProxyFactoryVTable,
}

const MATERIAL_PROXY_FACTORY: MaterialProxyFactory = MaterialProxyFactory {
    vtable: &MaterialProxyFactoryVTable {
        create_proxy,
        delete_proxy,
        get_factory: factory,
    },
};

#[repr(C)]
struct MaterialSystemVTable {
    _pad0: MaybeUninit<[unsafe extern "C" fn(); 3]>,
    _init: unsafe extern "C" fn(material_system: PtrMut<'_, MaterialSystem>) -> u32,
    _pad1: MaybeUninit<[unsafe extern "C" fn(); 6]>,
    init: unsafe extern "C" fn(
        material_system: PtrMut<'_, MaterialSystem>,
        api: *const ffi::c_char,
        material_proxy_factory: *const MaterialProxyFactory,
        file_system_factory: Factory,
        cvar_factory: Factory,
    ) -> *const u8,
    _pad2: MaybeUninit<[unsafe extern "C" fn(); 153]>,
    shader_api_kind: unsafe extern "C" fn(material_system: PtrMut<'_, MaterialSystem>) -> u32,
}

#[repr(C)]
struct MaterialSystem {
    vtable: &'static MaterialSystemVTable,
    _pad0: MaybeUninit<[u8; 1561]>,
    adapter: u8,             // 0x621
    configuration_flags: u8, // 0x622
    _pad1: MaybeUninit<[u8; 25]>,
    requested_editor_materials: u8, // 0x63C
    _pad2: MaybeUninit<[u8; 158]>,
    shader_api_kind: [u8; 4], // 0x6DB
    _pad3: MaybeUninit<[u8; 10796]>,
    adapter_flags: u32, // 0x310C
    _pad4: MaybeUninit<[u8; 209]>,
    requested_g_buffers: u8, // 0x31E1
}

use std::fmt;

impl fmt::Debug for MaterialSystem {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("MaterialSystem")
            .field("vtable", &(self.vtable as *const MaterialSystemVTable))
            .field("adapter", &self.adapter)
            .field("configuration_flags", &self.configuration_flags)
            .field(
                "requested_editor_materials",
                &self.requested_editor_materials,
            )
            .field("shader_api_kind", &self.shader_api_kind)
            .field("adapter_flags", &self.adapter_flags)
            .field("requested_g_buffers", &self.requested_g_buffers)
            .finish()
    }
}

/// The shader API material system uses.
///
/// Refer to `material_system + 0x6DB`, a 32-bit integer.
#[derive(Debug)]
#[repr(u32)]
pub enum ShaderApiKind {
    /// Use DX9 (OpenGL via ToGL).
    ///
    /// Enabled via checking if `CommandLine->FindParm("-opengl")` is non-zero, then setting the
    /// shader API value to `3`.
    ///
    /// XREF `"-opengl"` in `bin/linux64/materialsystem_client.so`.
    Dx9 = 3,

    /// Use nothing.
    ///
    /// Enabled via checking if `CommandLine->FindParm("-noshaderapi")` is non-zero, then setting the
    /// shader API value to `4`.
    ///
    /// XREF `"-noshaderapi"` in `bin/linux64/materialsystem_client.so`.
    Empty = 4,

    /// Use Vulkan.
    ///
    /// Enabled via checking if `CommandLine->FindParm("-vulkan")` is non-zero, then setting the
    /// shader API value to `2`.
    ///
    /// XREF `"-vulkan"` in `bin/linux64/materialsystem_client.so`.
    Vulkan = 2,
}

impl ShaderApiKind {
    pub const fn from_raw(value: u32) -> Option<Self> {
        if matches!(value, 2 | 3 | 4) {
            Some(unsafe { Self::from_raw_unchecked(value) })
        } else {
            None
        }
    }

    pub const unsafe fn from_raw_unchecked(value: u32) -> Self {
        match value {
            2 => ShaderApiKind::Dx9,
            3 => ShaderApiKind::Vulkan,
            4 => ShaderApiKind::Empty,
            _ => std::hint::unreachable_unchecked(),
        }
    }
}

impl MaterialSystem {
    /// Initialize the material system.
    pub unsafe fn _init(self: &PtrMut<'_, MaterialSystem>) -> Result<(), ()> {
        let result = (self.vtable._init)(PtrMut::clone(self));

        if result == 1 {
            Ok(())
        } else {
            Err(())
        }
    }

    /// Returns the current shader API kind.
    pub unsafe fn shader_api_kind(self: &PtrMut<'_, MaterialSystem>) -> ShaderApiKind {
        let kind = (self.vtable.shader_api_kind)(PtrMut::clone(self));

        ShaderApiKind::from_raw(kind).unwrap()
    }

    /// Set the shader API module name.
    ///
    /// Must be one of `shaderapidx9`, `shaderapivk`, or `shaderapiempty`.
    ///
    /// # Safety
    ///
    /// Must be called before [`init`].
    pub unsafe fn set_shader_api(self: &PtrMut<'_, MaterialSystem>, api: &str) -> Result<(), ()> {
        let _api = std::ffi::CString::new(api).map_err(|_| ())?;

        (self.vtable.init)(
            PtrMut::clone(self),
            std::ptr::null(),
            //api.as_ptr(),
            &MATERIAL_PROXY_FACTORY,
            factory,
            factory,
        );

        Ok(())
    }
}
