use crate::{const_cstr, Error, Options};
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStringExt;
use std::{ffi, iter, mem, ptr};

type Main = unsafe extern "C" fn(argc: ffi::c_int, argv: *const *const ffi::c_char);

const LAUNCHER_CLIENT: &str = "launcher_client.so";
const LAUNCHER_MAIN: &str = "LauncherMain";

const CONNECT: Cow<'static, CStr> = const_cstr("+connect\0");
const FPS: Cow<'static, CStr> = const_cstr("+fps_max\0");
const FULLSCREEN: Cow<'static, CStr> = const_cstr("-fullscreen\0");
const MAP: Cow<'static, CStr> = const_cstr("+map\0");
const NO_BREAKPAD: Cow<'static, CStr> = const_cstr("-nobreakpad\0");
const NO_VIDEO: Cow<'static, CStr> = const_cstr("-novid\0");
const NO_JOYSTICKS: Cow<'static, CStr> = const_cstr("-nojoy\0");
const STEAM: Cow<'static, CStr> = const_cstr("-steam\0");
const WINDOWED: Cow<'static, CStr> = const_cstr("-windowed\0");

#[inline]
pub fn launch(options: Options) -> Result<(), Error> {
    unsafe { launch_inner(options) }
}

#[inline]
unsafe fn launch_inner(options: Options) -> Result<(), Error> {
    let mut args = Vec::new();

    args.push(NO_BREAKPAD);
    args.push(NO_JOYSTICKS);

    if let Some(address) = options.address {
        let address = address.to_string();

        if let Ok(address) = CString::new(address) {
            args.push(CONNECT);
            args.push(Cow::Owned(address));
        }
    }

    let fps = options.fps.to_string();

    if let Ok(fps) = CString::new(fps) {
        args.push(FPS);
        args.push(Cow::Owned(fps));
    }

    if options.fullscreen {
        args.push(FULLSCREEN);
    }

    if let Some(path) = options.map {
        let path = path.into_os_string().into_vec();

        if let Ok(path) = CString::new(path) {
            args.push(MAP);
            args.push(Cow::Owned(path));
        }
    }

    if options.skip_launch_video {
        args.push(NO_VIDEO);
    }

    // NOTE: Omission of `-steam` implies `-insecure`.
    if !options.no_vac {
        args.push(STEAM);
    }

    if options.windowed {
        args.push(WINDOWED);
    }

    let args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .chain(iter::once(ptr::null()))
        .collect::<Vec<_>>();

    let launcher = link::load_module(LAUNCHER_CLIENT).map_err(Error::LoadLauncher)?;
    let address = launcher.symbol(LAUNCHER_MAIN).map_err(Error::FindMain)?;
    let main: Main = mem::transmute(address.symbol.address);

    println!("\x1b[38;5;2minfo:\x1b[m starting csgo");

    (main)(
        args.len().saturating_sub(1) as ffi::c_int,
        args.as_ptr().cast(),
    );

    Ok(())
}
