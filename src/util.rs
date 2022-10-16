use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, thread};

/// Determine the directory CSGO is installed to.
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
