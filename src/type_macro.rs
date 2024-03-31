#[macro_export]
macro_rules! type_conversion {
    ($from:ty, $intermediary:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                let hlp: $intermediary = value.as_num() as $intermediary;
                return Self::from_num(hlp);
            }
        }

        impl From<&$from> for $to {
            fn from(value: &$from) -> Self {
                let hlp: $intermediary = value.as_num() as $intermediary;
                return Self::from_num(hlp);
            }
        }
    };
}

#[macro_export]
macro_rules! common_traits_impl {
    ($source:ty, $helper:ty, $size:literal) => {
        impl Default for $source {
            fn default() -> Self {
                Self([0; $size])
            }
        }

        impl std::fmt::Debug for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::Display for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::Octal for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Octal::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::LowerHex for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerHex::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::UpperHex for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperHex::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::LowerExp for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::LowerExp::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::UpperExp for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::UpperExp::fmt(&self.as_num(), f)
            }
        }

        impl std::fmt::Binary for $source {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Binary::fmt(&self.as_num(), f)
            }
        }

        impl std::ops::Not for $source {
            type Output = $source;

            fn not(self) -> Self::Output {
                Self::from_num(self.as_num().not())
            }
        }

        impl PartialEq<$source> for $source {
            fn eq(&self, other: &$source) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $source {}

        impl PartialOrd<$source> for $source {
            fn partial_cmp(&self, other: &$source) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl PartialEq<$helper> for $source {
            fn eq(&self, other: &$helper) -> bool {
                self.as_num().eq(other)
            }
        }

        impl PartialOrd<$helper> for $source {
            fn partial_cmp(&self, other: &$helper) -> Option<std::cmp::Ordering> {
                self.as_num().partial_cmp(other)
            }
        }
        impl Ord for $source {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }

        impl std::ops::Add for $source {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::from_num_checked(self.as_num() + rhs.as_num())
            }
        }

        impl std::ops::Add<$source> for &$source {
            type Output = $source;

            fn add(self, rhs: $source) -> $source {
                <$source>::from_num_checked(self.as_num() + rhs.as_num())
            }
        }

        impl std::ops::Add<&$source> for &$source {
            type Output = $source;
            fn add(self, rhs: &$source) -> $source {
                <$source>::from_num_checked(self.as_num() + rhs.as_num())
            }
        }

        impl std::ops::Add<$helper> for &$source {
            type Output = $source;

            fn add(self, rhs: $helper) -> $source {
                <$source>::from_num(self.as_num() + rhs)
            }
        }

        impl std::ops::Add<&$helper> for &$source {
            type Output = $source;

            fn add(self, rhs: &$helper) -> $source {
                <$source>::from_num_checked(self.as_num() + rhs)
            }
        }

        impl std::ops::Add<&$source> for $source {
            type Output = Self;

            fn add(self, rhs: &Self) -> Self {
                Self::from_num_checked(self.as_num() + rhs.as_num())
            }
        }

        impl std::ops::Add<$helper> for $source {
            type Output = Self;

            fn add(self, rhs: $helper) -> Self {
                Self::from_num_checked(self.as_num() + rhs)
            }
        }

        impl std::ops::Add<&$helper> for $source {
            type Output = Self;

            fn add(self, rhs: &$helper) -> Self {
                Self::from_num_checked(self.as_num() + rhs)
            }
        }

        impl std::ops::AddAssign for $source {
            fn add_assign(&mut self, rhs: Self) {
                *self = Self::from_num_checked(self.as_num() + rhs.as_num());
            }
        }

        impl std::ops::AddAssign<&$source> for $source {
            fn add_assign(&mut self, rhs: &Self) {
                *self = Self::from_num_checked(self.as_num() + rhs.as_num());
            }
        }

        impl std::ops::AddAssign<$helper> for $source {
            fn add_assign(&mut self, rhs: $helper) {
                *self = Self::from_num_checked(self.as_num() + rhs);
            }
        }

        impl std::ops::AddAssign<&$helper> for $source {
            fn add_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num_checked(self.as_num() + rhs);
            }
        }

        impl std::ops::BitAnd for $source {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAnd<$source> for &$source {
            type Output = $source;

            fn bitand(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAnd<&$source> for &$source {
            type Output = $source;
            fn bitand(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAnd<$helper> for &$source {
            type Output = $source;

            fn bitand(self, rhs: $helper) -> $source {
                <$source>::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitAnd<&$helper> for &$source {
            type Output = $source;

            fn bitand(self, rhs: &$helper) -> $source {
                <$source>::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitAnd<&$source> for $source {
            type Output = Self;

            fn bitand(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAnd<$helper> for $source {
            type Output = Self;

            fn bitand(self, rhs: $helper) -> Self::Output {
                Self::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitAnd<&$helper> for $source {
            type Output = Self;

            fn bitand(self, rhs: &$helper) -> Self::Output {
                Self::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitAndAssign for $source {
            fn bitand_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAndAssign<&$source> for $source {
            fn bitand_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() & rhs.as_num())
            }
        }

        impl std::ops::BitAndAssign<$helper> for $source {
            fn bitand_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitAndAssign<&$helper> for $source {
            fn bitand_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() & rhs)
            }
        }

        impl std::ops::BitOr for $source {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOr<$source> for &$source {
            type Output = $source;

            fn bitor(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOr<&$source> for &$source {
            type Output = $source;
            fn bitor(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOr<$helper> for &$source {
            type Output = $source;

            fn bitor(self, rhs: $helper) -> $source {
                <$source>::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitOr<&$helper> for &$source {
            type Output = $source;

            fn bitor(self, rhs: &$helper) -> $source {
                <$source>::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitOr<&$source> for $source {
            type Output = Self;

            fn bitor(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOr<$helper> for $source {
            type Output = Self;

            fn bitor(self, rhs: $helper) -> Self::Output {
                Self::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitOr<&$helper> for $source {
            type Output = Self;

            fn bitor(self, rhs: &$helper) -> Self::Output {
                Self::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitOrAssign for $source {
            fn bitor_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOrAssign<&$source> for $source {
            fn bitor_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() | rhs.as_num())
            }
        }

        impl std::ops::BitOrAssign<$helper> for $source {
            fn bitor_assign(&mut self, rhs: $helper) {
                *self = Self::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitOrAssign<&$helper> for $source {
            fn bitor_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num_checked(self.as_num() | rhs)
            }
        }

        impl std::ops::BitXor for $source {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXor<$source> for &$source {
            type Output = $source;

            fn bitxor(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXor<&$source> for &$source {
            type Output = $source;
            fn bitxor(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXor<$helper> for &$source {
            type Output = $source;

            fn bitxor(self, rhs: $helper) -> $source {
                <$source>::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::BitXor<&$helper> for &$source {
            type Output = $source;

            fn bitxor(self, rhs: &$helper) -> $source {
                <$source>::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::BitXor<&$source> for $source {
            type Output = Self;

            fn bitxor(self, rhs: &Self) -> Self::Output {
                Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXor<$helper> for $source {
            type Output = Self;

            fn bitxor(self, rhs: $helper) -> Self::Output {
                Self::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::BitXor<&$helper> for $source {
            type Output = Self;

            fn bitxor(self, rhs: &$helper) -> Self::Output {
                Self::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::BitXorAssign for $source {
            fn bitxor_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXorAssign<&$source> for $source {
            fn bitxor_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() ^ rhs.as_num())
            }
        }

        impl std::ops::BitXorAssign<$helper> for $source {
            fn bitxor_assign(&mut self, rhs: $helper) {
                *self = Self::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::BitXorAssign<&$helper> for $source {
            fn bitxor_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num_checked(self.as_num() ^ rhs)
            }
        }

        impl std::ops::Sub for $source {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl std::ops::Sub<$source> for &$source {
            type Output = $source;

            fn sub(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl std::ops::Sub<&$source> for &$source {
            type Output = $source;
            fn sub(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl std::ops::Sub<$helper> for &$source {
            type Output = $source;

            fn sub(self, rhs: $helper) -> $source {
                <$source>::from_num(self.as_num() - rhs)
            }
        }

        impl std::ops::Sub<&$helper> for &$source {
            type Output = $source;

            fn sub(self, rhs: &$helper) -> $source {
                <$source>::from_num(self.as_num() - rhs)
            }
        }

        impl std::ops::Sub<&$source> for $source {
            type Output = Self;

            fn sub(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() - rhs.as_num())
            }
        }

        impl std::ops::Sub<$helper> for $source {
            type Output = Self;

            fn sub(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() - rhs)
            }
        }

        impl std::ops::Sub<&$helper> for $source {
            type Output = Self;

            fn sub(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() - rhs)
            }
        }

        impl std::ops::SubAssign for $source {
            fn sub_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() - rhs.as_num());
            }
        }

        impl std::ops::SubAssign<&$source> for $source {
            fn sub_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() - rhs.as_num());
            }
        }

        impl std::ops::SubAssign<$helper> for $source {
            fn sub_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() - rhs);
            }
        }

        impl std::ops::SubAssign<&$helper> for $source {
            fn sub_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() - rhs);
            }
        }

        impl std::ops::Mul for $source {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self::from_num_checked(self.as_num() * rhs.as_num())
            }
        }

        impl std::ops::Mul<$source> for &$source {
            type Output = $source;

            fn mul(self, rhs: $source) -> $source {
                <$source>::from_num_checked(self.as_num() * rhs.as_num())
            }
        }

        impl std::ops::Mul<&$source> for &$source {
            type Output = $source;
            fn mul(self, rhs: &$source) -> $source {
                <$source>::from_num_checked(self.as_num() * rhs.as_num())
            }
        }

        impl std::ops::Mul<$helper> for &$source {
            type Output = $source;

            fn mul(self, rhs: $helper) -> $source {
                <$source>::from_num_checked(self.as_num() * rhs)
            }
        }

        impl std::ops::Mul<&$helper> for &$source {
            type Output = $source;

            fn mul(self, rhs: &$helper) -> $source {
                <$source>::from_num_checked(self.as_num() * rhs)
            }
        }

        impl std::ops::Mul<&$source> for $source {
            type Output = Self;

            fn mul(self, rhs: &Self) -> Self {
                Self::from_num_checked(self.as_num() * rhs.as_num())
            }
        }

        impl std::ops::Mul<$helper> for $source {
            type Output = Self;

            fn mul(self, rhs: $helper) -> Self {
                Self::from_num_checked(self.as_num() * rhs)
            }
        }

        impl std::ops::Mul<&$helper> for $source {
            type Output = Self;

            fn mul(self, rhs: &$helper) -> Self {
                Self::from_num_checked(self.as_num() * rhs)
            }
        }

        impl std::ops::MulAssign for $source {
            fn mul_assign(&mut self, rhs: Self) {
                *self = Self::from_num_checked(self.as_num() * rhs.as_num());
            }
        }

        impl std::ops::MulAssign<&$source> for $source {
            fn mul_assign(&mut self, rhs: &Self) {
                *self = Self::from_num_checked(self.as_num() * rhs.as_num());
            }
        }

        impl std::ops::MulAssign<$helper> for $source {
            fn mul_assign(&mut self, rhs: $helper) {
                *self = Self::from_num_checked(self.as_num() * rhs);
            }
        }

        impl std::ops::MulAssign<&$helper> for $source {
            fn mul_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num_checked(self.as_num() * rhs);
            }
        }

        impl std::ops::Div for $source {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl std::ops::Div<$source> for &$source {
            type Output = $source;

            fn div(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl std::ops::Div<&$source> for &$source {
            type Output = $source;
            fn div(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl std::ops::Div<$helper> for &$source {
            type Output = $source;

            fn div(self, rhs: $helper) -> $source {
                <$source>::from_num(self.as_num() / rhs)
            }
        }

        impl std::ops::Div<&$helper> for &$source {
            type Output = $source;

            fn div(self, rhs: &$helper) -> $source {
                <$source>::from_num(self.as_num() / rhs)
            }
        }

        impl std::ops::Div<&$source> for $source {
            type Output = Self;

            fn div(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() / rhs.as_num())
            }
        }

        impl std::ops::Div<$helper> for $source {
            type Output = Self;

            fn div(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() / rhs)
            }
        }

        impl std::ops::Div<&$helper> for $source {
            type Output = Self;

            fn div(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() / rhs)
            }
        }

        impl std::ops::DivAssign for $source {
            fn div_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() / rhs.as_num());
            }
        }

        impl std::ops::DivAssign<&$source> for $source {
            fn div_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() / rhs.as_num());
            }
        }

        impl std::ops::DivAssign<$helper> for $source {
            fn div_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() / rhs);
            }
        }

        impl std::ops::DivAssign<&$helper> for $source {
            fn div_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() / rhs);
            }
        }

        impl std::ops::Rem for $source {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl std::ops::Rem<$source> for &$source {
            type Output = $source;

            fn rem(self, rhs: $source) -> $source {
                <$source>::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl std::ops::Rem<&$source> for &$source {
            type Output = $source;
            fn rem(self, rhs: &$source) -> $source {
                <$source>::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl std::ops::Rem<$helper> for &$source {
            type Output = $source;

            fn rem(self, rhs: $helper) -> $source {
                <$source>::from_num(self.as_num() % rhs)
            }
        }

        impl std::ops::Rem<&$helper> for &$source {
            type Output = $source;

            fn rem(self, rhs: &$helper) -> $source {
                <$source>::from_num(self.as_num() % rhs)
            }
        }

        impl std::ops::Rem<&$source> for $source {
            type Output = Self;

            fn rem(self, rhs: &Self) -> Self {
                Self::from_num(self.as_num() % rhs.as_num())
            }
        }

        impl std::ops::Rem<$helper> for $source {
            type Output = Self;

            fn rem(self, rhs: $helper) -> Self {
                Self::from_num(self.as_num() % rhs)
            }
        }

        impl std::ops::Rem<&$helper> for $source {
            type Output = Self;

            fn rem(self, rhs: &$helper) -> Self {
                Self::from_num(self.as_num() % rhs)
            }
        }

        impl std::ops::RemAssign for $source {
            fn rem_assign(&mut self, rhs: Self) {
                *self = Self::from_num(self.as_num() % rhs.as_num());
            }
        }

        impl std::ops::RemAssign<&$source> for $source {
            fn rem_assign(&mut self, rhs: &Self) {
                *self = Self::from_num(self.as_num() % rhs.as_num());
            }
        }

        impl std::ops::RemAssign<$helper> for $source {
            fn rem_assign(&mut self, rhs: $helper) {
                *self = Self::from_num(self.as_num() % rhs);
            }
        }

        impl std::ops::RemAssign<&$helper> for $source {
            fn rem_assign(&mut self, rhs: &$helper) {
                *self = Self::from_num(self.as_num() % rhs);
            }
        }

        impl From<&$source> for $source {
            fn from(value: &$source) -> Self {
                return value.clone();
            }
        }
    };
}

#[macro_export]
macro_rules! common_fn_impl {
    ($source:ty, $helper:ty, $size:literal) => {
        #[inline(always)]
        pub(crate) const fn from_num_checked(n: $helper) -> Self {
            debug_assert!(n <= Self::MAX_VALUE, "overflow");
            return Self::from_num(n);
        }

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
        pub const fn from_be(x: Self) -> Self {
            x.swap_bytes()
        }

        #[cfg(target_endian = "big")]
        #[inline]
        pub const fn from_be(x: Self) -> Self {
            x
        }

        #[cfg(target_endian = "little")]
        #[inline]
        pub const fn from_le(x: Self) -> Self {
            x
        }

        #[cfg(target_endian = "big")]
        #[inline]
        pub const fn from_le(x: Self) -> Self {
            x.swap_bytes()
        }

        #[inline]
        pub const fn shr(self, rhs: u32) -> Self {
            Self::from_num(self.as_num() >> rhs)
        }

        #[inline]
        pub const fn shl(self, rhs: u32) -> Self {
            Self::from_num(self.as_num() << rhs)
        }

        #[inline]
        pub const fn sub(self, rhs: Self) -> Self {
            Self::from_num(self.as_num() - rhs.as_num())
        }

        #[inline]
        pub const fn mul(self, rhs: Self) -> Self {
            Self::from_num(self.as_num() * rhs.as_num())
        }

        #[inline]
        pub const fn eq(&self, other: &Self) -> bool {
            return self.as_num() == other.as_num();
        }

        #[inline]
        pub const fn gt(&self, other: &Self) -> bool {
            return self.as_num() > other.as_num();
        }

        #[inline]
        pub const fn ge(&self, other: &Self) -> bool {
            return self.as_num() >= other.as_num();
        }

        #[inline]
        pub const fn lt(&self, other: &Self) -> bool {
            return self.as_num() < other.as_num();
        }

        #[inline]
        pub const fn le(&self, other: &Self) -> bool {
            return self.as_num() <= other.as_num();
        }

        #[inline]
        pub const fn add(self, rhs: Self) -> Self {
            Self::from_num(self.as_num() + rhs.as_num())
        }

        #[inline]
        pub const fn rem(self, rhs: Self) -> Self {
            Self::from_num(self.as_num() % rhs.as_num())
        }

        #[inline]
        pub const fn div(self, rhs: Self) -> Self {
            Self::from_num(self.as_num() / rhs.as_num())
        }

        #[inline]
        pub fn from_str_radix(src: &str, radix: u32) -> Result<Self, std::num::ParseIntError> {
            let r = <$helper>::from_str_radix(src, radix)?;
            if r > Self::MAX_VALUE {
                //Pain
                u8::from_str_radix("FFF", 16)?;
                unreachable!();
            }

            return Ok(Self::from_num(r));
        }
        #[inline]
        pub const fn rotate_right(self, n: u32) -> Self {
            let nsz = n as usize % Self::NUM_BITS;
            let accumulator = self.as_num();
            let remainder = accumulator >> n;
            let shift_off = accumulator << (Self::NUM_BITS - nsz);
            let assembled = remainder | shift_off;
            return Self::from_num(assembled);
        }

        #[inline]
        pub const fn rotate_left(self, n: u32) -> Self {
            let nsz = n as usize % Self::NUM_BITS;
            let accumulator = self.as_num();
            let remainder = accumulator << n;
            let shift_off = accumulator >> (Self::NUM_BITS - nsz);
            let assembled = remainder | shift_off;
            return Self::from_num(assembled);
        }

        #[inline]
        pub const fn count_zeros(self) -> u32 {
            return self.as_num().count_zeros() - Self::NUM_BITS_MISSING_FOR_ALIGNMENT as u32;
        }

        #[inline]
        pub fn count_ones(self) -> u32 {
            self.as_num().count_ones()
        }

        #[inline]
        pub const fn leading_zeros(self) -> u32 {
            return self.as_num().leading_zeros() - Self::NUM_BITS_MISSING_FOR_ALIGNMENT as u32;
        }

        #[inline]
        pub const fn trailing_zeros(self) -> u32 {
            let n = self.as_num().trailing_zeros();
            if n > Self::NUM_BITS as u32 {
                return Self::NUM_BITS as u32;
            }

            return n;
        }

        #[inline]
        pub const fn leading_ones(self) -> u32 {
            return (self.as_num() | !Self::MAX_VALUE).leading_ones()
                - Self::NUM_BITS_MISSING_FOR_ALIGNMENT as u32;
        }

        #[inline]
        pub const fn trailing_ones(self) -> u32 {
            return self.as_num().trailing_ones();
        }

        #[inline]
        pub const fn reverse_bits(self) -> Self {
            let mut swap = self.swap_bytes();
            let n = 0;
            while n < swap.0.len() {
                swap.0[n] = swap.0[n].reverse_bits();
            }

            return swap;
        }

        #[inline]
        pub const fn checked_add(self, rhs: Self) -> Option<Self> {
            let n = self.as_num() + rhs.as_num();
            if n & !Self::MAX_VALUE != 0 {
                return None;
            }

            return Some(Self::from_num(n));
        }

        #[inline]
        pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
            return match self.as_num().checked_sub(rhs.as_num()) {
                Some(x) => Some(Self::from_num(x)),
                None => None,
            };
        }

        #[inline]
        pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
            return match self.as_num().checked_mul(rhs.as_num()) {
                Some(x) => Some(Self::from_num(x)),
                None => None,
            };
        }

        #[inline]
        pub const fn checked_div(self, rhs: Self) -> Option<Self> {
            if rhs.as_num() == 0 {
                return None;
            }

            return Some(Self::from_num(self.as_num() / rhs.as_num()));
        }

        #[inline]
        pub const fn div_euclid(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num() / rhs.as_num());
        }

        #[inline]
        pub const fn rem_euclid(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num() % rhs.as_num());
        }

        #[inline]
        pub const fn div_ceil(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().div_ceil(rhs.as_num()));
        }

        #[inline]
        pub const fn next_multiple_of(self, rhs: Self) -> Self {
            let result = self.as_num().next_multiple_of(rhs.as_num());
            debug_assert!(result > Self::MAX_VALUE, "Overflow");
            return Self::from_num(result);
        }

        #[inline]
        pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
            return match self.as_num().checked_mul(rhs.as_num()) {
                Some(result) => {
                    if result & !Self::MAX_VALUE != 0 {
                        return None;
                    }

                    return Some(Self::from_num(result));
                }
                None => None,
            };
        }

        #[inline]
        pub const fn is_power_of_two(self) -> bool {
            return self.as_num().is_power_of_two();
        }

        #[inline]
        pub const fn next_power_of_two(self) -> Self {
            let result = self.as_num().next_power_of_two();
            debug_assert!(result > Self::MAX_VALUE, "Overflow");
            return Self::from_num(result);
        }

        #[inline]
        pub const fn checked_next_power_of_two(self) -> Option<Self> {
            return match self.as_num().checked_next_power_of_two() {
                Some(result) => {
                    if result & !Self::MAX_VALUE != 0 {
                        return None;
                    }

                    return Some(Self::from_num(result));
                }
                None => None,
            };
        }

        #[inline]
        pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
            if rhs >= Self::NUM_BITS as u32 {
                return (Self::from_num(self.as_num() >> (Self::NUM_BITS - 1)), true);
            }

            return (Self::from_num(self.as_num() >> rhs), false);
        }

        #[inline]
        pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
            if rhs >= Self::NUM_BITS as u32 {
                return (Self::from_num(self.as_num() << (Self::NUM_BITS - 1)), true);
            }

            return (Self::from_num(self.as_num() << rhs), false);
        }

        #[inline]
        pub const fn overflowing_neg(self) -> (Self, bool) {
            let (a, b) = self.as_num().overflowing_neg();
            return (Self::from_num(a), b);
        }

        #[inline]
        pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
            return (self.rem_euclid(rhs), false);
        }

        #[inline]
        pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
            return (self.rem(rhs), false);
        }

        #[inline]
        pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
            return (self.div_euclid(rhs), false);
        }

        #[inline]
        pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
            return (self.div(rhs), false);
        }

        #[inline]
        pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
            let (a, b) = self.as_num().overflowing_mul(rhs.as_num());
            if b {
                return (Self::from_num(a), true);
            }

            if a & !Self::MAX_VALUE != 0 {
                return (Self::from_num(a), true);
            }

            return (Self::from_num(a), false);
        }

        #[inline]
        pub const fn abs_diff(self, other: Self) -> Self {
            return Self::from_num(self.as_num().abs_diff(other.as_num()));
        }

        #[inline]
        pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
            let (a, b) = self.as_num().overflowing_sub(rhs.as_num());
            return (Self::from_num(a), b);
        }

        #[inline]
        pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
            let result = self.as_num() + rhs.as_num();
            if result & !Self::MAX_VALUE != 0 {
                return (Self::from_num(result), true);
            }

            return (Self::from_num(result), false);
        }

        #[inline]
        pub const fn wrapping_pow(self, exp: u32) -> Self {
            return Self::from_num(self.as_num().pow(exp));
        }

        #[inline]
        pub const fn wrapping_shr(self, rhs: u32) -> Self {
            return Self::from_num(self.as_num().wrapping_shr(rhs));
        }

        #[inline]
        pub const fn wrapping_shl(self, rhs: u32) -> Self {
            return Self::from_num(self.as_num().wrapping_shl(rhs));
        }

        #[inline]
        pub const fn wrapping_neg(self) -> Self {
            return Self::from_num(self.as_num().wrapping_neg());
        }

        #[inline]
        pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_rem_euclid(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_rem(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_rem(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_div_euclid(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_div(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_div(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_mul(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_mul(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_sub(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_sub(rhs.as_num()));
        }

        #[inline]
        pub const fn wrapping_add(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().wrapping_add(rhs.as_num()));
        }

        #[inline]
        pub const fn saturating_pow(self, exp: u32) -> Self {
            return match self.as_num().checked_pow(exp) {
                None => Self::MAX,
                Some(v) => {
                    if v >= Self::MAX_VALUE {
                        return Self::MAX;
                    }

                    return Self::from_num(v);
                }
            };
        }

        #[inline]
        pub const fn saturating_div(self, rhs: Self) -> Self {
            if rhs.eq(&Self::MIN) {
                return Self::MAX;
            }

            return self.div(rhs);
        }

        #[inline]
        pub const fn saturating_mul(self, rhs: Self) -> Self {
            return match self.as_num().checked_mul(rhs.as_num()) {
                None => Self::MAX,
                Some(v) => {
                    if v >= Self::MAX_VALUE {
                        return Self::MAX;
                    }

                    return Self::from_num(v);
                }
            };
        }

        #[inline]
        pub const fn saturating_sub(self, rhs: Self) -> Self {
            return Self::from_num(self.as_num().saturating_sub(rhs.as_num()));
        }

        #[inline]
        pub const fn saturating_add(self, rhs: Self) -> Self {
            let result = self.as_num() + rhs.as_num();
            if result > Self::MAX_VALUE {
                return Self::MAX;
            }

            return Self::from_num(result);
        }

        #[inline]
        pub const fn checked_pow(self, exp: u32) -> Option<Self> {
            return match self.as_num().checked_pow(exp) {
                None => None,
                Some(x) => {
                    if x > Self::MAX_VALUE {
                        return None;
                    }

                    return Some(Self::from_num(x));
                }
            };
        }

        #[inline]
        pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
            if rhs >= Self::NUM_BITS as u32 {
                return None;
            }

            return Some(self.shr(rhs));
        }

        #[inline]
        pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
            if rhs >= Self::NUM_BITS as u32 {
                return None;
            }

            return Some(self.shl(rhs));
        }

        #[inline]
        pub const fn checked_neg(self) -> Option<Self> {
            if self.eq(&Self::MIN) {
                return Some(Self::MIN);
            }

            return None;
        }

        #[inline]
        pub const fn ilog(self, base: Self) -> u32 {
            return self.as_num().ilog(base.as_num());
        }

        #[inline]
        pub const fn ilog2(self) -> u32 {
            return self.as_num().ilog2();
        }

        #[inline]
        pub const fn ilog10(self) -> u32 {
            return self.as_num().ilog10();
        }

        #[inline]
        pub const fn checked_ilog(self, base: Self) -> Option<u32> {
            return self.as_num().checked_ilog(base.as_num());
        }

        #[inline]
        pub const fn checked_ilog10(self) -> Option<u32> {
            return self.as_num().checked_ilog10();
        }

        #[inline]
        pub const fn checked_ilog2(self) -> Option<u32> {
            return self.as_num().checked_ilog2();
        }

        #[inline]
        pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
            if rhs.eq(&Self::MIN) {
                return None;
            }

            return Some(self.rem_euclid(rhs));
        }

        #[inline]
        pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
            if rhs.eq(&Self::MIN) {
                return None;
            }

            return Some(self.rem(rhs));
        }

        #[inline]
        pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
            if rhs.eq(&Self::MIN) {
                return None;
            }

            return Some(self.div_euclid(rhs));
        }

        #[inline]
        pub const fn pow(self, exp: u32) -> Self {
            Self::from_num(self.as_num().pow(exp))
        }
    };
}

#[cfg(feature = "unsafe_fetch")]
#[macro_export]
macro_rules! unsafe_math_impl {
    ($source:ty, $helper:ty) => {
        #[cfg(target_endian = "little")]
        #[inline]
        pub unsafe fn fetch_unsafe(data: *const Self) -> $helper {
            return data.cast::<$helper>().read_unaligned();
        }

        #[cfg(target_endian = "big")]
        #[inline]
        pub unsafe fn fetch_unsafe(data: *const Self) -> $helper {
            return data.cast::<$helper>().read_unaligned() >> Self::NUM_BITS_MISSING_FOR_ALIGNMENT;
        }

        #[cfg(target_endian = "little")]
        #[inline]
        pub unsafe fn fetch_unsafe_clamped(data: *const Self) -> $helper {
            return data.cast::<$helper>().read_unaligned() & Self::MAX_VALUE;
        }

        #[cfg(target_endian = "big")]
        #[inline]
        pub unsafe fn fetch_unsafe_clamped(data: *const Self) -> $helper {
            return data.cast::<$helper>().read_unaligned() >> Self::NUM_BITS_MISSING_FOR_ALIGNMENT;
        }

        #[inline]
        pub unsafe fn unsafe_add_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) + Self::fetch_unsafe_clamped(rhs);
        }

        #[inline]
        pub unsafe fn unsafe_add_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(
                Self::fetch_unsafe_clamped(lhs) + Self::fetch_unsafe_clamped(rhs),
            );
        }

        #[inline]
        pub unsafe fn unsafe_add_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe_clamped(lhs) + rhs);
        }

        #[inline]
        pub unsafe fn unsafe_add_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) + rhs;
        }

        #[inline]
        pub unsafe fn unsafe_sub_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) - Self::fetch_unsafe_clamped(rhs);
        }

        #[inline]
        pub unsafe fn unsafe_sub_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) - rhs;
        }

        #[inline]
        pub unsafe fn unsafe_sub_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(
                Self::fetch_unsafe_clamped(lhs) - Self::fetch_unsafe_clamped(rhs),
            );
        }

        #[inline]
        pub unsafe fn unsafe_sub_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe_clamped(lhs) - rhs);
        }

        #[inline]
        pub unsafe fn unsafe_mul_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) * Self::fetch_unsafe_clamped(rhs);
        }

        #[inline]
        pub unsafe fn unsafe_mul_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) * rhs;
        }

        #[inline]
        pub unsafe fn unsafe_mul_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(
                Self::fetch_unsafe_clamped(lhs) * Self::fetch_unsafe_clamped(rhs),
            );
        }

        #[inline]
        pub unsafe fn unsafe_mul_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe_clamped(lhs) * rhs);
        }

        #[inline]
        pub unsafe fn unsafe_div_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) / Self::fetch_unsafe_clamped(rhs);
        }

        #[inline]
        pub unsafe fn unsafe_div_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) / rhs;
        }

        #[inline]
        pub unsafe fn unsafe_div_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(
                Self::fetch_unsafe_clamped(lhs) / Self::fetch_unsafe_clamped(rhs),
            );
        }

        #[inline]
        pub unsafe fn unsafe_div_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe_clamped(lhs) / rhs);
        }

        #[inline]
        pub unsafe fn unsafe_rem_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) % Self::fetch_unsafe_clamped(rhs);
        }

        #[inline]
        pub unsafe fn unsafe_rem_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) % rhs;
        }

        #[inline]
        pub unsafe fn unsafe_rem_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(
                Self::fetch_unsafe_clamped(lhs) % Self::fetch_unsafe_clamped(rhs),
            );
        }

        #[inline]
        pub unsafe fn unsafe_rem_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe_clamped(lhs) % rhs);
        }

        #[inline]
        pub unsafe fn unsafe_or_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return (Self::fetch_unsafe(lhs) | Self::fetch_unsafe(rhs)) & Self::MAX_VALUE;
        }

        #[inline]
        pub unsafe fn unsafe_or_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) | rhs;
        }

        #[inline]
        pub unsafe fn unsafe_or_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) | Self::fetch_unsafe(rhs));
        }

        #[inline]
        pub unsafe fn unsafe_or_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) | rhs);
        }

        #[inline]
        pub unsafe fn unsafe_and_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return (Self::fetch_unsafe(lhs) & Self::fetch_unsafe(rhs)) & Self::MAX_VALUE;
        }

        #[inline]
        pub unsafe fn unsafe_and_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe(lhs) & rhs;
        }

        #[inline]
        pub unsafe fn unsafe_and_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) & Self::fetch_unsafe(rhs));
        }

        #[inline]
        pub unsafe fn unsafe_and_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) & rhs);
        }

        #[inline]
        pub unsafe fn unsafe_xor_into_aligned(lhs: *const Self, rhs: *const Self) -> $helper {
            return (Self::fetch_unsafe(lhs) ^ Self::fetch_unsafe(rhs)) & Self::MAX_VALUE;
        }

        #[inline]
        pub unsafe fn unsafe_xor_with_aligned_into_aligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> $helper {
            return Self::fetch_unsafe_clamped(lhs) ^ rhs;
        }

        #[inline]
        pub unsafe fn unsafe_xor_into_unaligned(lhs: *const Self, rhs: *const Self) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) ^ Self::fetch_unsafe(rhs));
        }

        #[inline]
        pub unsafe fn unsafe_xor_with_aligned_into_unaligned(
            lhs: *const Self,
            rhs: $helper,
        ) -> Self {
            return Self::from_num(Self::fetch_unsafe(lhs) ^ rhs);
        }
    };
}

