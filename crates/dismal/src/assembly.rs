use crate::Maps;
use iced_x86::{Decoder, DecoderOptions, FlowControl, Instruction};
use std::io;

/// Attempt to disassemble the provided `bytes`.
pub fn disassemble(bytes: &[u8]) -> io::Result<Vec<Instruction>> {
    let decoder = Decoder::try_with_ip(
        usize::BITS,
        bytes,
        bytes.as_ptr().addr() as u64,
        DecoderOptions::NONE,
    )
    .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

    let mut instructions = Vec::new();

    for instruction in decoder {
        instructions.push(instruction);

        if !matches!(instruction.flow_control(), FlowControl::Next) {
            break;
        }
    }

    Ok(instructions)
}

/// Attempt to disassemble from the provided `ptr`.
pub(crate) fn disassemble_ptr(ptr: *const u8) -> io::Result<Vec<Instruction>> {
    let ptr = ptr.cast_mut();
    let maps = Maps::current()?
        .into_iter()
        .skip_while(|map| !map.range().contains(&ptr))
        .collect();

    let mut maps = Maps { maps };
    let first_map = maps.maps.first_mut().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "unable to find a memory map containing the provided function pointer",
        )
    })?;

    // adjust the first address to the function pointer.
    first_map.range.start = ptr;

    // SAFETY: in order to be here, one map has to exist.
    unsafe {
        maps.ranges()
            .next()
            .map(|bytes| disassemble(bytes))
            .unwrap_unchecked()
    }
}
