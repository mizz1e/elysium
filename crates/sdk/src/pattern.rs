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

pub const CSPLAYER_SPAWN: Pattern = Pattern::new(
    r#"(?msx-u)\x55\x48\x89\xE5\x53\x48\x89\xFB\x48\x83\xEC\x28\x48\x8B\x05....\x48\x8B\x00"#,
);

pub const KEY_VALUES_FROM_STRING: Pattern =
    Pattern::new(r#"(?msx-u)\xE8....\x48\x89\xDF\x48\x89\x45\xE0"#);
