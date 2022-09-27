use std::borrow::Cow;
use std::ffi::CStr;

const CONNECT: Cow<'static, CStr> = const_cstr("+connect\0");
const MAP: Cow<'static, CStr> = const_cstr("+map\0");
const NO_BREAKPAD: Cow<'static, CStr> = const_cstr("-nobreakpad\0");

pub enum Launcher {
    Connect(SockAddr),
    Map(PathBuf),
}

impl Launcher {
    #[inline]
    pub fn launch(self) -> ! {
        // not sure yet
    }
}