#[cfg(feature = "num_traits_support")]
#[macro_export]
macro_rules! num_traits_impl {
    ($source:ty) => {
        impl num_traits::One for $source {
            fn one() -> Self {
                return Self::from_num(1);
            }
        }

        impl num_traits::Zero for $source {
            fn zero() -> Self {
                Self::MIN
            }

            fn is_zero(&self) -> bool {
                return *self == 0;
            }
        }

        impl num_traits::Bounded for $source {
            fn min_value() -> Self {
                return Self::MIN;
            }

            fn max_value() -> Self {
                return Self::MAX;
            }
        }

        impl num_traits::Num for $source {
            type FromStrRadixErr = std::num::ParseIntError;

            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                return Self::from_str_radix(str, radix);
            }
        }

        impl num_traits::Saturating for $source {
            fn saturating_add(self, v: Self) -> Self {
                Self::saturating_add(self, v)
            }

            fn saturating_sub(self, v: Self) -> Self {
                Self::saturating_sub(self, v)
            }
        }

        impl num_traits::ToPrimitive for $source {
            fn to_isize(&self) -> Option<isize> {
                Some(self.as_num() as isize)
            }

            fn to_i8(&self) -> Option<i8> {
                Some(self.as_num() as i8)
            }

            fn to_i16(&self) -> Option<i16> {
                Some(self.as_num() as i16)
            }

            fn to_i32(&self) -> Option<i32> {
                Some(self.as_num() as i32)
            }

            fn to_i64(&self) -> Option<i64> {
                Some(self.as_num() as i64)
            }

            fn to_i128(&self) -> Option<i128> {
                Some(self.as_num() as i128)
            }

            fn to_usize(&self) -> Option<usize> {
                Some(self.as_num() as usize)
            }

            fn to_u8(&self) -> Option<u8> {
                Some(self.as_num() as u8)
            }

            fn to_u16(&self) -> Option<u16> {
                Some(self.as_num() as u16)
            }

            fn to_u32(&self) -> Option<u32> {
                Some(self.as_num() as u32)
            }

            fn to_u64(&self) -> Option<u64> {
                Some(self.as_num() as u64)
            }

            fn to_u128(&self) -> Option<u128> {
                Some(self.as_num() as u128)
            }

            fn to_f32(&self) -> Option<f32> {
                Some(self.as_num() as f32)
            }

            fn to_f64(&self) -> Option<f64> {
                Some(self.as_num() as f64)
            }
        }

        impl num_traits::CheckedAdd for $source {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                let n = self.as_num() + v.as_num();
                if n & !Self::MAX_VALUE != 0 {
                    return None;
                }

                return Some(Self::from_num(n));
            }
        }

        impl num_traits::CheckedSub for $source {
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                return match self.as_num().checked_sub(v.as_num()) {
                    Some(x) => Some(Self::from_num(x)),
                    None => None,
                };
            }
        }

        impl num_traits::CheckedMul for $source {
            fn checked_mul(&self, v: &Self) -> Option<Self> {
                return match self.as_num().checked_mul(v.as_num()) {
                    Some(x) => {
                        if x > Self::MAX_VALUE {
                            return None;
                        }
                        return Some(Self::from_num(x));
                    }
                    None => None,
                };
            }
        }

        impl num_traits::CheckedDiv for $source {
            fn checked_div(&self, v: &Self) -> Option<Self> {
                if num_traits::Zero::is_zero(v) {
                    return None;
                }

                return Some(Self::from_num(self.as_num() / v.as_num()));
            }
        }

        impl num_traits::NumCast for $source {
            fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
                let m = n.to_u128();
                if m.is_some() {
                    return Some(<Self as From<u128>>::from(m.unwrap()));
                }

                let m = n.to_u64();
                if m.is_some() {
                    return Some(<Self as From<u64>>::from(m.unwrap()));
                }

                let m = n.to_u32();
                if m.is_some() {
                    return Some(<Self as From<u32>>::from(m.unwrap()));
                }

                let m = n.to_u16();
                if m.is_some() {
                    return Some(<Self as From<u16>>::from(m.unwrap()));
                }

                let m = n.to_u8();
                if m.is_some() {
                    return Some(<Self as From<u8>>::from(m.unwrap()));
                }

                return None;
            }
        }

        impl num_traits::PrimInt for $source {
            fn count_ones(self) -> u32 {
                Self::count_ones(self)
            }

            fn count_zeros(self) -> u32 {
                Self::count_zeros(self)
            }

            fn leading_zeros(self) -> u32 {
                Self::leading_zeros(self)
            }

            fn trailing_zeros(self) -> u32 {
                Self::trailing_zeros(self)
            }

            fn rotate_left(self, n: u32) -> Self {
                Self::rotate_left(self, n)
            }

            fn rotate_right(self, n: u32) -> Self {
                Self::rotate_right(self, n)
            }

            fn signed_shl(self, n: u32) -> Self {
                Self::shl(self, n)
            }

            fn signed_shr(self, n: u32) -> Self {
                Self::shr(self, n)
            }

            fn unsigned_shl(self, n: u32) -> Self {
                Self::shl(self, n)
            }

            fn unsigned_shr(self, n: u32) -> Self {
                Self::shl(self, n)
            }

            fn swap_bytes(self) -> Self {
                Self::swap_bytes(self)
            }

            fn from_be(x: Self) -> Self {
                Self::from_be(x)
            }

            fn from_le(x: Self) -> Self {
                Self::from_le(x)
            }

            fn to_be(self) -> Self {
                Self::to_be(self)
            }

            fn to_le(self) -> Self {
                Self::to_le(self)
            }

            fn pow(self, exp: u32) -> Self {
                Self::pow(self, exp)
            }
        }
    };
}

