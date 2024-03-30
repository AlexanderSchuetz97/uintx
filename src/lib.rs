mod type_macro;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24(pub(crate) [u8; 3]);

impl u24 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u32 {
        let crate::u24([a, b, c]) = self;
        u32::from_ne_bytes([0, a, b, c])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u32 {
        let u24([a, b, c]) = self;
        u32::from_ne_bytes([a, b, c, 0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u32) -> Self {
        let [a, b, c, _] = n.to_ne_bytes();
        u24([a, b, c])
    }


    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u32) -> Self {
        let [_, b, c, d] = n.to_ne_bytes();
        u24([b, c, d])
    }

    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 3]) -> [u8; 3] {
        [data[2], data[1], data[0]]
    }




}
type_impl!(u24, u32, 3);
type_conversion!(u40, u32, u24);
type_conversion!(u48, u32, u24);
type_conversion!(u56, u32, u24);
type_conversion!(u72, u32, u24);
type_conversion!(u80, u32, u24);
type_conversion!(u88, u32, u24);
type_conversion!(u96, u32, u24);
type_conversion!(u104, u32, u24);
type_conversion!(u112, u32, u24);
type_conversion!(u120, u32, u24);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u40(pub(crate) [u8; 5]);

impl u40 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u40([a, b, c, d,e]) = self;
        u64::from_ne_bytes([0,0,0, a, b, c, d, e])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u40([a, b, c,d,e]) = self;
        u64::from_ne_bytes([a, b, c, d, e, 0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [a, b, c, d, e, _, _, _] = n.to_ne_bytes();
        u40([a, b, c, d, e])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [_, _, _, d, e, f, g, h] = n.to_ne_bytes();
        u40([d, e, f, g, h])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 5]) -> [u8; 5] {
        [data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u40, u64, 5);
type_conversion!(u24, u64, u40);
type_conversion!(u48, u64, u40);
type_conversion!(u56, u64, u40);
type_conversion!(u72, u64, u40);
type_conversion!(u80, u64, u40);
type_conversion!(u88, u64, u40);
type_conversion!(u96, u64, u40);
type_conversion!(u104, u64, u40);
type_conversion!(u112, u64, u40);
type_conversion!(u120, u64, u40);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u48(pub(crate) [u8; 6]);

impl u48 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u48([a, b, c, d, e, f]) = self;
        u64::from_ne_bytes([0,0, a, b, c, d, e, f])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u48([a, b, c,d,e, f]) = self;
        u64::from_ne_bytes([a, b, c, d, e, f,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [a, b, c, d, e, f, _, _] = n.to_ne_bytes();
        u48([a, b, c, d, e, f])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [_, _, c, d, e, f, g, h] = n.to_ne_bytes();
        u48([c, d, e, f, g, h])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 6]) -> [u8; 6] {
        [data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u48, u64, 6);
type_conversion!(u24, u64, u48);
type_conversion!(u40, u64, u48);
type_conversion!(u56, u64, u48);
type_conversion!(u72, u64, u48);
type_conversion!(u80, u64, u48);
type_conversion!(u88, u64, u48);
type_conversion!(u96, u64, u48);
type_conversion!(u104, u64, u48);
type_conversion!(u112, u64, u48);
type_conversion!(u120, u64, u48);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u56(pub(crate) [u8; 7]);

impl u56 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u56([a, b, c, d, e, f, g]) = self;
        u64::from_ne_bytes([0,a, b, c, d, e, f, g])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u56([a, b, c,d, e, f, g]) = self;
        u64::from_ne_bytes([a, b, c, d, e, f, g, 0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [a, b, c, d, e, f, g, _] = n.to_ne_bytes();
        u56([a, b, c, d, e, f, g])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u64) -> Self {
        let [_, b, c, d, e, f, g, h] = n.to_ne_bytes();
        u56([b, c, d, e, f, g, h])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 7]) -> [u8; 7] {
        [data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u56, u64, 7);
type_conversion!(u24, u64, u56);
type_conversion!(u40, u64, u56);
type_conversion!(u48, u64, u56);
type_conversion!(u72, u64, u56);
type_conversion!(u80, u64, u56);
type_conversion!(u88, u64, u56);
type_conversion!(u96, u64, u56);
type_conversion!(u104, u64, u56);
type_conversion!(u112, u64, u56);
type_conversion!(u120, u64, u56);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u72(pub(crate) [u8; 9]);

impl u72 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u72([a, b, c, d, e, f, g, h, i]) = self;
        u128::from_ne_bytes([0,0,0,0,0,0,0, a, b, c, d, e, f, g, h, i])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u72([a, b, c,d, e, f, g, h, i]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, 0,0,0,0,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, _, _, _, _, _, _, _] = n.to_ne_bytes();
        u72([a, b, c, d, e, f, g, h, i])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u72([h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 9]) -> [u8; 9] {
        [data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u72, u128, 9);
type_conversion!(u24, u128, u72);
type_conversion!(u40, u128, u72);
type_conversion!(u48, u128, u72);
type_conversion!(u56, u128, u72);
type_conversion!(u80, u128, u72);
type_conversion!(u88, u128, u72);
type_conversion!(u96, u128, u72);
type_conversion!(u104, u128, u72);
type_conversion!(u112, u128, u72);
type_conversion!(u120, u128, u72);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u80(pub(crate) [u8; 10]);

impl u80 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u80([a, b, c, d, e, f, g, h, i, j]) = self;
        u128::from_ne_bytes([0,0,0,0,0,0,a, b, c, d, e, f, g, h, i, j])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u80([a, b, c,d, e, f, g, h, i, j]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j,0,0,0,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, _, _, _, _, _, _] = n.to_ne_bytes();
        u80([a, b, c, d, e, f, g, h, i, j])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, _, _, _, _, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u80([g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 10]) -> [u8; 10] {
        [data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u80, u128, 10);
type_conversion!(u24, u128, u80);
type_conversion!(u40, u128, u80);
type_conversion!(u48, u128, u80);
type_conversion!(u56, u128, u80);
type_conversion!(u72, u128, u80);
type_conversion!(u88, u128, u80);
type_conversion!(u96, u128, u80);
type_conversion!(u104, u128, u80);
type_conversion!(u112, u128, u80);
type_conversion!(u120, u128, u80);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u88(pub(crate) [u8; 11]);

impl u88 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u88([a, b, c, d, e, f, g, h, i, j, k]) = self;
        u128::from_ne_bytes([0,0,0,0,0,a, b, c, d, e, f, g, h, i, j, k])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u88([a, b, c,d, e, f, g, h, i, j, k]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k,0,0,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, _, _, _, _, _] = n.to_ne_bytes();
        u88([a, b, c, d, e, f, g, h, i, j, k])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, _, _, _, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u88([f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 11]) -> [u8; 11] {
        [data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u88, u128, 11);
type_conversion!(u24, u128, u88);
type_conversion!(u40, u128, u88);
type_conversion!(u48, u128, u88);
type_conversion!(u56, u128, u88);
type_conversion!(u72, u128, u88);
type_conversion!(u80, u128, u88);
type_conversion!(u96, u128, u88);
type_conversion!(u104, u128, u88);
type_conversion!(u112, u128, u88);
type_conversion!(u120, u128, u88);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u96(pub(crate) [u8; 12]);

impl u96 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u96([a, b, c, d, e, f, g, h, i, j, k, l]) = self;
        u128::from_ne_bytes([0,0,0,0,a, b, c, d, e, f, g, h, i, j, k, l])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u96([a, b, c,d, e, f, g, h, i, j, k, l]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k,l,0,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, _, _, _, _] = n.to_ne_bytes();
        u96([a, b, c, d, e, f, g, h, i, j, k,l])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, _, _, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u96([e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 12]) -> [u8; 12] {
        [data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u96, u128, 12);
type_conversion!(u24, u128, u96);
type_conversion!(u40, u128, u96);
type_conversion!(u48, u128, u96);
type_conversion!(u56, u128, u96);
type_conversion!(u72, u128, u96);
type_conversion!(u80, u128, u96);
type_conversion!(u88, u128, u96);
type_conversion!(u104, u128, u96);
type_conversion!(u112, u128, u96);
type_conversion!(u120, u128, u96);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u104(pub(crate) [u8; 13]);

impl u104 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u104([a, b, c, d, e, f, g, h, i, j, k, l, m]) = self;
        u128::from_ne_bytes([0,0,0,a, b, c, d, e, f, g, h, i, j, k, l, m])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u104([a, b, c,d, e, f, g, h, i, j, k, l, m]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m,0,0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, _, _, _] = n.to_ne_bytes();
        u104([a, b, c, d, e, f, g, h, i, j, k, l, m])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, _, d, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u104([d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 13]) -> [u8; 13] {
        [data[12], data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u104, u128, 13);
type_conversion!(u24, u128, u104);
type_conversion!(u40, u128, u104);
type_conversion!(u48, u128, u104);
type_conversion!(u56, u128, u104);
type_conversion!(u72, u128, u104);
type_conversion!(u80, u128, u104);
type_conversion!(u88, u128, u104);
type_conversion!(u96, u128, u104);
type_conversion!(u112, u128, u104);
type_conversion!(u120, u128, u104);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u112(pub(crate) [u8; 14]);

impl u112 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u112([a, b, c, d, e, f, g, h, i, j, k, l, m, n]) = self;
        u128::from_ne_bytes([0,0, a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u112([a, b, c,d, e, f, g, h, i, j, k, l, m, n]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, 0,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, _, _] = n.to_ne_bytes();
        u112([a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, _, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u112([c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 14]) -> [u8; 14] {
        [data[13], data[12], data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u112, u128, 14);
type_conversion!(u24, u128, u112);
type_conversion!(u40, u128, u112);
type_conversion!(u48, u128, u112);
type_conversion!(u56, u128, u112);
type_conversion!(u72, u128, u112);
type_conversion!(u80, u128, u112);
type_conversion!(u88, u128, u112);
type_conversion!(u96, u128, u112);
type_conversion!(u104, u128, u112);
type_conversion!(u120, u128, u112);

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u120(pub(crate) [u8; 15]);

impl u120 {
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u120([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]) = self;
        u128::from_ne_bytes([0, a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u120([a, b, c,d, e, f, g, h, i, j, k, l, m, n, o]) = self;
        u128::from_ne_bytes([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o,0])
    }

    #[cfg(target_endian = "little")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, _] = n.to_ne_bytes();
        u120([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    }

    #[cfg(target_endian = "big")]
    #[inline]
    pub(crate) const fn from_num(n: u128) -> Self {
        let [_, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u112([b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 15]) -> [u8; 15] {
        [data[14], data[13], data[12], data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }
}


type_impl!(u120, u128, 15);
type_conversion!(u24, u128, u120);
type_conversion!(u40, u128, u120);
type_conversion!(u48, u128, u120);
type_conversion!(u56, u128, u120);
type_conversion!(u72, u128, u120);
type_conversion!(u80, u128, u120);
type_conversion!(u88, u128, u120);
type_conversion!(u96, u128, u120);
type_conversion!(u104, u128, u120);
type_conversion!(u112, u128, u120);