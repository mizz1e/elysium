use crate::{assembly, Maps, Permissions};
use iced_x86::Instruction;
use std::{io, marker::Tuple, mem, ptr::Thin};

/// A pointer.
pub trait Ptr: Copy + Sized + Thin {
    fn is_readable(&self) -> bool {
        Maps::assert_permissions(self.transmute(), Permissions::READ)
    }

    fn is_writable(&self) -> bool {
        Maps::assert_permissions(self.transmute(), Permissions::WRITE)
    }

    fn is_executable(&self) -> bool {
        Maps::assert_permissions(self.transmute(), Permissions::EXECUTE)
    }

    fn transmute<U: Ptr>(self) -> U {
        // SAFETY: Conversion between pointer types is safe.
        unsafe { mem::transmute_copy(&self) }
    }
}

impl<T> Ptr for *const T {}
impl<T> Ptr for *mut T {}

/// A function pointer.
pub trait FnPtr<Args>: Ptr
where
    Args: Tuple,
{
    type Output;

    /// Attempt to disassemble the function.
    #[inline]
    fn disassemble(&self) -> io::Result<Vec<Instruction>> {
        assembly::disassemble_ptr(self.transmute())
    }

    /// Convenience method to obtain the last instruction of the function.
    ///
    /// Useful for replacing target addresses.
    #[inline]
    fn last_instruction(&self) -> io::Result<Instruction> {
        // SAFETY: A function without instructions is impossible.
        unsafe {
            self.disassemble()?
                .into_iter()
                .last()
                .map(Ok)
                .unwrap_unchecked()
        }
    }

    /// Call this function pointer.
    ///
    /// # Safety
    ///
    /// In the case of foreign function pointers, this is unsafe.
    unsafe fn call(self, args: Args) -> Self::Output;
}

macro_rules! impl_fn_ptr {
    ({ $($prefix:tt)* }; $($arg:ident)*) => {
        #[doc(hidden)]
        impl<$($arg,)* Output> Ptr for $($prefix)*($($arg,)*) -> Output {}

        #[doc(hidden)]
        impl<$($arg,)* Output> Ptr for Option<$($prefix)*($($arg,)*) -> Output> {}

        #[doc(hidden)]
        impl<$($arg,)* Output> FnPtr<($($arg,)*)> for $($prefix)*($($arg,)*) -> Output {
            type Output = Output;

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn call(self, ($($arg,)*): ($($arg,)*)) -> Self::Output {
                #[cfg(debug_assertions)]
                {
                    ::core::assert!(self.is_readable(), "function pointer is not readable");
                    ::core::assert!(self.is_executable(), "function pointer is not executable");
                    ::core::assert!(self.disassemble().is_ok(), "function pointer does not contain valid code");
                }

                (self)($($arg,)*)
            }
        }
    };
}

macro_rules! impl_fn_ptr_variadic {
    ({ $($prefix:tt)* }; $($arg:ident)*) => {
        #[doc(hidden)]
        impl<$($arg,)* Output> Ptr for $($prefix)*($($arg,)* ...) -> Output {}

        #[doc(hidden)]
        impl<$($arg,)* Output> Ptr for Option<$($prefix)*($($arg,)* ...) -> Output> {}
    };
}

macro_rules! impl_fn_ptr_all {
    ($($arg:ident)*) => {
        impl_fn_ptr!({ fn }; $($arg)*);
        impl_fn_ptr!({ unsafe fn }; $($arg)*);

        impl_fn_ptr!({ extern "C" fn }; $($arg)*);
        impl_fn_ptr!({ unsafe extern "C" fn }; $($arg)*);

        //impl_fn_ptr!({ extern "thiscall" fn }; $($arg)*);
        //impl_fn_ptr!({ unsafe extern "thiscall" fn }; $($arg)*);
    };
}

macro_rules! impl_fn_ptr_variadic_all {
    ($($arg:ident)*) => {
        impl_fn_ptr_variadic!({ extern "C" fn }; $($arg)*);
        impl_fn_ptr_variadic!({ unsafe extern "C" fn }; $($arg)*);
    };
}

macro_rules! impl_fn_ptr_recursive {
    () => {
        impl_fn_ptr_all!();
    };
    ($first:ident $($tail:ident)*) => {
        impl_fn_ptr_all!($first $($tail)*);
        impl_fn_ptr_variadic_all!($first $($tail)*);
        impl_fn_ptr_recursive!($($tail)*);
    };
}

impl_fn_ptr_recursive!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