#[cfg(feature = "ux_support")]
#[macro_export]
///
/// The guy forgot to implement Into trait for the wrapper of any 128 bit aligned types. :(
/// We got format tho and can format+parse. Slow but it works oh well...
///
macro_rules! ux_conversion_via_format {
    ($uintx_type:ty, $uintx_intermediary:ty, $intermediary:ty, $intermediary_signed:ty, $mask:expr, $sign_bit:expr, $ux_type:ty, $ux_type_signed:ty) => {
        impl Into<$ux_type> for $uintx_type {
            fn into(self) -> $ux_type {
                return <$ux_type>::new((self.as_num() as $intermediary) & $mask);
            }
        }

        impl Into<$ux_type> for &$uintx_type {
            fn into(self) -> $ux_type {
                return <$ux_type>::new((self.as_num() as $intermediary) & $mask);
            }
        }

        impl Into<$ux_type_signed> for $uintx_type {
            fn into(self) -> $ux_type_signed {
                let mut num = (self.as_num() as $intermediary) & $mask;
                if (num & (1 << $sign_bit)) != 0 {
                    num |= !($mask as $intermediary)
                }

                return <$ux_type_signed>::new(num as $intermediary_signed);
            }
        }

        impl Into<$ux_type_signed> for &$uintx_type {
            fn into(self) -> $ux_type_signed {
                let mut num = (self.as_num() as $intermediary) & $mask;
                if (num & (1 << $sign_bit)) != 0 {
                    num |= !($mask as $intermediary)
                }

                return <$ux_type_signed>::new(num as $intermediary_signed);
            }
        }

        impl From<$ux_type> for $uintx_type {
            fn from(value: $ux_type) -> Self {
                let x: $intermediary =
                    <$intermediary>::from_str_radix(format!("{:X}", value).as_str(), 16)
                        .expect("Conversion failed");
                return <$uintx_type>::from_num(x as $uintx_intermediary);
            }
        }

        impl From<&$ux_type> for $uintx_type {
            fn from(value: &$ux_type) -> Self {
                let x: $intermediary =
                    <$intermediary>::from_str_radix(format!("{:X}", *value).as_str(), 16)
                        .expect("Conversion failed");
                return <$uintx_type>::from_num(x as $uintx_intermediary);
            }
        }

        impl From<&$ux_type_signed> for $uintx_type {
            fn from(value: &$ux_type_signed) -> Self {
                let x: $intermediary_signed =
                    <$intermediary_signed>::from_str_radix(format!("{:X}", *value).as_str(), 16)
                        .expect("Conversion failed");
                return <$uintx_type>::from_num(x as $intermediary as $uintx_intermediary);
            }
        }

        impl From<$ux_type_signed> for $uintx_type {
            fn from(value: $ux_type_signed) -> Self {
                let x: $intermediary_signed =
                    <$intermediary_signed>::from_str_radix(format!("{:X}", value).as_str(), 16)
                        .expect("Conversion failed");
                return <$uintx_type>::from_num(x as $intermediary as $uintx_intermediary);
            }
        }
    };
}

