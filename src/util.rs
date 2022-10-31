use crate::Error;
use std::ffi::OsStr;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::{env, fs, thread};

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

        let has_csgo = apps.position(|app| app == 730).is_some();

        has_csgo.then(|| Path::new(path))
    });

    let path = path.next()?;

    Some(path.join(CSGO_DIR))
}

// Automatically append "LD_LIBRARY_PATH" otherwise CSGO can't find any libraries!
pub fn check_linker_path<P>(csgo_dir: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    if env::var_os("SANE_LINKER_PATH").is_some() {
        return Ok(());
    }

    let csgo_dir = csgo_dir.as_ref();
    let current_exe = env::current_exe().map_err(|_| Error::NoCsgo)?;
    let mut linker_path = var_path("LD_LIBRARY_PATH");

    let steam_dir = csgo_dir.ancestors().nth(3).ok_or(Error::NoCsgo)?;

    /*/ely/data/Steam/ubuntu12_32
    /ely/data/Steam/ubuntu12_32/panorama
    /ely/data/Steam/ubuntu12_32/steam-runtime/pinned_libs_32
    /ely/data/Steam/ubuntu12_32/steam-runtime/pinned_libs_64
    /usr/lib/gcc/x86_64-pc-linux-gnu/11.3.0
    /usr/lib/gcc/x86_64-pc-linux-gnu/11.3.0/32
    /lib64
    /usr/lib64
    /usr/local/lib64
    /lib
    /usr/lib
    /usr/local/lib
    /usr/lib/llvm/14/lib
    /usr/lib/llvm/14/lib64
    /ely/data/Steam/ubuntu12_32/steam-runtime/lib/i386-linux-gnu
    /ely/data/Steam/ubuntu12_32/steam-runtime/usr/lib/i386-linux-gnu
    /ely/data/Steam/ubuntu12_32/steam-runtime/lib/x86_64-linux-gnu
    /ely/data/Steam/ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu
    /ely/data/Steam/ubuntu12_32/steam-runtime/lib
    /ely/data/Steam/ubuntu12_32/steam-runtime/usr/lib*/

    linker_path.insert(0, csgo_dir.join("bin/linux64"));

    linker_path.insert(0, csgo_dir.join("csgo/bin/linux64"));

    linker_path.insert(
        0,
        steam_dir.join("ubuntu12_32/steam-runtime/lib/x86_64-linux-gnu"),
    );

    linker_path.insert(
        0,
        steam_dir.join("ubuntu12_32/steam-runtime/usr/lib/x86_64-linux-gnu"),
    );

    linker_path.insert(
        0,
        steam_dir.join("ubuntu12_32/steam-runtime/pinned_libs_64"),
    );

    linker_path.insert(0, steam_dir.join("ubuntu12_32/panorama"));

    let linker_path = env::join_paths(linker_path).unwrap_or_default();
    let error = Command::new(current_exe)
        .args(env::args_os().skip(1))
        .current_dir(csgo_dir)
        .env("SANE_LINKER_PATH", "sane")
        .env("LD_LIBRARY_PATH", dbg!(linker_path))
        .exec();

    Err(Error::Io(error))
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

/// Fetches the environment variable `key` from the current process, parsing it as a `PATH`,
/// returning an empty `Vec` if the variable isn’t set or there’s another error.
pub fn var_path<K: AsRef<OsStr>>(key: K) -> Vec<PathBuf> {
    let path = match env::var_os(key) {
        Some(path) => path,
        None => return Vec::new(),
    };

    env::split_paths(&path).collect()
}
