#[doc(hidden)]
#[macro_export]
macro_rules! impl_cmp {
    ($ident:ident) => {
        impl PartialEq for $ident {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.to_simd() == other.to_simd()
            }

            #[inline]
            fn ne(&self, other: &Self) -> bool {
                self.to_simd() != other.to_simd()
            }
        }

        impl PartialOrd for $ident {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                self.to_simd().partial_cmp(&other.to_simd())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_methods {
    () => {
        #[inline]
        pub fn is_finite(self) -> bool {
            self.to_simd().is_finite().all()
        }

        #[inline]
        pub fn distance(self, other: Self) -> f32 {
            self.distance_squared(other).sqrt()
        }

        #[inline]
        pub fn distance_squared(self, other: Self) -> f32 {
            (self - other).magnitude_squared()
        }

        #[inline]
        pub fn dot(self, other: Self) -> f32 {
            (self * other).sum()
        }

        #[inline]
        pub fn magnitude(self) -> f32 {
            self.magnitude_squared().sqrt()
        }

        #[inline]
        pub fn magnitude_squared(self) -> f32 {
            self.dot(self)
        }

        #[inline]
        pub fn sum(self) -> f32 {
            self.to_simd().reduce_sum()
        }

        #[inline]
        pub fn to_degrees(self) -> Self {
            Self::from_simd(self.to_simd().to_degrees())
        }

        #[inline]
        pub fn to_radians(self) -> Self {
            Self::from_simd(self.to_simd().to_radians())
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_ops {
    ($ident:ident; $(($trait:ident, $trait_assign:ident, $method:ident, $method_assign:ident, $op:tt),)*) => {$(
        impl ::core::ops::$trait for $ident {
            type Output = $ident;

            #[inline]
            fn $method(self, other: Self) -> Self {
                Self::from_simd(self.to_simd() $op other.to_simd())
            }
        }

        impl ::core::ops::$trait_assign for $ident {
            #[inline]
            fn $method_assign(&mut self, other: Self) {
                *self = *self $op other;
            }
        }
    )*}
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_all_ops {
    ($ident:ident) => {
        $crate::impl_cmp! {
            $ident
        }

        $crate::impl_ops! {
            $ident;
            (Add, AddAssign, add, add_assign, +),
            (Div, DivAssign, div, div_assign, /),
            (Mul, MulAssign, mul, mul_assign, *),
            (Rem, RemAssign, rem, rem_assign, %),
            (Sub, SubAssign, sub, sub_assign, -),
        }
    };
}
