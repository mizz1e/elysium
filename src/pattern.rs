use once_cell::sync::OnceCell;
use regex::bytes::Regex;

pub struct Pattern {
    cell: OnceCell<Regex>,
    pattern: &'static str,
}

impl Pattern {
    #[inline]
    #[must_use]
    pub const fn new(pattern: &'static str) -> Self {
        let cell = OnceCell::new();

        Self { cell, pattern }
    }

    #[inline]
    pub fn find<'a>(&self, bytes: &'a [u8]) -> Option<(usize, &'a [u8])> {
        let regex = self
            .cell
            .get_or_try_init(|| Regex::new(self.pattern))
            .ok()?;

        regex.find(bytes).map(|found| {
            let position = found.start();
            let bytes = &bytes[position..];

            (position, bytes)
        })
    }
}

pub const VDF_FROM_BYTES: Pattern = Pattern::new(r#"(?msx-u)\xE8....\x48\x89\xDF\x48\x89\x45\xE0"#);
