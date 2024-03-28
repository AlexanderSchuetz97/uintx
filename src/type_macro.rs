
#[macro_export]
macro_rules! type_conversion {
    ($from:ty, $intermediary:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                let hlp : $intermediary = value.as_num() as $intermediary;
                return Self::from_num(hlp);
            }
        }
    }
}

#[macro_export]
macro_rules! from_to_impl {
    ($source:ty, $helper:ty, $target:ty) => {
        impl From<$target> for $source {
            fn from(value: $target) -> Self {
                return Self::from_num(value as $helper);
            }
        }

        impl Into<$target> for $source {
            fn into(self) -> $target {
                return self.as_num() as $target;
            }
        }
    };
}

#[macro_export]
macro_rules! sh_impl {
    ($lhs:ty, $rhs:ty) => {
        impl Shl<$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs);
            }
        }

        impl Shl<&$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs);
            }
        }

        impl ShlAssign<$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() << rhs);
            }
        }

        impl ShlAssign<&$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() << rhs);
            }
        }

        impl Shr<$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs);
            }
        }

        impl Shr<&$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs);
            }
        }

        impl ShrAssign<$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() >> rhs);
            }
        }

        impl ShrAssign<&$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() >> rhs);
            }
        }
    };
}

#[macro_export]
macro_rules! type_impl {
    ($source:ty, $helper:ty, $size:literal) => {
        use std::cmp::Ordering;
        use std::fmt::{Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, UpperExp, UpperHex};
        use std::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
            Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
        };

        use crate::sh_impl;
        use crate::from_to_impl;


        #[allow(dead_code)]
        impl $source {
            pub const MAX: $source = Self([0xFFu8; $size]);
            pub const MIN: $source = Self([0x00u8; $size]);
            pub const MAX_VALUE: $helper = Self([0xFFu8; $size]).as_num();

            #[inline]
            pub const fn swap_bytes(self) -> Self {
                Self(Self::swap_data_copy(&self.0))
            }

            #[inline]
            pub fn inner_ref(&self) -> &[u8; $size] {
                &self.0
            }

            #[inline]
            pub fn inner_ref_mut(&mut self) -> &mut [u8; $size] {
                &mut self.0
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn from_le_bytes(data: [u8; $size]) -> Self {
                return Self(data);
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn from_le_bytes(data: [u8; $size]) -> Self {
                return Self(Self::swap_data_copy(&data));
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn from_be_bytes(data: [u8; $size]) -> Self {
                return Self(Self::swap_data_copy(&data));
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn from_be_bytes(data: [u8; $size]) -> Self {
                return Self(data);
            }

            #[inline]
            pub const fn from_ne_bytes(data: [u8; $size]) -> Self {
                return Self(data);
            }

            #[inline]
            pub const fn to_ne_bytes(self) -> [u8; $size] {
                return self.0;
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; $size] {
                return self.0;
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn to_le_bytes(self) -> [u8; $size] {
                return Self::swap_data_copy(&self.0);
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; $size] {
                return Self::swap_data_copy(&self.0);
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn to_be_bytes(self) -> [u8; $size] {
                return self.0;
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn to_le(self) -> Self {
                self
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn to_le(self) -> Self {
                return self.swap_bytes();
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub const fn to_be(self) -> Self {
                return self.swap_bytes();
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub const fn to_be(self) -> Self {
                self
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub unsafe fn fetch_unsafe(&self) -> $helper {
                return self.0.as_ptr().cast::<$helper>().read_unaligned();
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub unsafe fn fetch_unsafe(&self) -> $helper {
                //Not optimized for big endian
                return self.as_num();
            }

            #[cfg(target_endian = "little")]
            #[inline]
            pub unsafe fn fetch_unsafe_clamped(&self) -> $helper {
                return self.0.as_ptr().cast::<$helper>().read_unaligned() & Self::MAX_VALUE;
            }

            #[cfg(target_endian = "big")]
            #[inline]
            pub unsafe fn fetch_unsafe_clamped(&self) -> $helper {
                //Not optimized for big endian
                return self.as_num();
            }

            #[inline]
            pub unsafe fn unsafe_add_into_aligned(&self, rhs: &Self) -> $helper {
                return self.fetch_unsafe_clamped() + rhs.fetch_unsafe_clamped();
            }

            #[inline]
            pub unsafe fn unsafe_add_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() + rhs.fetch_unsafe_clamped());
            }

            #[inline]
            pub unsafe fn unsafe_add_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() + rhs);
            }

            #[inline]
            pub unsafe fn unsafe_add_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() + rhs;
            }

            #[inline]
            pub unsafe fn unsafe_sub_into_aligned(&self, rhs: &Self) -> $helper {
                return self.fetch_unsafe_clamped() - rhs.fetch_unsafe_clamped();
            }

            #[inline]
            pub unsafe fn unsafe_sub_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() - rhs;
            }

            #[inline]
            pub unsafe fn unsafe_sub_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() - rhs.fetch_unsafe_clamped());
            }

            #[inline]
            pub unsafe fn unsafe_sub_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() - rhs);
            }

            #[inline]
            pub unsafe fn unsafe_mul_into_aligned(&self, rhs: &Self) -> $helper {
                return self.fetch_unsafe_clamped() * rhs.fetch_unsafe_clamped();
            }

            #[inline]
            pub unsafe fn unsafe_mul_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() * rhs;
            }

            #[inline]
            pub unsafe fn unsafe_mul_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() * rhs.fetch_unsafe_clamped());
            }

            #[inline]
            pub unsafe fn unsafe_mul_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() * rhs);
            }

            #[inline]
            pub unsafe fn unsafe_div_into_aligned(&self, rhs: &Self) -> $helper {
                return self.fetch_unsafe_clamped() / rhs.fetch_unsafe_clamped();
            }

            #[inline]
            pub unsafe fn unsafe_div_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() / rhs;
            }

            #[inline]
            pub unsafe fn unsafe_div_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() / rhs.fetch_unsafe_clamped());
            }

            #[inline]
            pub unsafe fn unsafe_div_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() / rhs);
            }

            #[inline]
            pub unsafe fn unsafe_rem_into_aligned(&self, rhs: &Self) -> $helper {
                return self.fetch_unsafe_clamped() % rhs.fetch_unsafe_clamped();
            }

            #[inline]
            pub unsafe fn unsafe_rem_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() % rhs;
            }

            #[inline]
            pub unsafe fn unsafe_rem_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() % rhs.fetch_unsafe_clamped());
            }

            #[inline]
            pub unsafe fn unsafe_rem_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe_clamped() % rhs);
            }

            #[inline]
            pub unsafe fn unsafe_or_into_aligned(&self, rhs: &Self) -> $helper {
                return (self.fetch_unsafe() | rhs.fetch_unsafe()) & Self::MAX_VALUE;
            }

            #[inline]
            pub unsafe fn unsafe_or_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() | rhs;
            }

            #[inline]
            pub unsafe fn unsafe_or_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe() | rhs.fetch_unsafe());
            }

            #[inline]
            pub unsafe fn unsafe_or_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe() | rhs);
            }

            #[inline]
            pub unsafe fn unsafe_and_into_aligned(&self, rhs: &Self) -> $helper {
                return (self.fetch_unsafe() & rhs.fetch_unsafe()) & Self::MAX_VALUE;
            }

            #[inline]
            pub unsafe fn unsafe_and_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe() & rhs;
            }

            #[inline]
            pub unsafe fn unsafe_and_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe() & rhs.fetch_unsafe());
            }

            #[inline]
            pub unsafe fn unsafe_and_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe() & rhs);
            }

            #[inline]
            pub unsafe fn unsafe_xor_into_aligned(&self, rhs: &Self) -> $helper {
                return (self.fetch_unsafe() ^ rhs.fetch_unsafe()) & Self::MAX_VALUE;
            }

            #[inline]
            pub unsafe fn unsafe_xor_with_aligned_into_aligned(&self, rhs: $helper) -> $helper {
                return self.fetch_unsafe_clamped() ^ rhs;
            }

            #[inline]
            pub unsafe fn unsafe_xor_into_unaligned(&self, rhs: &Self) -> Self {
                return Self::from_num(self.fetch_unsafe() ^ rhs.fetch_unsafe());
            }

            #[inline]
            pub unsafe fn unsafe_xor_with_aligned_into_unaligned(&self, rhs: $helper) -> Self {
                return Self::from_num(self.fetch_unsafe() ^ rhs);
            }

        }

        impl From<[u8; $size]> for $source {
            fn from(value: [u8; $size]) -> Self {
                return Self(value);
            }
        }

        impl Into<[u8; $size]> for $source {
            fn into(self) -> [u8; $size] {
                return self.0;
            }
        }

        from_to_impl!($source, $helper, u8);
        from_to_impl!($source, $helper, u16);
        from_to_impl!($source, $helper, u32);
        from_to_impl!($source, $helper, u64);
        from_to_impl!($source, $helper, u128);
        from_to_impl!($source, $helper, i8);
        from_to_impl!($source, $helper, i16);
        from_to_impl!($source, $helper, i32);
        from_to_impl!($source, $helper, i64);
        from_to_impl!($source, $helper, i128);

        impl Default for $source {
            fn default() -> Self {
                Self([0; $size])
            }
        }

        impl Debug for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.as_num(), f)
            }
        }

        impl Display for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self.as_num(), f)
            }
        }

        impl Octal for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Octal::fmt(&self.as_num(), f)
            }
        }

        impl LowerHex for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                LowerHex::fmt(&self.as_num(), f)
            }
        }

        impl UpperHex for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                UpperHex::fmt(&self.as_num(), f)
            }
        }

        impl LowerExp for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                LowerExp::fmt(&self.as_num(), f)
            }
        }

        impl UpperExp for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                UpperExp::fmt(&self.as_num(), f)
            }
        }

        impl Binary for $source {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                Binary::fmt(&self.as_num(), f)
            }
        }

        impl Not for $source {
            type Output = $source;

            fn not(self) -> Self::Output {
                Self::from_num(self.as_num().not())
            }
        }

        impl PartialEq<Self> for $source {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $source {}

        impl PartialOrd<Self> for $source {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl PartialEq<$helper> for $source {
            fn eq(&self, other: &$helper) -> bool {
                self.as_num().eq(other)
            }
        }

        impl PartialOrd<$helper> for $source {
            fn partial_cmp(&self, other: &$helper) -> Option<Ordering> {
                self.as_num().partial_cmp(other)
            }
        }

        impl Ord for $source {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl Add for $source {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() + rhs.as_num())
            }
        }

        impl Add<&$source> for $source {
            type Output = Self;

            fn add(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() + rhs.as_num())
            }
        }

        impl Add<$helper> for $source {
            type Output = Self;

            fn add(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() + rhs)
            }
        }

        impl Add<&$helper> for $source {
            type Output = Self;

            fn add(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() + rhs)
            }
        }

        impl AddAssign for $source {
            fn add_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() + rhs.as_num());
            }
        }

        impl AddAssign<&$source> for $source {
            fn add_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() + rhs.as_num());
            }
        }

        impl AddAssign<$helper> for $source {
            fn add_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() + rhs);
            }
        }

        impl AddAssign<&$helper> for $source {
            fn add_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() + rhs);
            }
        }

        impl BitAnd for $source {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl BitAnd<&$source> for $source {
            type Output = Self;

            fn bitand(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl BitAnd<$helper> for $source {
            type Output = Self;

            fn bitand(self, rhs: $helper) -> Self::Output {
                Self::from_num(self.as_num() & rhs)
            }
        }

        impl BitAnd<&$helper> for $source {
            type Output = Self;

            fn bitand(self, rhs: &$helper) -> Self::Output {
                Self::from_num(self.as_num() & rhs)
            }
        }

        impl BitAndAssign for $source {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl BitAndAssign<&$source> for $source {
            fn bitand_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl BitAndAssign<$helper> for $source {
            fn bitand_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() & rhs)
            }
        }

        impl BitAndAssign<&$helper> for $source {
            fn bitand_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() & rhs)
            }
        }

        impl BitOr for $source {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl BitOr<&$source> for $source {
            type Output = Self;

            fn bitor(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl BitOr<$helper> for $source {
            type Output = Self;

            fn bitor(self, rhs: $helper) -> Self::Output {
                Self::from_num(self.as_num() | rhs)
            }
        }

        impl BitOr<&$helper> for $source {
            type Output = Self;

            fn bitor(self, rhs: &$helper) -> Self::Output {
                Self::from_num(self.as_num() | rhs)
            }
        }

        impl BitOrAssign for $source {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl BitOrAssign<&$source> for $source {
            fn bitor_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl BitOrAssign<$helper> for $source {
            fn bitor_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() | rhs)
            }
        }

        impl BitOrAssign<&$helper> for $source {
            fn bitor_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() | rhs)
            }
        }

        impl BitXor for $source {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl BitXor<&$source> for $source {
            type Output = Self;

            fn bitxor(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl BitXor<$helper> for $source {
            type Output = Self;

            fn bitxor(self, rhs: $helper) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs)
            }
        }

        impl BitXor<&$helper> for $source {
            type Output = Self;

            fn bitxor(self, rhs: &$helper) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs)
            }
        }

        impl BitXorAssign for $source {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl BitXorAssign<&$source> for $source {
            fn bitxor_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl BitXorAssign<$helper> for $source {
            fn bitxor_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() ^ rhs)
            }
        }

        impl BitXorAssign<&$helper> for $source {
            fn bitxor_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() ^ rhs)
            }
        }

        impl Sub for $source {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl Sub<&$source> for $source {
            type Output = Self;

            fn sub(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl Sub<$helper> for $source {
            type Output = Self;

            fn sub(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() - rhs)
            }
        }

        impl Sub<&$helper> for $source {
            type Output = Self;

            fn sub(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() - rhs)
            }
        }

        impl SubAssign for $source {
            fn sub_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() - rhs.as_num());
            }
        }

        impl SubAssign<&$source> for $source {
            fn sub_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() - rhs.as_num());
            }
        }

        impl SubAssign<$helper> for $source {
            fn sub_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() - rhs);
            }
        }

        impl SubAssign<&$helper> for $source {
            fn sub_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() - rhs);
            }
        }

        impl Mul for $source {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() * rhs.as_num())
            }
        }

        impl Mul<&$source> for $source {
            type Output = Self;

            fn mul(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() * rhs.as_num())
            }
        }

        impl Mul<$helper> for $source {
            type Output = Self;

            fn mul(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() * rhs)
            }
        }

        impl Mul<&$helper> for $source {
            type Output = Self;

            fn mul(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() * rhs)
            }
        }

        impl MulAssign for $source {
            fn mul_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() * rhs.as_num());
            }
        }

        impl MulAssign<&$source> for $source {
            fn mul_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() * rhs.as_num());
            }
        }

        impl MulAssign<$helper> for $source {
            fn mul_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() * rhs);
            }
        }

        impl MulAssign<&$helper> for $source {
            fn mul_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() * rhs);
            }
        }

        impl Div for $source {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl Div<&$source> for $source {
            type Output = Self;

            fn div(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl Div<$helper> for $source {
            type Output = Self;

            fn div(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() / rhs)
            }
        }

        impl Div<&$helper> for $source {
            type Output = Self;

            fn div(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() / rhs)
            }
        }

        impl DivAssign for $source {
            fn div_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() / rhs.as_num());
            }
        }

        impl DivAssign<&$source> for $source {
            fn div_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() / rhs.as_num());
            }
        }

        impl DivAssign<$helper> for $source {
            fn div_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() / rhs);
            }
        }

        impl DivAssign<&$helper> for $source {
            fn div_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() / rhs);
            }
        }

        impl Rem for $source {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl Rem<&$source> for $source {
            type Output = Self;

            fn rem(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl Rem<$helper> for $source {
            type Output = Self;

            fn rem(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() % rhs)
            }
        }

        impl Rem<&$helper> for $source {
            type Output = Self;

            fn rem(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() % rhs)
            }
        }

        impl RemAssign for $source {
            fn rem_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() % rhs.as_num());
            }
        }

        impl RemAssign<&$source> for $source {
            fn rem_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() % rhs.as_num());
            }
        }

        impl RemAssign<$helper> for $source {
            fn rem_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() % rhs);
            }
        }

        impl RemAssign<&$helper> for $source {
            fn rem_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() % rhs);
            }
        }

        impl Shr for $source {
            type Output = $source;

            fn shr(self, rhs: Self) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl Shr<&$source> for $source {
            type Output = $source;

            fn shr(self, rhs: &Self) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl ShrAssign for $source {
            fn shr_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl ShrAssign<&$source> for $source {
            fn shr_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl Shl for $source {
            type Output = $source;

            fn shl(self, rhs: Self) -> Self::Output {
                return Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl Shl<&$source> for $source {
            type Output = $source;

            fn shl(self, rhs: &Self) -> Self::Output {
                return Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl ShlAssign for $source {
            fn shl_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl ShlAssign<&$source> for $source {
            fn shl_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        sh_impl!($source, u8);
        sh_impl!($source, u16);
        sh_impl!($source, u32);
        sh_impl!($source, u64);
        sh_impl!($source, u128);
        sh_impl!($source, i8);
        sh_impl!($source, i16);
        sh_impl!($source, i32);
        sh_impl!($source, i64);
        sh_impl!($source, i128);
        sh_impl!($source, usize);
        sh_impl!($source, isize);
    };
}
