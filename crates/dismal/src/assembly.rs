use iced_x86::{
    Decoder, DecoderOptions, FlowControl, Formatter, FormatterOutput, FormatterTextKind,
    Instruction, IntelFormatter,
};

use std::fmt::Write;
use yansi::Paint;

#[derive(Default)]
struct Output(String);

impl FormatterOutput for Output {
    fn write(&mut self, text: &str, kind: FormatterTextKind) {
        let paint = match kind {
            FormatterTextKind::Directive | FormatterTextKind::Keyword => Paint::magenta(text),
            FormatterTextKind::Prefix | FormatterTextKind::Mnemonic => Paint::red(text),
            FormatterTextKind::FunctionAddress | FormatterTextKind::Number => Paint::blue(text),
            FormatterTextKind::Register => Paint::blue(text),
            _ => Paint::default(text),
        };

        let _ = write!(&mut self.0, "{paint}");
    }
}

#[derive(Debug)]
pub struct FlowInfo {
    pub rel_addr: usize,
    pub target: usize,
}

pub fn disassemble(bytes: &[u8]) -> Option<FlowInfo> {
    const BITNESS: u32 = 64;

    let base_ip = bytes.as_ptr().addr();
    let mut decoder = Decoder::with_ip(BITNESS, bytes, base_ip as u64, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::with_options(None, None);
    let mut instruction = Instruction::default();
    let mut output = Output::default();

    let options = formatter.options_mut();

    options.set_binary_prefix("0b");
    options.set_binary_suffix("");
    options.set_branch_leading_zeros(false);
    options.set_digit_separator("_");
    options.set_displacement_leading_zeros(false);
    options.set_decimal_digit_group_size(3);
    options.set_hex_digit_group_size(0);
    options.set_hex_prefix("0x");
    options.set_hex_suffix("");
    options.set_octal_prefix("0o");
    options.set_octal_suffix("");
    options.set_rip_relative_addresses(true);
    options.set_signed_immediate_operands(true);
    options.set_signed_memory_displacements(true);
    options.set_small_hex_numbers_in_decimal(true);
    options.set_uppercase_hex(false);

    let mut lines = Vec::new();
    let mut max_len = 0;
    let mut result = None;
    let mut rel_addr = 0;

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        output.0.clear();
        formatter.format(&instruction, &mut output);

        let ip = instruction.ip() as usize;
        let len = instruction.len();
        let offset = ip - base_ip;
        let bytes = unsafe { bytes.get_unchecked(offset..(offset + len)) };

        let pretty_ip = Paint::green(format!("{ip:016x}")).to_string();
        let mut pretty_bytes = String::new();

        for byte in bytes {
            pretty_bytes += &Paint::blue(format!("{byte:02x} ")).to_string();
        }

        let pretty_instruction = output.0.to_string();

        max_len = max_len.max(len);

        let new_rel_addr = instruction.ip_rel_memory_address() as usize;

        if new_rel_addr != 0 {
            rel_addr = new_rel_addr;
        }

        lines.push((pretty_ip, (pretty_bytes, len), pretty_instruction));

        match instruction.flow_control() {
            FlowControl::Call
            | FlowControl::IndirectCall
            | FlowControl::IndirectBranch
            | FlowControl::UnconditionalBranch => {
                let target = ip + rel_addr;

                result = Some(FlowInfo { rel_addr, target });

                break;
            }
            FlowControl::Exception | FlowControl::Return => break,
            _ => {}
        }
    }

    let split_at = lines.len().saturating_sub(result.is_some() as usize);
    let (start, end) = unsafe { lines.split_at_unchecked(split_at) };

    for (ip, (bytes, len), instruction) in start {
        let padding = "   ".repeat(max_len - len);

        tracing::trace!("{ip} {bytes}{padding}{instruction}");
    }

    if let Some(info) = &result {
        for (ip, (bytes, len), instruction) in end {
            let padding = "   ".repeat(max_len - len);

            tracing::trace!(
                "{ip} {bytes}{padding}{instruction} -> {:#0x?}",
                Paint::magenta(info.target)
            );
        }
    }

    result
}

/*#[test]
mod test {
    #[test]
}

2022-12-11T11:18:23.024974Z TRACE elysium_mem: 7f8061e409a0 Mov(Rax, Int(355fc1)) [48, 8B, 05, C1, 5F, 35, 00] -> 0x7f8062196968
2022-12-11T11:18:23.024981Z TRACE elysium_sdk::ui: target = Some(0x7f8062196968)
2022-12-11T11:18:23.026260Z TRACE dismal::assembly: 00007f8061e409a0 48 8b 05 c1 5f 35 00 mov rax,[rip+0x355fc1]
2022-12-11T11:18:23.026266Z TRACE dismal::assembly: 00007f8061e409a7 ff e0                jmp rax -> 0x7f8061e409a7
2022-12-11T11:18:23.026271Z TRACE elysium_sdk::ui: target = FlowInfo { displacement: 0, target: 140189374876071 }*/
