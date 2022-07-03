use core::fmt;

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

impl fmt::Debug for Vdf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
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
