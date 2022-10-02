use crate::ffi::with_cstr_os_str;
use cake::ffi::COsStr;
use crossbeam_utils::atomic::AtomicCell;
use std::ffi::OsStr;
use std::{ffi, fmt, ptr};

pub type FromBytes = unsafe extern "C" fn(
    name: *const ffi::c_char,
    value: *const ffi::c_char,
    end_of_parser: *const *const ffi::c_char,
) -> Option<&'static Vdf>;

static FROM_BYTES: AtomicCell<FromBytes> = AtomicCell::new(from_bytes);

/// Placeholder function until [`set_from_bytes`](Vdf::set_from_bytes) is called.
#[inline]
unsafe extern "C" fn from_bytes(
    _name: *const ffi::c_char,
    _value: *const ffi::c_char,
    _end_of_parser: *const *const ffi::c_char,
) -> Option<&'static Vdf> {
    unimplemented!();
}

#[allow(dead_code)]
#[derive(Debug)]
#[non_exhaustive]
#[repr(u8)]
enum Kind {
    None = 0,
    String = 1,
    Int = 2,
    Float = 3,
    Pointer = 4,
    WideString = 5,
    Color = 6,
    Uint64 = 7,
    CompiledIntByte = 8,
    CompiledInt0 = 9,
    CompiledInt1 = 10,
    NumTypes = 11,
}

#[repr(C)]
union Value {
    pub int: ffi::c_char,
    pub float: ffi::c_float,
    pub data: *const ffi::c_void,
    pub color: [ffi::c_uchar; 4],
}

#[repr(C)]
#[non_exhaustive]
pub struct Vdf {
    name: u32,
    string_value: *const ffi::c_char,
    wide_string_value: *const ffi::c_short,
    value: Value,
    kind: Kind,
    has_escape_sequences: ffi::c_char,
    case_sensitive_name: u16,
    vdf_system: *const (),
    owns_vdf_system: bool,
    peer: Option<&'static Vdf>,
    sub: *const Vdf,   //Option<&'static Vdf>,
    chain: *const Vdf, //Option<&'static Vdf>,
}

impl Vdf {
    #[inline]
    pub fn from_bytes<S, T>(base: S, vdf: Option<T>) -> Option<&'static Vdf>
    where
        S: AsRef<OsStr>,
        T: AsRef<OsStr>,
    {
        let vdf = with_cstr_os_str(base, |base| unsafe {
            let base = base.as_ptr();

            match vdf {
                Some(vdf) => Vdf::from_base_vdf_unchecked(base, vdf),
                None => Vdf::from_base_unchecked(base),
            }
        });

        println!("{vdf:?}");

        vdf
    }

    #[inline]
    unsafe fn from_raw_parts_unchecked(
        base: *const libc::c_char,
        vdf: *const libc::c_char,
    ) -> Option<&'static Vdf> {
        let from_bytes = FROM_BYTES.load();

        unsafe { (from_bytes)(base, vdf, ptr::null()) }
    }

    #[inline]
    unsafe fn from_base_unchecked(base: *const libc::c_char) -> Option<&'static Vdf> {
        Vdf::from_raw_parts_unchecked(base, ptr::null())
    }

    #[inline]
    unsafe fn from_base_vdf_unchecked<S>(base: *const libc::c_char, vdf: S) -> Option<&'static Vdf>
    where
        S: AsRef<OsStr>,
    {
        with_cstr_os_str(vdf, |vdf| Vdf::from_raw_parts_unchecked(base, vdf.as_ptr()))
    }

    #[inline]
    pub fn set_from_bytes(function: FromBytes) {
        FROM_BYTES.store(function);
    }

    fn string_value(&self) -> Option<&'static OsStr> {
        let string_value = unsafe { self.string_value.as_ref()? };
        let string_value = unsafe { COsStr::from_ptr(string_value).as_os_str() };

        Some(string_value)
    }
}

impl fmt::Debug for Vdf {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Vdf")
            .field("name", &self.name)
            .field("string_value", &self.string_value())
            .field("wide_string_value", &self.wide_string_value)
            .field("value", &"<union>")
            .field("kind", &self.kind)
            .field("has_escape_sequences", &self.has_escape_sequences)
            //.field("case_sensitive_name", &self.case_sensitive_name)
            //.field("vdf_system", &self.vdf_system)
            //.field("owns_vdf_system", &self.owns_vdf_system)
            .field("peer", &self.peer)
            .field("sub", &self.sub)
            .field("chain", &self.chain)
            .finish_non_exhaustive()
    }
}
