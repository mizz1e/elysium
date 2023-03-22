use regex::bytes::Regex;
use std::{panic, sync::OnceLock};

pub use std::ffi::CStr;

/// A lazily compiled pattern.
pub struct Pattern(OnceLock<Regex>);

impl Pattern {
    /// Create a new `Pattern`.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(OnceLock::new())
    }

    /// Compiles the provided `pattern`, returning a `&'static Regex`.
    #[inline]
    pub fn with(&'static self, pattern: &'static str) -> &'static Regex {
        self.0.get_or_init(move || {
            let pattern = format!("(?msx-u){pattern}");

            Regex::new(&pattern).unwrap_or_else(|error| {
                panic!("failed to compile pattern: {error}");
            })
        })
    }
}

/// Create a `&'static CStr` from a `&'static str`.
pub const fn cstr(string: &'static str) -> &'static CStr {
    let bytes = string.as_bytes();
    let (last, mut rest) = match bytes.split_last() {
        Some((last, rest)) => (*last, rest),
        None => panic!("input is empty"),
    };

    while let [current, new_rest @ ..] = rest {
        if *current == 0 {
            panic!("input contains interior nul-terminator");
        }

        rest = new_rest;
    }

    if last != 0 {
        panic!("input is not nul-terminated");
    }

    // SAFETY: just checked that it is valid.
    unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
}

/// Assert that the provided instruction matches the mnemonic specified.
#[macro_export]
macro_rules! assert_mnemonic {
    ($instruction:expr, $mnemonic:ident $(,)?) => {
        ::core::assert_eq!($instruction.mnemonic(), $crate::iced_x86::Mnemonic::$mnemonic $(,)?)
    };
    ($instruction:expr, $mnemonic:ident, $($arg:tt)+) => {
        ::core::assert_eq!($instruction.mnemonic(), $crate::iced_x86::Mnemonic::$mnemonic, $($arg)+)
    };
}

/// Create a [`CStr`](CStr).
///
/// # Compile-time panics
///
/// If the input string is empty, contains an interior nul-terminator, or is not nul-terminated.
#[macro_export]
macro_rules! cstr {
    ($string:literal) => {{
        static CSTR: &'static ::core::ffi::CStr = $crate::macros::cstr($string);

        CSTR
    }};
}

/// Create a byte-wise regex.
///
/// # Panics
///
/// If the provided pattern is an invalid regex.
#[macro_export]
macro_rules! pattern {
    ($pattern:literal) => {{
        static PATTERN: $crate::macros::Pattern = $crate::macros::Pattern::new();

        PATTERN.with($pattern)
    }};
}