#[cfg(feature = "ux_support")]
#[macro_export]
macro_rules! ux_conversion {
    ($uintx_type:ty, $uintx_intermediary:ty, $intermediary:ty, $intermediary_signed:ty, $mask:expr, $sign_bit:expr, $ux_type:ty, $ux_type_signed:ty) => {
        impl Into<$ux_type> for $uintx_type {
            fn into(self) -> $ux_type {
                return <$ux_type>::new((self.as_num() as $intermediary) & $mask);
            }
        }

        impl Into<$ux_type> for &$uintx_type {
            fn into(self) -> $ux_type {
                return <$ux_type>::new((self.as_num() as $intermediary) & $mask);
            }
        }

        impl Into<$ux_type_signed> for $uintx_type {
            fn into(self) -> $ux_type_signed {
                let mut num = (self.as_num() as $intermediary) & $mask;
                if (num & (1 << $sign_bit)) != 0 {
                    num |= !($mask as $intermediary)
                }

                return <$ux_type_signed>::new(num as $intermediary_signed);
            }
        }

        impl Into<$ux_type_signed> for &$uintx_type {
            fn into(self) -> $ux_type_signed {
                let mut num = (self.as_num() as $intermediary) & $mask;
                if (num & (1 << $sign_bit)) != 0 {
                    num |= !($mask as $intermediary)
                }

                return <$ux_type_signed>::new(num as $intermediary_signed);
            }
        }

        impl From<$ux_type> for $uintx_type {
            fn from(value: $ux_type) -> Self {
                let x: $intermediary = value.into();
                return <$uintx_type>::from_num(x as $uintx_intermediary);
            }
        }

        impl From<&$ux_type> for $uintx_type {
            fn from(value: &$ux_type) -> Self {
                let x: $intermediary = (*value).into();
                return <$uintx_type>::from_num(x as $uintx_intermediary);
            }
        }

        impl From<&$ux_type_signed> for $uintx_type {
            fn from(value: &$ux_type_signed) -> Self {
                let x: $intermediary_signed = (*value).into();
                return <$uintx_type>::from_num(x as $intermediary as $uintx_intermediary);
            }
        }

        impl From<$ux_type_signed> for $uintx_type {
            fn from(value: $ux_type_signed) -> Self {
                let x: $intermediary_signed = value.into();
                return <$uintx_type>::from_num(x as $intermediary as $uintx_intermediary);
            }
        }
    };
}

