use crate::Error;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs, thread};
use std::time::Duration;

const SANE_LINKER_PATH: &str = "SANE_LINKER_PATH";

/// Determine the directory CSGO is installed in.
pub fn determine_csgo_dir() -> Option<PathBuf> {
    const LIB_DIRS: &str = ".steam/steam/steamapps/libraryfolders.vdf";
    const CSGO_DIR: &str = "steamapps/common/Counter-Strike Global Offensive";

    let home_dir: PathBuf = env::var_os("HOME")?.into();
    let lib_dirs = home_dir.join(LIB_DIRS);
    let config = fs::read_to_string(lib_dirs).ok()?;
    let vdf = vdf::Pair::from_str(&config).ok()?;

    let mut path = vdf.iter().flat_map(|pair| {
        let path = pair.get_string("path")?;
        let apps = pair.get_list("apps")?;
        let mut apps = apps.iter().flat_map(|pair| {
            let key: u32 = pair.key().parse().ok()?;

            Some(key)
        });

        apps.position(|app| app == 730).map(|_| Path::new(path))
    });

    let path = path.next()?.join(CSGO_DIR);

    if env::var_os(SANE_LINKER_PATH).is_none() {
        log::info!("found csgo at {path:?}");
    }

    Some(path)
}

/// X11 `DISPLAY` sanity check as CSGO prefers to segmentation fault.
pub fn check_display() -> Result<(), Error> {
    env::var_os("DISPLAY").ok_or(Error::NoDisplay)?;

    Ok(())
}

/// Automatically append `LD_LIBRARY_PATH` otherwise CSGO can't find any libraries, and likes to
/// segmentation fault!!
pub fn check_linker_path(csgo_dir: impl AsRef<Path>) -> Result<(), Error> {
    const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";
    const BIN_LINUX64: &str = "bin/linux64";
    const CSGO_BIN_LINUX64: &str = "csgo/bin/linux64";
    const STEAM_RT_LIB: &str = "ubuntu12_32/steam-runtime/lib/x86_64-linux-gnu";
    const STEAM_RT_USR_LIB: &str = "ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu";
    const STEAM_RT_PINNED: &str = "ubuntu12_32/steam-runtime/pinned_libs_64";
    const STEAM_RT_PANORAMA: &str = "ubuntu12_32/panorama";

    if env::var_os(SANE_LINKER_PATH).is_some() {
        return Ok(());
    }

    let csgo_dir = csgo_dir.as_ref();
    let current_exe = env::current_exe().map_err(|_| Error::NoCsgo)?;
    let steam_dir = csgo_dir.ancestors().nth(3).ok_or(Error::NoCsgo)?;
    let mut linker_path = var_path(LD_LIBRARY_PATH);

    linker_path.insert(0, csgo_dir.join(BIN_LINUX64));
    linker_path.insert(0, csgo_dir.join(CSGO_BIN_LINUX64));
    linker_path.insert(0, steam_dir.join(STEAM_RT_LIB));
    linker_path.insert(0, steam_dir.join(STEAM_RT_USR_LIB));
    linker_path.insert(0, steam_dir.join(STEAM_RT_PINNED));
    linker_path.insert(0, steam_dir.join(STEAM_RT_PANORAMA));

    let linker_path = env::join_paths(linker_path).unwrap_or_default();

    log::info!("set environment variable {LD_LIBRARY_PATH:?} to {linker_path:?}");

    log::info!(
        "re-executing self (glibc does not respect chaning {LD_LIBRARY_PATH:?} during program execution)"
    );

    let error = Command::new(current_exe)
        .args(env::args_os().skip(1))
        .current_dir(csgo_dir)
        .env(SANE_LINKER_PATH, "sane")
        .env(LD_LIBRARY_PATH, linker_path)
        .exec();

    Err(Error::Io(error))
}

/// Fetches the environment variable `key` from the current process, parsing it as a `PATH`,
/// returning an empty `Vec` if the variable isn’t set or there’s another error.
pub fn var_path<K: AsRef<OsStr>>(key: K) -> Vec<PathBuf> {
    let path = match env::var_os(key) {
        Some(path) => path,
        None => return Vec::new(),
    };

    env::split_paths(&path).collect()
}

/// Determine whether SDL has been loaded.
pub fn is_sdl_loaded() -> bool {
    let sdl = link::is_module_loaded("libSDL2-2.0.so.0");

    // SDL may not be initialized yet, wait for VGUI to initialize it.
    let vgui = link::is_module_loaded("vgui2_client.so");

    sdl && vgui
}

/// Determine whether the material system has been loaded.
pub fn is_materials_loaded() -> bool {
    let materials = link::is_module_loaded("materialsystem_client.so");

    // Client contains `Vdf::from_bytes`, which is needed to create materials.
    let client = link::is_module_loaded("client_client.so");

    materials && client
}

/// Determine whether the server browser has been loaded.
///
/// This is the last module to be loaded.
pub fn is_browser_loaded() -> bool {
    let browser = link::is_module_loaded("serverbrowser_client.so");

    browser
}

/// Block until a condition to becomes true.
pub fn sleep_until<F>(f: F)
where
    F: Fn() -> bool,
{
    while !f() {
        thread::sleep(Duration::from_millis(100));
    }
}
