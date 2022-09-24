use core::cell::SyncUnsafeCell;
use core::{fmt, ptr};

#[repr(C)]
pub union VdfValue {
    pub int: i32,
    pub float: f32,
    pub data: *const u8,
    pub color: [u8; 4],
}

#[repr(C)]
pub struct Vdf {
    pub key_name: i32,
    pub value: *const u8,
    pub value_wide: *const u8,
    pub vdf_value: VdfValue,
    pub data_kind: u8,
    pub has_escape_sequences: bool,
    pub evaluate_conditionals: bool,
    pub _unused: bool,
    pub _unk1: u32,
    pub _unk2: u32,
    pub peer: *const Vdf,
    pub sub: *const Vdf,
    pub chain: *const Vdf,
}

#[inline]
unsafe extern "C" fn from_bytes(
    _name: *const u8,
    _value: *const u8,
    _unk1: *const u8,
) -> *const Vdf {
    panic!("Vdf::from_bytes called without loading the function from the game");
}

static FROM_BYTES: SyncUnsafeCell<
    unsafe extern "C" fn(name: *const u8, value: *const u8, _unk1: *const u8) -> *const Vdf,
> = SyncUnsafeCell::new(from_bytes);

impl Vdf {
    #[inline]
    pub fn from_bytes(name: &str, value: Option<&str>) -> Option<&'static Vdf> {
        let name = name.as_ptr();
        let value = value.map(|value| value.as_ptr()).unwrap_or(ptr::null());

        unsafe { (*FROM_BYTES.get())(name, value, ptr::null()).as_ref() }
    }

    #[inline]
    pub fn set_from_bytes(
        function: unsafe extern "C" fn(
            name: *const u8,
            value: *const u8,
            _unk1: *const u8,
        ) -> *const Vdf,
    ) {
        unsafe {
            (*FROM_BYTES.get()) = function;
        }
    }
}

impl fmt::Debug for Vdf {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Vdf")
            .field("key_name", &self.key_name)
            .field("value", &self.value)
            .field("value_wide", &self.value_wide)
            .field("data_kind", &self.data_kind)
            .field("has_escape_sequences", &self.has_escape_sequences)
            .field("peer", &self.peer)
            .field("sub", &self.sub)
            .field("chain", &self.chain)
            .finish()
    }
}