#[cfg(feature = "intx_support")]
#[macro_export]
macro_rules! intx_conv_impl {
    ($uintx_type:ty, $uintx_intermediary:ty, $intx_intermediary:ty, $intx_type:ty, $intx_signed_type:ty) => {
        impl From<$intx_type> for $uintx_type {
            fn from(value: $intx_type) -> Self {
                let helper: $intx_intermediary = value.into();
                return <Self as From<$intx_intermediary>>::from(helper);
            }
        }

        impl From<&$intx_type> for $uintx_type {
            fn from(value: &$intx_type) -> Self {
                let helper: $intx_intermediary = (*value).into();
                return <Self as From<$intx_intermediary>>::from(helper);
            }
        }

        impl From<$intx_signed_type> for $uintx_type {
            fn from(value: $intx_signed_type) -> Self {
                let helper: $intx_intermediary =
                    <$intx_type>::from_ne_bytes(value.to_ne_bytes()).into();
                return <Self as From<$intx_intermediary>>::from(helper);
            }
        }

        impl From<&$intx_signed_type> for $uintx_type {
            fn from(value: &$intx_signed_type) -> Self {
                let helper: $intx_intermediary =
                    <$intx_type>::from_ne_bytes(value.to_ne_bytes()).into();
                return <Self as From<$intx_intermediary>>::from(helper);
            }
        }

        impl Into<$intx_type> for $uintx_type {
            fn into(self) -> $intx_type {
                let helper: $uintx_intermediary = self.into();
                return <$intx_type>::from_ne_bytes(helper.to_ne_bytes());
            }
        }

        impl Into<$intx_type> for &$uintx_type {
            fn into(self) -> $intx_type {
                let helper: $uintx_intermediary = self.into();
                return <$intx_type>::from_ne_bytes(helper.to_ne_bytes());
            }
        }

        impl Into<$intx_signed_type> for $uintx_type {
            fn into(self) -> $intx_signed_type {
                let helper: $uintx_intermediary = self.into();
                return <$intx_signed_type>::from_ne_bytes(helper.to_ne_bytes());
            }
        }

        impl Into<$intx_signed_type> for &$uintx_type {
            fn into(self) -> $intx_signed_type {
                let helper: $uintx_intermediary = self.into();
                return <$intx_signed_type>::from_ne_bytes(helper.to_ne_bytes());
            }
        }
    };
}

