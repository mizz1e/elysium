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

    pub(crate) fn find<'a>(&self, bytes: &'a [u8]) -> Option<&'a [u8]> {
        let regex = self
            .cell
            .get_or_try_init(|| Regex::new(self.pattern))
            .ok()?;

        regex
            .find(bytes)
            .map(|found| unsafe { bytes.get_unchecked(found.start()..) })
    }
}
