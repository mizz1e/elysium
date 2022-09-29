use crate::model::{ModelInfo, ModelRender};
use crate::{Client, Engine, EntityList, GameConsole, InputSystem, Trace};
use crate::{Console, Debug, Effects, Events, Filesystem, InputInternal};
use crate::{Kinds, Localize, MaterialSystem, Movement, Panel, Panorama, Physics};
use crate::{Prediction, Sound, Surface, VGui};
use cake::ffi::CUtf8Str;
use std::{fmt, mem, ptr};

macro_rules! libraries {
    ($($name:ident => $string:literal),*) => {
        /// library
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum LibraryKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $name,
            )*
        }

        impl LibraryKind {
            /// Return's the path of this library.
            #[inline]
            pub const fn path(&self) -> &'static str {
                match self {
                    $(LibraryKind::$name => $string,)*
                }
            }
        }
    }
}

macro_rules! interfaces {
    ($(($ident:ident, $field:ident) => ($library:ident, $string:literal)),*) => {
        /// interface
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum InterfaceKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $ident,
            )*
        }

        impl InterfaceKind {
            /// Returns the interface's name.
            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(InterfaceKind::$ident => $string,)*
                }
            }

            /// Returns the library this interface is in.
            #[inline]
            pub const fn library(&self) -> LibraryKind {
                match self {
                    $(InterfaceKind::$ident => LibraryKind::$library,)*
                }
            }
        }

        //#[derive(Debug)]
        #[non_exhaustive]
        pub struct Interfaces {
            $(pub $field: &'static mut $ident,)*
        }

        impl Interfaces {
            #[inline]
            pub unsafe fn from_loader<L>(mut loader: L) -> Self
            where
                L: FnMut(InterfaceKind) -> *mut u8,
            {
                Self { $($field: &mut *loader(InterfaceKind::$ident).cast(),)* }
            }
        }
    }
}

libraries! {
    Client => "./csgo/bin/linux64/client_client.so",
    Engine => "./bin/linux64/engine_client.so",
    Filesystem => "./bin/linux64/filesystem_stdio_client.so",
    Input => "./bin/linux64/inputsystem_client.so",
    Localize => "./bin/linux64/localize_client.so",
    Matchmaking => "./csgo/bin/linux64/matchmaking_client.so",
    MaterialSystem => "./bin/linux64/materialsystem_client.so",
    Panorama => "./bin/linux64/panorama_gl_client.so",
    Physics => "./bin/linux64/vphysics_client.so",
    ServerBrowser => "./bin/linux64/serverbrowser_client.so",
    Surface => "./bin/linux64/vguimatsurface_client.so",
    Tier0 => "./bin/linux64/libtier0_client.so",
    VGui => "./bin/linux64/vgui2_client.so"
}

interfaces! {
    (GameConsole, game_console) => (Client, "GameConsole004"),
    (Client, client) => (Client, "VClient"),
    (Console, console) => (MaterialSystem, "VEngineCvar"),
    (Debug, debug) => (Engine, "VDebugOverlay"),
    (Effects, effects) => (Engine, "VEngineEffects"),
    (Engine, engine) => (Engine, "VEngineClient"),
    (EntityList, entity_list) => (Client, "VClientEntityList"),
    (Events, events) => (Engine, "GAMEEVENTSMANAGER002"),
    (Filesystem, filesystem) => (Filesystem, "VFileSystem"),
    (InputInternal, input_internal) => (VGui, "VGUI_InputInternal"),
    (InputSystem, input_system) => (Input, "InputSystemVersion"),
    (Kinds, kinds) => (Matchmaking, "VENGINE_GAMETYPES_VERSION002"),
    (Localize, localize) => (Localize, "Localize_"),
    (MaterialSystem, material_system) => (MaterialSystem, "VMaterialSystem"),
    (ModelInfo, model_info) => (Engine, "VModelInfoClient"),
    (ModelRender, model_render) => (Engine, "VEngineModel"),
    (Movement, movement) => (Client, "GameMovement"),
    (Panel, panel) => (VGui, "VGUI_Panel"),
    (Panorama, panorama) => (Panorama, "PanoramaUIEngine001"),
    (Physics, physics) => (Physics, "VPhysicsSurfaceProps"),
    (Prediction, prediction) => (Client, "VClientPrediction001"),
    (Sound, sound) => (Engine, "IEngineSoundClient"),
    (Surface, surface) => (Surface, "VGUI_Surface"),
    (Trace, trace) => (Engine, "EngineTraceClient"),
    (VGui, vgui) => (Engine, "VEngineVGui")
}

#[inline]
fn is_exact(target: &str) -> bool {
    target.chars().rev().take(3).all(char::is_numeric)
}

/// An interface.
#[repr(C)]
pub struct Interface {
    new: unsafe extern "C" fn() -> *mut u8,
    name: CUtf8Str<'static>,
    next: *const Interface,
}

impl Interface {
    #[inline]
    pub fn new(&self) -> *mut u8 {
        unsafe {
            let interface = (self.new)();
            //let vtable = &mut **(interface as *mut *mut crate::app_system::AppSystemVTable<u8>);

            let name = self.name();

            /*let ha = !matches!(name, "GameUI011"
            | "GameMovement001"
            | "CustomSteamImageOnModel_IMaterialProxy003"
            | "ItemTintColor_IMaterialProxy003"
            | "IEffects001"
            | "ClientAlphaPropertyMgrV001"
            | "ClientLeafSystem002"
            | "VClientEntityList003");*/

            //if ha {
            println!("{:?}", name);
            //println!("{:?}", vtable);
            //println!("{:?}", vtable.tier(interface));
            //}

            interface
        }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn iter(&self) -> InterfaceIter<'_> {
        InterfaceIter { iter: Some(self) }
    }

    #[inline]
    pub fn get(&self, target: &str) -> *mut u8 {
        let cmp = if is_exact(target) {
            |name: &str, target: &str| name == target
        } else {
            |name: &str, target: &str| {
                let name = unsafe { name.get_unchecked(0..name.len().saturating_sub(3)) };

                name == target
            }
        };

        for interface in self.iter() {
            interface.new();
        }

        for interface in self.iter() {
            let name = interface.name();

            if cmp(name, target) {
                return interface.new();
            }
        }

        ptr::null_mut()
    }
}

impl fmt::Debug for Interface {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Interface")
            .field("name", &self.name())
            .finish_non_exhaustive()
    }
}

pub struct InterfaceIter<'a> {
    iter: Option<&'a Interface>,
}

impl<'a> Iterator for InterfaceIter<'a> {
    type Item = &'a Interface;

    #[inline]
    fn next(&mut self) -> Option<&'a Interface> {
        let next = unsafe { self.iter?.next.as_ref() };

        mem::replace(&mut self.iter, next)
    }
}