#[macro_export]
macro_rules! from_to_impl_for_primitive {
    ($source:ty, $helper:ty, $target:ty) => {
        impl From<$target> for $source {
            fn from(value: $target) -> Self {
                return Self::from_num(value as $helper);
            }
        }

        impl From<&$target> for $source {
            fn from(value: &$target) -> Self {
                return Self::from_num(*value as $helper);
            }
        }

        impl Into<$target> for $source {
            fn into(self) -> $target {
                return self.as_num() as $target;
            }
        }

        impl Into<$target> for &$source {
            fn into(self) -> $target {
                return self.as_num() as $target;
            }
        }
    };
}

#[macro_export]
macro_rules! sh_impl_conv {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Shl<$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl std::ops::Shl<&$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl std::ops::ShlAssign<$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl std::ops::ShlAssign<&$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() << rhs.as_num());
            }
        }

        impl std::ops::Shr<$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl std::ops::Shr<&$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl std::ops::ShrAssign<$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() >> rhs.as_num());
            }
        }

        impl std::ops::ShrAssign<&$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() >> rhs.as_num());
            }
        }
    };
}

#[macro_export]
macro_rules! sh_impl {
    ($lhs:ty, $rhs:ty) => {
        impl std::ops::Shl<$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs);
            }
        }

        impl std::ops::Shl<&$rhs> for $lhs {
            type Output = $lhs;

            fn shl(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() << rhs);
            }
        }

        impl std::ops::ShlAssign<$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() << rhs);
            }
        }

        impl std::ops::ShlAssign<&$rhs> for $lhs {
            fn shl_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() << rhs);
            }
        }

        impl std::ops::Shr<$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: $rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs);
            }
        }

        impl std::ops::Shr<&$rhs> for $lhs {
            type Output = $lhs;

            fn shr(self, rhs: &$rhs) -> Self::Output {
                return Self::from_num(self.as_num() >> rhs);
            }
        }

        impl std::ops::ShrAssign<$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: $rhs) {
                *self = Self::from_num(self.as_num() >> rhs);
            }
        }

        impl std::ops::ShrAssign<&$rhs> for $lhs {
            fn shr_assign(&mut self, rhs: &$rhs) {
                *self = Self::from_num(self.as_num() >> rhs);
            }
        }
    };
}

