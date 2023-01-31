use crate::Error;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const HAS_CSGO_LINKER_PATH: &str = "HAS_CSGO_LINKER_PATH";
const HAS_STEAM_LINKER_PATH: &str = "HAS_STEAM_LINKER_PATH";

/// Determine the location of Steam, and read it's libraryfolders config.
pub fn steam_csgo_dirs() -> Option<(PathBuf, PathBuf)> {
    let home_dir = dirs::home_dir()?;
    let steam_dir = home_dir.join(".steam/root");

    if let Some(csgo_dir) = try_load_lib_dirs(&steam_dir) {
        return Some((steam_dir, csgo_dir));
    }

    let steam_dir = home_dir.join(".steam/steam");

    if let Some(csgo_dir) = try_load_lib_dirs(&steam_dir) {
        return Some((steam_dir, csgo_dir));
    }

    let steam_dir = home_dir.join(".var/app/com.valvesoftware.Steam");

    if let Some(csgo_dir) = try_load_lib_dirs(&steam_dir) {
        return Some((steam_dir, csgo_dir));
    }

    let data_dir = dirs::data_dir()?;
    let steam_dir = data_dir.join("esteem");

    if let Some(csgo_dir) = try_load_lib_dirs(&steam_dir) {
        return Some((steam_dir, csgo_dir));
    }

    let steam_dir = data_dir.join("Steam");

    if let Some(csgo_dir) = try_load_lib_dirs(&steam_dir) {
        return Some((steam_dir, csgo_dir));
    }

    None
}

fn try_load_lib_dirs(steam_dir: impl AsRef<Path>) -> Option<PathBuf> {
    const LIB_DIRS: &str = "steamapps/libraryfolders.vdf";
    const CSGO_DIR: &str = "steamapps/common/Counter-Strike Global Offensive";

    let steam_dir = steam_dir.as_ref();
    let lib_dirs_path = steam_dir.join(LIB_DIRS);
    let config = fs::read_to_string(lib_dirs_path).ok()?;
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

    if !path.exists() {
        return None;
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
pub fn check_linker_path() -> Result<(), Error> {
    const LD_LIBRARY_PATH: &str = "LD_LIBRARY_PATH";

    const BIN_LINUX64: &str = "bin/linux64";
    const CSGO_BIN_LINUX64: &str = "csgo/bin/linux64";

    const STEAM_RT_PINNED: &str = "ubuntu12_32/steam-runtime/pinned_libs_64";

    const STEAM_RT_LIB: &str = "ubuntu12_32/steam-runtime/lib/x86_64-linux-gnu";
    const STEAM_RT_USR_LIB: &str = "ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu";

    if env::var_os(HAS_CSGO_LINKER_PATH).is_some() {
        return Ok(());
    }

    let (steam_dir, csgo_dir) = steam_csgo_dirs().ok_or(Error::NoCsgo)?;
    let current_exe = env::current_exe().map_err(|_| Error::NoCsgo)?;
    let mut linker_path = var_path(LD_LIBRARY_PATH);

    tracing::info!("found csgo at {csgo_dir:?}");
    tracing::info!("appending csgo linker path");

    if !env::var_os(HAS_STEAM_LINKER_PATH).is_some() {
        tracing::info!("appending steamrt linker path");

        linker_path.insert(0, steam_dir.join(STEAM_RT_USR_LIB));
        linker_path.insert(0, steam_dir.join(STEAM_RT_LIB));
        linker_path.insert(0, steam_dir.join(STEAM_RT_PINNED));
    }

    linker_path.insert(0, csgo_dir.join(CSGO_BIN_LINUX64));
    linker_path.insert(0, csgo_dir.join(BIN_LINUX64));

    let linker_path = env::join_paths(linker_path).unwrap_or_default();

    tracing::info!("set environment variable {LD_LIBRARY_PATH:?} to {linker_path:?}");

    tracing::info!(
        "re-executing self (glibc does not respect chaning {LD_LIBRARY_PATH:?} during program execution)"
    );

    let error = Command::new(current_exe)
        .args(env::args_os().skip(1))
        .current_dir(csgo_dir)
        .env(HAS_CSGO_LINKER_PATH, "elysium")
        .env(HAS_STEAM_LINKER_PATH, "elysium")
        .env(LD_LIBRARY_PATH, linker_path)
        .exec();

    Err(Error::Io(error))
}

pub fn pre_launch() -> Result<(), Error> {
    // steam_appid.txt isn't needed when you set these.
    env::set_var("SteamAppId", "730");
    env::set_var("SteamGameId", "730");

    check_display()?;
    check_linker_path()?;

    Ok(())
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
