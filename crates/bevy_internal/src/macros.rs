use {
    regex::bytes::Regex,
    std::{marker::FnPtr, panic, sync::OnceLock},
};

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

/// Obtain the address of a function pointer.
#[inline(always)]
pub fn fn_addr<F: FnPtr>(f: F) -> u64 {
    f.addr().addr() as u64
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

                    $crate::macros::fn_addr(mov_jmp)
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

                    $crate::macros::fn_addr(mov_jmp)
                }),
            );
        }
    };
}
