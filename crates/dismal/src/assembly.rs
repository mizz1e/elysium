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
    pub displacement: usize,
    pub target: usize,
}

pub fn disassemble(bytes: &[u8]) -> Option<FlowInfo> {
    const BITNESS: u32 = 64;

    let base_ip = bytes.as_ptr().addr() as usize;
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
        lines.push((pretty_ip, (pretty_bytes, len), pretty_instruction));

        match instruction.flow_control() {
            FlowControl::Call
            | FlowControl::IndirectCall
            | FlowControl::IndirectBranch
            | FlowControl::UnconditionalBranch => {
                let displacement = instruction.ip_rel_memory_address() as usize;
                let target = ip + displacement;

                result = Some(FlowInfo {
                    displacement,
                    target,
                });

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
