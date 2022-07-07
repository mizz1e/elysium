#![deny(warnings)]
#![feature(once_cell)]
#![feature(const_mut_refs)]
#![feature(const_ptr_write)]

use regex::bytes::Regex;
use std::cell::OnceCell;
use std::marker::PhantomPinned;
use std::{fmt, str};

mod panic;
mod parse;
mod util;
mod validate;

pub struct Pattern<const N: usize> {
    source: &'static str,
    pattern: [u8; N],
    regex: OnceCell<Regex>,
    _pin: PhantomPinned,
}

impl<const N: usize> Pattern<N> {
    #[inline]
    pub const fn new(pattern: &'static str) -> Pattern<N> {
        let source = pattern;
        let pattern = parse::parse_pattern(source);
        let regex = OnceCell::new();
        let _pin = PhantomPinned;

        Self {
            source,
            pattern,
            regex,
            _pin,
        }
    }

    #[inline]
    pub(crate) fn pattern(&self) -> &'static str {
        let pattern = unsafe { str::from_utf8_unchecked(self.pattern.as_slice()) };
        let pattern: &'static str = unsafe { util::change_lifetime(pattern) };

        pattern
    }

    #[inline]
    pub fn regex(&self) -> &Regex {
        self.regex.get_or_init(|| util::new_regex(self.pattern()))
    }
}

impl<const N: usize> fmt::Debug for Pattern<N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.source)
    }
}
