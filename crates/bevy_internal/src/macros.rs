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

/// Obtain shared read access to the global application.
#[macro_export]
macro_rules! app {
    () => {
        unsafe { $crate::app::get() }
    };
}

/// Obtain exclusive write access to the global application.
#[macro_export]
macro_rules! app_mut {
    () => {
        unsafe { $crate::app::get_mut() }
    };
}

/// Set the global application.
#[macro_export]
macro_rules! set_app {
    ($app:expr) => {
        unsafe { $crate::app::set($app) };
    };
}

/// Assert that the provided pointer is non-null.
#[macro_export]
macro_rules! assert_non_null {
    ($ptr:expr $(,)?) => {
        ::core::assert!(!$ptr.is_null())
    };
    ($ptr:expr, $(arg:tt)+) => {
        ::core::assert!(!$ptr.is_null(), $($arg)+)
    };
}

/// Assert that the provided instruction matches the mnemonic specified.
#[macro_export]
macro_rules! assert_mnemonic {
    ($instruction:expr, $mnemonic:ident $(,)?) => {
        ::core::assert_eq!($instruction.mnemonic(), $crate::iced_x86::Mnemonic::$mnemonic)
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
        const CSTR: &'static ::core::ffi::CStr = $crate::macros::cstr($string);

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

/// Create a mov jmp hook.
#[macro_export]
macro_rules! inline_mov_jmp {
    (
        $ptr:expr,
        unsafe extern "C" fn($($arg:ident: $argty:ty),*$(,)?) $(-> $output:ty)? {
            $($body:tt)*
        }
    ) => {
        let ptr = $ptr;

        unsafe {
            $crate::replace(
                $crate::Ptr::transmute::<*mut $crate::assembly::AbsoluteJump>(ptr),
                $crate::assembly::AbsoluteJump::new({
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn mov_jmp($($arg: $argty,)*) -> $($output)? {
                        $($body)*
                    }

                    // coerce to a function pointer
                    let mov_jmp: unsafe extern "C" fn($($argty,)*) -> $($output)? = mov_jmp;

                    mov_jmp as usize as u64
                }),
            );
        }
    };
}

/// Create a mov jmp hook for a variadic function.
#[macro_export]
macro_rules! inline_mov_jmp_variadic {
    (
        $ptr:expr,
        unsafe extern "C" fn($($arg:ident: $argty:ty),+; mut $variadic:ident: ...) $(-> $output:ty)? {
            $($body:tt)*
        }
    ) => {
        let ptr = $ptr;

        unsafe {
            $crate::replace(
                $crate::Ptr::transmute::<*mut $crate::assembly::AbsoluteJump>(ptr),
                $crate::assembly::AbsoluteJump::new({
                    #[allow(non_snake_case)]
                    unsafe extern "C" fn mov_jmp($($arg: $argty,)+ mut $variadic: ...) -> $($output)? {
                        $($body)*
                    }

                    // coerce to a function pointer
                    let mov_jmp: unsafe extern "C" fn($($argty,)+ ...) -> $($output)? = mov_jmp;

                    mov_jmp as usize as u64
                }),
            );
        }
    };
}
