use once_cell::sync::OnceCell;
use regex::bytes::Regex;

pub struct Pattern {
    cell: OnceCell<Regex>,
    pattern: &'static str,
}

impl Pattern {
    #[must_use]
    pub const fn new(pattern: &'static str) -> Self {
        let cell = OnceCell::new();

        Self { cell, pattern }
    }

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

pub static CSPLAYER_SPAWN: Pattern = Pattern::new(
    r#"(?msx-u)\x55\x48\x89\xE5\x53\x48\x89\xFB\x48\x83\xEC\x28\x48\x8B\x05....\x48\x8B\x00"#,
);

pub static HOST_RUNFRAME_INPUT: Pattern = Pattern::new(
    r#"(?msx-u)\x55\x48\x89\xE5\x41\x57\x66\x41\x0F\x7E\xC7\x41\x56\x41\x55\x41\x89\xFD\x41\x54\x53"#,
);

pub static KEY_VALUES_FROM_STRING: Pattern =
    Pattern::new(r#"(?msx-u)\xE8....\x48\x89\xDF\x48\x89\x45\xE0"#);

pub static INSERT_INTO_TREE: Pattern = Pattern::new(r#"(?msx-u)\x74\x24\x4C\x8B\x10"#);
