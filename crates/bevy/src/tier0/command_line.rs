use {
    crate::{internal::assert_non_null, sys},
    bevy::prelude::*,
    std::{
        ffi::{self, CStr},
        ptr,
    },
};

// Magic "index" constants for `ICommandLine_FindParm`, `ICommandLine_GetParm`, `ICommandLine_SetParm`.
const BASE_DIR: ffi::c_int = ffi::c_int::from_ne_bytes(*b"BASE");
const GAME: ffi::c_int = ffi::c_int::from_ne_bytes(*b"GAME");
const VULKAN: ffi::c_int = ffi::c_int::from_ne_bytes(*b"VKVK");

// Path constants.
const CURRENT_EXE: &CStr = c"csgo_linux64";
const MOD_DIR: &CStr = c"csgo";

static COMMAND_LINE: CommandLine = CommandLine(sys::ICommandLine {
    vtable_: &sys::ICommandLine__bindgen_vtable {
        ICommandLine_CreateCmdLine,
        ICommandLine_CreateCmdLine1,
        ICommandLine_GetCmdLine,
        ICommandLine_CheckParm,
        ICommandLine_HasParm,
        ICommandLine_RemoveParm,
        ICommandLine_AppendParm,
        ICommandLine_ParmValue,
        ICommandLine_ParmValue1,
        ICommandLine_ParmValue2,
        ICommandLine_ParmCount,
        ICommandLine_FindParm,
        ICommandLine_GetParm,
        ICommandLine_SetParm,
        ICommandLine_GetParms,
    },
});

struct CommandLine(sys::ICommandLine);

unsafe impl Send for CommandLine {}
unsafe impl Sync for CommandLine {}

pub fn command_line() -> *mut sys::ICommandLine {
    ptr::from_ref(&COMMAND_LINE.0).cast_mut()
}

#[inline(never)]
unsafe fn map_arg(arg: &[u8]) -> Option<&'static CStr> {
    // TODO: use phf
    let result = match arg {
        b"+fps_max" => c"400",
        b"-basedir" => c"",
        b"-defaultgamedir" => c"hl2",
        b"-game" => MOD_DIR,
        b"-language" => c"english",
        b"-transmitevents" => c"",
        b"-window_name_suffix" => c"",
        _ => return None,
    };

    Some(result)
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_CreateCmdLine(
    this: *mut sys::ICommandLine,
    commandline: *const ffi::c_char,
) {
    assert_non_null!(this);
    assert_non_null!(commandline);
    debug!("ICommandLine_CreateCmdLine");
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_CreateCmdLine1(
    this: *mut sys::ICommandLine,
    _argc: ffi::c_int,
    argv: *mut *mut ffi::c_char,
) {
    assert_non_null!(this);
    assert_non_null!(argv);
    debug!("ICommandLine_CreateCmdLine1");
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_GetCmdLine(this: *const sys::ICommandLine) -> *const ffi::c_char {
    assert_non_null!(this);
    debug!("ICommandLine_GetCmdLine() -> {CURRENT_EXE:?}");

    CURRENT_EXE.as_ptr()
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_CheckParm(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
    _ppszValue: *mut *const ffi::c_char,
) -> *const ffi::c_char {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);
    let Some(result) = map_arg(arg.to_bytes()) else {
        return ptr::null();
    };

    debug!("ICommandLine_CheckParm({arg:?}) -> {result:?}");

    result.as_ptr()
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_HasParm(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
) -> bool {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);
    let result = map_arg(arg.to_bytes()).is_some();

    debug!("ICommandLine_HasParm({arg:?}) -> {result:?}");

    result
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_RemoveParm(
    this: *mut sys::ICommandLine,
    parm: *const ffi::c_char,
) {
    assert_non_null!(this);
    assert_non_null!(parm);

    let arg = CStr::from_ptr(parm);

    debug!("ICommandLine_RemoveParm({arg:?})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_AppendParm(
    this: *mut sys::ICommandLine,
    pszParm: *const ffi::c_char,
    _pszValues: *const ffi::c_char,
) {
    assert_non_null!(this);
    assert_non_null!(pszParm);

    let arg = CStr::from_ptr(pszParm);

    debug!("ICommandLine_AppendParm({arg:?})");
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_ParmValue(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
    pDefaultVal: *const ffi::c_char,
) -> *const ffi::c_char {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);
    let Some(result) = map_arg(arg.to_bytes()) else {
        return pDefaultVal;
    };

    debug!("ICommandLine_ParmValue({arg:?}) -> {result:?}");

    result.as_ptr()
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_ParmValue1(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
    nDefaultVal: ffi::c_int,
) -> ffi::c_int {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);

    debug!("ICommandLine_ParmValue1({arg:?}) -> {nDefaultVal:?}");

    nDefaultVal
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_ParmValue2(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
    flDefaultVal: f32,
) -> f32 {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);

    debug!("ICommandLine_ParmValue2({arg:?}) -> {flDefaultVal:?}");

    flDefaultVal
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_ParmCount(this: *const sys::ICommandLine) -> ffi::c_int {
    assert_non_null!(this);

    debug!("ICommandLine_ParmCount() -> 1");

    1
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_FindParm(
    this: *const sys::ICommandLine,
    psz: *const ffi::c_char,
) -> ffi::c_int {
    assert_non_null!(this);
    assert_non_null!(psz);

    let arg = CStr::from_ptr(psz);
    let result = match arg.to_bytes() {
        b"-basedir" => BASE_DIR,
        b"-game" => GAME,
        b"-vulkan" => VULKAN,
        _ => 0,
    };

    debug!("ICommandLine_FindParm({arg:?}) -> {result:?}");

    result
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_GetParm(
    this: *const sys::ICommandLine,
    nIndex: ffi::c_int,
) -> *const ffi::c_char {
    assert_non_null!(this);

    let result = match nIndex {
        // The zeroth index is always the program name.
        0 => CURRENT_EXE,
        BASE_DIR => c"",
        GAME => MOD_DIR,
        VULKAN => c"",
        _ => {
            debug!("ICommandLine_GetParm({nIndex:?}) -> (none)");

            return ptr::null();
        }
    };

    debug!("ICommandLine_GetParm({nIndex:?}) -> {result:?}");

    result.as_ptr()
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_SetParm(
    this: *mut sys::ICommandLine,
    _nIndex: ffi::c_int,
    _pNewParm: *const ffi::c_char,
) {
    assert_non_null!(this);

    debug!("ICommandLine_SetParm");
}

#[allow(non_snake_case)]
unsafe extern "C" fn ICommandLine_GetParms(
    this: *const sys::ICommandLine,
) -> *mut *const ffi::c_char {
    assert_non_null!(this);

    debug!("ICommandLine_GetParms");

    [CURRENT_EXE.as_ptr(), ptr::null()].as_ptr().cast_mut()
}