#[macro_export]
macro_rules! type_impl {
    ($source:ty, $helper:ty, $size:literal) => {
        #[allow(dead_code)]
        impl $source {
            pub const MAX: $source = Self([0xFFu8; $size]);
            pub const MIN: $source = Self([0x00u8; $size]);
            pub const MAX_VALUE: $helper = Self([0xFFu8; $size]).as_num();
            pub const NUM_BITS: usize = $size * 8;
            pub const NUM_BITS_MISSING_FOR_ALIGNMENT: usize =
                (std::mem::size_of::<$helper>() - $size) * 8;

            crate::common_fn_impl!($source, $helper, $size);

            #[cfg(feature = "unsafe_fetch")]
            crate::unsafe_math_impl!($source, $helper);
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

        crate::common_traits_impl!($source, $helper, $size);

        crate::from_to_impl_for_primitive!($source, $helper, u8);
        crate::from_to_impl_for_primitive!($source, $helper, u16);
        crate::from_to_impl_for_primitive!($source, $helper, u32);
        crate::from_to_impl_for_primitive!($source, $helper, u64);
        crate::from_to_impl_for_primitive!($source, $helper, u128);
        crate::from_to_impl_for_primitive!($source, $helper, i8);
        crate::from_to_impl_for_primitive!($source, $helper, i16);
        crate::from_to_impl_for_primitive!($source, $helper, i32);
        crate::from_to_impl_for_primitive!($source, $helper, i64);
        crate::from_to_impl_for_primitive!($source, $helper, i128);

        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u24, u32, intx::U24, intx::I24);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, u32, u32, intx::U32, intx::I32);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u40, u64, intx::U40, intx::I40);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u48, u64, intx::U48, intx::I48);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u56, u64, intx::U56, intx::I56);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, u64, u64, intx::U64, intx::I64);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u72, u128, intx::U72, intx::I72);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u80, u128, intx::U80, intx::I80);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u88, u128, intx::U88, intx::I88);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u96, u128, intx::U96, intx::I96);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u104, u128, intx::U104, intx::I104);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u112, u128, intx::U112, intx::I112);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, crate::u120, u128, intx::U120, intx::I120);
        #[cfg(feature = "intx_support")]
        crate::intx_conv_impl!($source, u128, u128, intx::U128, intx::I128);

        crate::sh_impl_conv!($source, crate::u24);
        crate::sh_impl_conv!($source, crate::u40);
        crate::sh_impl_conv!($source, crate::u48);
        crate::sh_impl_conv!($source, crate::u56);
        crate::sh_impl_conv!($source, crate::u72);
        crate::sh_impl_conv!($source, crate::u80);
        crate::sh_impl_conv!($source, crate::u88);
        crate::sh_impl_conv!($source, crate::u96);
        crate::sh_impl_conv!($source, crate::u104);
        crate::sh_impl_conv!($source, crate::u112);
        crate::sh_impl_conv!($source, crate::u120);

        crate::sh_impl!($source, u8);
        crate::sh_impl!($source, u16);
        crate::sh_impl!($source, u32);
        crate::sh_impl!($source, u64);
        crate::sh_impl!($source, u128);
        crate::sh_impl!($source, i8);
        crate::sh_impl!($source, i16);
        crate::sh_impl!($source, i32);
        crate::sh_impl!($source, i64);
        crate::sh_impl!($source, i128);
        crate::sh_impl!($source, usize);
        crate::sh_impl!($source, isize);

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 127) - 1),
            126,
            ux::u127,
            ux::i127
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 126) - 1),
            125,
            ux::u126,
            ux::i126
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 125) - 1),
            124,
            ux::u125,
            ux::i125
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 124) - 1),
            123,
            ux::u124,
            ux::i124
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 123) - 1),
            122,
            ux::u123,
            ux::i123
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 122) - 1),
            121,
            ux::u122,
            ux::i122
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 121) - 1),
            120,
            ux::u121,
            ux::i121
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 120) - 1),
            119,
            ux::u120,
            ux::i120
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 119) - 1),
            118,
            ux::u119,
            ux::i119
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 118) - 1),
            117,
            ux::u118,
            ux::i118
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 117) - 1),
            116,
            ux::u117,
            ux::i117
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 116) - 1),
            115,
            ux::u116,
            ux::i116
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 115) - 1),
            114,
            ux::u115,
            ux::i115
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 114) - 1),
            113,
            ux::u114,
            ux::i114
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 113) - 1),
            112,
            ux::u113,
            ux::i113
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 112) - 1),
            111,
            ux::u112,
            ux::i112
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 111) - 1),
            110,
            ux::u111,
            ux::i111
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 110) - 1),
            109,
            ux::u110,
            ux::i110
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 109) - 1),
            108,
            ux::u109,
            ux::i109
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 108) - 1),
            107,
            ux::u108,
            ux::i108
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 107) - 1),
            106,
            ux::u107,
            ux::i107
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 106) - 1),
            105,
            ux::u106,
            ux::i106
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 105) - 1),
            104,
            ux::u105,
            ux::i105
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 104) - 1),
            103,
            ux::u104,
            ux::i104
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 103) - 1),
            102,
            ux::u103,
            ux::i103
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 102) - 1),
            101,
            ux::u102,
            ux::i102
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 101) - 1),
            100,
            ux::u101,
            ux::i101
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 100) - 1),
            99,
            ux::u100,
            ux::i100
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 99) - 1),
            98,
            ux::u99,
            ux::i99
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 98) - 1),
            97,
            ux::u98,
            ux::i98
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 97) - 1),
            96,
            ux::u97,
            ux::i97
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 96) - 1),
            95,
            ux::u96,
            ux::i96
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 95) - 1),
            94,
            ux::u95,
            ux::i95
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 94) - 1),
            93,
            ux::u94,
            ux::i94
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 93) - 1),
            92,
            ux::u93,
            ux::i93
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 92) - 1),
            91,
            ux::u92,
            ux::i92
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 91) - 1),
            90,
            ux::u91,
            ux::i91
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 90) - 1),
            89,
            ux::u90,
            ux::i90
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 89) - 1),
            88,
            ux::u89,
            ux::i89
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 88) - 1),
            87,
            ux::u88,
            ux::i88
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 87) - 1),
            86,
            ux::u87,
            ux::i87
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 86) - 1),
            85,
            ux::u86,
            ux::i86
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 85) - 1),
            84,
            ux::u85,
            ux::i85
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 84) - 1),
            83,
            ux::u84,
            ux::i84
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 83) - 1),
            82,
            ux::u83,
            ux::i83
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 82) - 1),
            81,
            ux::u82,
            ux::i82
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 81) - 1),
            80,
            ux::u81,
            ux::i81
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 80) - 1),
            79,
            ux::u80,
            ux::i80
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 79) - 1),
            78,
            ux::u79,
            ux::i79
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 78) - 1),
            77,
            ux::u78,
            ux::i78
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 77) - 1),
            76,
            ux::u77,
            ux::i77
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 76) - 1),
            75,
            ux::u76,
            ux::i76
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 75) - 1),
            74,
            ux::u75,
            ux::i75
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 74) - 1),
            73,
            ux::u74,
            ux::i74
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 73) - 1),
            72,
            ux::u73,
            ux::i73
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 72) - 1),
            71,
            ux::u72,
            ux::i72
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 71) - 1),
            70,
            ux::u71,
            ux::i71
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 70) - 1),
            69,
            ux::u70,
            ux::i70
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 69) - 1),
            68,
            ux::u69,
            ux::i69
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 68) - 1),
            67,
            ux::u68,
            ux::i68
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 67) - 1),
            66,
            ux::u67,
            ux::i67
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 66) - 1),
            65,
            ux::u66,
            ux::i66
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion_via_format!(
            $source,
            $helper,
            u128,
            i128,
            ((1u128 << 65) - 1),
            64,
            ux::u65,
            ux::i65
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 63) - 1),
            62,
            ux::u63,
            ux::i63
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 62) - 1),
            61,
            ux::u62,
            ux::i62
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 61) - 1),
            60,
            ux::u61,
            ux::i61
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 60) - 1),
            59,
            ux::u60,
            ux::i60
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 59) - 1),
            58,
            ux::u59,
            ux::i59
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 58) - 1),
            57,
            ux::u58,
            ux::i58
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 57) - 1),
            56,
            ux::u57,
            ux::i57
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 56) - 1),
            55,
            ux::u56,
            ux::i56
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 55) - 1),
            54,
            ux::u55,
            ux::i55
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 54) - 1),
            53,
            ux::u54,
            ux::i54
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 53) - 1),
            52,
            ux::u53,
            ux::i53
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 52) - 1),
            51,
            ux::u52,
            ux::i52
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 51) - 1),
            50,
            ux::u51,
            ux::i51
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 50) - 1),
            49,
            ux::u50,
            ux::i50
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 49) - 1),
            48,
            ux::u49,
            ux::i49
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 48) - 1),
            47,
            ux::u48,
            ux::i48
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 47) - 1),
            46,
            ux::u47,
            ux::i47
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 46) - 1),
            45,
            ux::u46,
            ux::i46
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 45) - 1),
            44,
            ux::u45,
            ux::i45
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 44) - 1),
            43,
            ux::u44,
            ux::i44
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 43) - 1),
            42,
            ux::u43,
            ux::i43
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 42) - 1),
            41,
            ux::u42,
            ux::i42
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 41) - 1),
            40,
            ux::u41,
            ux::i41
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 40) - 1),
            39,
            ux::u40,
            ux::i40
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 39) - 1),
            38,
            ux::u39,
            ux::i39
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 38) - 1),
            37,
            ux::u38,
            ux::i38
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 37) - 1),
            36,
            ux::u37,
            ux::i37
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 36) - 1),
            35,
            ux::u36,
            ux::i36
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 35) - 1),
            34,
            ux::u35,
            ux::i35
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 34) - 1),
            33,
            ux::u34,
            ux::i34
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u64,
            i64,
            ((1u64 << 33) - 1),
            32,
            ux::u33,
            ux::i33
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 31) - 1),
            30,
            ux::u31,
            ux::i31
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 30) - 1),
            29,
            ux::u30,
            ux::i30
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 29) - 1),
            28,
            ux::u29,
            ux::i29
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 28) - 1),
            27,
            ux::u28,
            ux::i28
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 27) - 1),
            26,
            ux::u27,
            ux::i27
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 26) - 1),
            25,
            ux::u26,
            ux::i26
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 25) - 1),
            24,
            ux::u25,
            ux::i25
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 24) - 1),
            23,
            ux::u24,
            ux::i24
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 23) - 1),
            22,
            ux::u23,
            ux::i23
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 22) - 1),
            21,
            ux::u22,
            ux::i22
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 21) - 1),
            20,
            ux::u21,
            ux::i21
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 20) - 1),
            19,
            ux::u20,
            ux::i20
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 19) - 1),
            18,
            ux::u19,
            ux::i19
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 18) - 1),
            17,
            ux::u18,
            ux::i18
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u32,
            i32,
            ((1u32 << 17) - 1),
            16,
            ux::u17,
            ux::i17
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 15) - 1),
            14,
            ux::u15,
            ux::i15
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 14) - 1),
            13,
            ux::u14,
            ux::i14
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 13) - 1),
            12,
            ux::u13,
            ux::i13
        );
        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 12) - 1),
            11,
            ux::u12,
            ux::i12
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 11) - 1),
            10,
            ux::u11,
            ux::i11
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 10) - 1),
            9,
            ux::u10,
            ux::i10
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u16,
            i16,
            ((1u16 << 9) - 1),
            8,
            ux::u9,
            ux::i9
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 7) - 1),
            6,
            ux::u7,
            ux::i7
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 6) - 1),
            5,
            ux::u6,
            ux::i6
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 5) - 1),
            4,
            ux::u5,
            ux::i5
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 4) - 1),
            3,
            ux::u4,
            ux::i4
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 3) - 1),
            2,
            ux::u3,
            ux::i3
        );

        #[cfg(feature = "ux_support")]
        crate::ux_conversion!(
            $source,
            $helper,
            u8,
            i8,
            ((1u8 << 2) - 1),
            1,
            ux::u2,
            ux::i2
        );

        #[cfg(feature = "num_traits_support")]
        crate::num_traits_impl!($source);
    };
}
