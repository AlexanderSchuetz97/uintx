#![cfg_attr(not(feature = "std"), no_std)]
mod type_macro;

use crate::type_macro::{*};


#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u24(pub(crate) [u8; 3]);

impl u24 {

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u32 {
        let crate::u24([a, b, c]) = self;
        u32::from_ne_bytes([0, a, b, c])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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


    #[inline]
    pub const fn as_i8_array(self) -> [i8; 3] {
        let u24([a, b, c]) = self;
        return [
            a as i8,
            b as i8,
            c as i8
        ];
    }

}

impl From<[i8; 3]> for u24 {
    fn from(value: [i8; 3]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8
        ]);
    }
}

impl Into<[i8; 3]> for u24 {
    fn into(self) -> [i8; 3] {
        return self.as_i8_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u40(pub(crate) [u8; 5]);

impl u40 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u40([a, b, c, d,e]) = self;
        u64::from_ne_bytes([0,0,0, a, b, c, d, e])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 5] {
        let u40([a, b, c, d,e]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8
        ];
    }
}

impl From<[i8; 5]> for u40 {
    fn from(value: [i8; 5]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
        ]);
    }
}

impl Into<[i8; 5]> for u40 {
    fn into(self) -> [i8; 5] {
        return self.as_i8_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u48(pub(crate) [u8; 6]);

impl u48 {

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u48([a, b, c, d, e, f]) = self;
        u64::from_ne_bytes([0,0, a, b, c, d, e, f])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 6] {
        let u48([a, b, c,d,e, f]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8
        ];
    }

    #[inline]
    pub const fn as_u16_array(self) -> [u16; 3]{
        let u48([a, b, c,d,e, f]) = self;
        return [
            u16::from_ne_bytes([a, b]),
            u16::from_ne_bytes([c, d]),
            u16::from_ne_bytes([e, f]),
        ];
    }

    #[inline]
    pub const fn as_i16_array(self) -> [i16; 3] {
        let u48([a, b, c,d,e, f]) = self;
        return [
            i16::from_ne_bytes([a, b]),
            i16::from_ne_bytes([c, d]),
            i16::from_ne_bytes([e, f]),
        ];
    }

    #[inline]
    pub const fn as_u24_array(self) -> [u24; 2] {
        let u48([a, b, c,d,e, f]) = self;
        return [
            u24::from_ne_bytes([a, b,c]),
            u24::from_ne_bytes([d,e,f])
        ];
    }

    #[cfg(feature = "half_support")]
    pub const fn as_f16_array(self) -> [half::f16; 3] {
        let u48([a, b, c,d,e, f]) = self;
        return [
            half::f16::from_ne_bytes([a, b]),
            half::f16::from_ne_bytes([c, d]),
            half::f16::from_ne_bytes([e, f]),
        ];
    }
}

impl From<[i8; 6]> for u48 {
    fn from(value: [i8; 6]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
        ]);
    }
}

impl Into<[i8; 6]> for u48 {
    fn into(self) -> [i8; 6] {
        return self.as_i8_array();
    }
}

impl From<[u16; 3]> for u48 {
    fn from(value: [u16; 3]) -> Self {
        let n : [[u8; 2]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
        ]);
    }
}

impl Into<[u16; 3]> for u48 {
    fn into(self) -> [u16; 3] {
        return self.as_u16_array();
    }
}

impl From<[i16; 3]> for u48 {
    fn from(value: [i16; 3]) -> Self {
        let n : [[u8; 2]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
        ]);
    }
}

impl Into<[i16; 3]> for u48 {
    fn into(self) -> [i16; 3] {
        return self.as_i16_array();
    }
}

#[cfg(feature = "half_support")]
impl From<[half::f16; 3]> for u48 {
    fn from(value: [half::f16; 3]) -> Self {
        let n : [[u8; 2]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
        ]);
    }
}

#[cfg(feature = "half_support")]
impl Into<[half::f16; 3]> for u48 {
    fn into(self) -> [half::f16; 3] {
        return self.as_f16_array();
    }
}

impl From<[u24; 2]> for u48 {
    fn from(value: [u24; 2]) -> Self {
        let n : [[u8; 3]; 2] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[1][0],
            n[1][1],
            n[1][2],
        ]);
    }
}

impl Into<[u24; 2]> for u48 {
    fn into(self) -> [u24; 2] {
        return self.as_u24_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u56(pub(crate) [u8; 7]);

impl u56 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u64 {
        let u56([a, b, c, d, e, f, g]) = self;
        u64::from_ne_bytes([0,a, b, c, d, e, f, g])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 7] {
        let u56([a, b, c,d,e, f, g]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8
        ];
    }
}

impl From<[i8; 7]> for u56 {
    fn from(value: [i8; 7]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
        ]);
    }
}

impl Into<[i8; 7]> for u56 {
    fn into(self) -> [i8; 7] {
        return self.as_i8_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u72(pub(crate) [u8; 9]);

impl u72 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u72([a, b, c, d, e, f, g, h, i]) = self;
        u128::from_ne_bytes([0,0,0,0,0,0,0, a, b, c, d, e, f, g, h, i])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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
        let [_, _, _, _, _, _, _, h, i, j, k, l, m, n, o, p] = n.to_ne_bytes();
        u72([h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 9]) -> [u8; 9] {
        [data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 9] {
        let u72([a, b, c,d,e, f, g, h, i]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8
        ];
    }

    #[inline]
    pub const fn as_u24_array(self) -> [u24; 3] {
        let u72([a, b, c,d,e, f, g, h, i]) = self;
        return [
            u24::from_ne_bytes([a,b,c]),
            u24::from_ne_bytes([d,e,f]),
            u24::from_ne_bytes([g,h,i]),
        ];
    }
}

impl From<[i8; 9]> for u72 {
    fn from(value: [i8; 9]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
        ]);
    }
}

impl Into<[i8; 9]> for u72 {
    fn into(self) -> [i8; 9] {
        return self.as_i8_array();
    }
}

impl From<[u24; 3]> for u72 {
    fn from(value: [u24; 3]) -> Self {
        let n : [[u8; 3]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[1][0],
            n[1][1],
            n[1][2],
            n[2][0],
            n[2][1],
            n[2][2],
        ]);
    }
}

impl Into<[u24; 3]> for u72 {
    fn into(self) -> [u24; 3] {
        return self.as_u24_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u80(pub(crate) [u8; 10]);

impl u80 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u80([a, b, c, d, e, f, g, h, i, j]) = self;
        u128::from_ne_bytes([0,0,0,0,0,0,a, b, c, d, e, f, g, h, i, j])
    }
    ///
    /// Unwraps the type into the next largest aligned type.
    ///

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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 10] {
        let u80([a, b, c,d,e, f, g, h, i, j]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8
        ];
    }

    #[inline]
    pub const fn as_u16_array(self) -> [u16; 5] {
        let u80([a, b, c,d,e, f, g, h, i, j]) = self;
        return [
            u16::from_ne_bytes([a,b]),
            u16::from_ne_bytes([c,d]),
            u16::from_ne_bytes([e,f]),
            u16::from_ne_bytes([g,h]),
            u16::from_ne_bytes([i,j]),
        ];
    }

    #[inline]
    pub const fn as_i16_array(self) -> [i16; 5] {
        let u80([a, b, c,d,e, f, g, h, i, j]) = self;
        return [
            i16::from_ne_bytes([a,b]),
            i16::from_ne_bytes([c,d]),
            i16::from_ne_bytes([e,f]),
            i16::from_ne_bytes([g,h]),
            i16::from_ne_bytes([i,j]),
        ];
    }

    #[cfg(feature = "half_support")]
    #[inline]
    pub const fn as_f16_array(self) -> [half::f16; 5] {
        let u80([a, b, c,d,e, f, g, h, i, j]) = self;
        return [
            half::f16::from_ne_bytes([a,b]),
            half::f16::from_ne_bytes([c,d]),
            half::f16::from_ne_bytes([e,f]),
            half::f16::from_ne_bytes([g,h]),
            half::f16::from_ne_bytes([i,j]),
        ];
    }

    #[inline]
    pub const fn as_u40_array(self) -> [u40; 2] {
        let u80([a, b, c,d,e, f, g, h, i, j]) = self;
        return [
            u40::from_ne_bytes([a,b,c,d,e]),
            u40::from_ne_bytes([f,g,h,i,j]),
        ];
    }
}

impl From<[i8; 10]> for u80 {
    fn from(value: [i8; 10]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
        ]);
    }
}

impl Into<[i8; 10]> for u80 {
    fn into(self) -> [i8; 10] {
        return self.as_i8_array();
    }
}

impl From<[u16; 5]> for u80 {
    fn from(value: [u16; 5]) -> Self {
        let n : [[u8; 2]; 5] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
        ]);
    }
}

impl Into<[u16; 5]> for u80 {
    fn into(self) -> [u16; 5] {
        return self.as_u16_array();
    }
}

impl From<[i16; 5]> for u80 {
    fn from(value: [i16; 5]) -> Self {
        let n : [[u8; 2]; 5] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
        ]);
    }
}

impl Into<[i16; 5]> for u80 {
    fn into(self) -> [i16; 5] {
        return self.as_i16_array();
    }
}

#[cfg(feature = "half_support")]
impl From<[half::f16; 5]> for u80 {
    fn from(value: [half::f16; 5]) -> Self {
        let n : [[u8; 2]; 5] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
        ]);
    }
}

#[cfg(feature = "half_support")]
impl Into<[half::f16; 5]> for u80 {
    fn into(self) -> [half::f16; 5] {
        return self.as_f16_array();
    }
}

impl From<[u40; 2]> for u80 {
    fn from(value: [u40; 2]) -> Self {
        let n : [[u8; 5]; 2] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[0][4],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[1][4],
        ]);
    }
}

impl Into<[u40; 2]> for u80 {
    fn into(self) -> [u40; 2] {
        return self.as_u40_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u88(pub(crate) [u8; 11]);

impl u88 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u88([a, b, c, d, e, f, g, h, i, j, k]) = self;
        u128::from_ne_bytes([0,0,0,0,0,a, b, c, d, e, f, g, h, i, j, k])
    }
    ///
    /// Unwraps the type into the next largest aligned type.
    ///

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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 11] {
        let u88([a, b, c,d,e, f, g, h, i, j, k]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8,
            k as i8,
        ];
    }

}

impl From<[i8; 11]> for u88 {
    fn from(value: [i8; 11]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
            value[10] as u8,
        ]);
    }
}

impl Into<[i8; 11]> for u88 {
    fn into(self) -> [i8; 11] {
        return self.as_i8_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u96(pub(crate) [u8; 12]);

impl u96 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u96([a, b, c, d, e, f, g, h, i, j, k, l]) = self;
        u128::from_ne_bytes([0,0,0,0,a, b, c, d, e, f, g, h, i, j, k, l])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 12] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8,
            k as i8,
            l as i8,
        ];
    }

    #[inline]
    pub const fn as_u16_array(self) -> [u16; 6] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            u16::from_ne_bytes([a,b]),
            u16::from_ne_bytes([c,d]),
            u16::from_ne_bytes([e,f]),
            u16::from_ne_bytes([g,h]),
            u16::from_ne_bytes([i,j]),
            u16::from_ne_bytes([k,l]),
        ];
    }

    #[inline]
    pub const fn as_i16_array(self) -> [i16; 6] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            i16::from_ne_bytes([a,b]),
            i16::from_ne_bytes([c,d]),
            i16::from_ne_bytes([e,f]),
            i16::from_ne_bytes([g,h]),
            i16::from_ne_bytes([i,j]),
            i16::from_ne_bytes([k,l]),
        ];
    }

    #[cfg(feature = "half_support")]
    #[inline]
    pub const fn as_f16_array(self) -> [half::f16; 6] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            half::f16::from_ne_bytes([a,b]),
            half::f16::from_ne_bytes([c,d]),
            half::f16::from_ne_bytes([e,f]),
            half::f16::from_ne_bytes([g,h]),
            half::f16::from_ne_bytes([i,j]),
            half::f16::from_ne_bytes([k,l]),
        ];
    }

    #[inline]
    pub const fn as_u24_array(self) -> [u24; 4] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            u24::from_ne_bytes([a,b,c]),
            u24::from_ne_bytes([d,e,f]),
            u24::from_ne_bytes([g,h,i]),
            u24::from_ne_bytes([j,k,l]),
        ];
    }

    #[inline]
    pub const fn as_u32_array(self) -> [u32; 3] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            u32::from_ne_bytes([a,b,c,d]),
            u32::from_ne_bytes([e,f,g,h]),
            u32::from_ne_bytes([i,j,k,l]),
        ];
    }

    #[inline]
    pub const fn as_i32_array(self) -> [i32; 3] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            i32::from_ne_bytes([a,b,c,d]),
            i32::from_ne_bytes([e,f,g,h]),
            i32::from_ne_bytes([i,j,k,l]),
        ];
    }

    #[inline]
    pub fn as_f32_array(self) -> [f32; 3] {
        //const not stable
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            f32::from_bits(u32::from_ne_bytes([a,b,c,d])),
            f32::from_bits(u32::from_ne_bytes([e,f,g,h])),
            f32::from_bits(u32::from_ne_bytes([i,j,k,l])),
        ];
    }

    #[inline]
    pub const fn as_u48_array(self) -> [u48; 2] {
        let u96([a, b, c,d,e, f, g, h, i, j, k, l]) = self;
        return [
            u48::from_ne_bytes([a,b,c,d,e,f]),
            u48::from_ne_bytes([g,h,i,j,k,l]),
        ];
    }
}

impl From<[i8; 12]> for u96 {
    fn from(value: [i8; 12]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
            value[10] as u8,
            value[11] as u8,
        ]);
    }
}

impl Into<[i8; 12]> for u96 {
    fn into(self) -> [i8; 12] {
        return self.as_i8_array();
    }
}

impl From<[u16; 6]> for u96 {
    fn from(value: [u16; 6]) -> Self {
        let n : [[u8; 2]; 6] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
        ]);
    }
}

impl Into<[u16; 6]> for u96 {
    fn into(self) -> [u16; 6] {
        return self.as_u16_array();
    }
}

impl From<[i16; 6]> for u96 {
    fn from(value: [i16; 6]) -> Self {
        let n : [[u8; 2]; 6] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
        ]);
    }
}

impl Into<[i16; 6]> for u96 {
    fn into(self) -> [i16; 6] {
        return self.as_i16_array();
    }
}
#[cfg(feature = "half_support")]
impl From<[half::f16; 6]> for u96 {
    fn from(value: [half::f16; 6]) -> Self {
        let n : [[u8; 2]; 6] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
        ]);
    }
}

#[cfg(feature = "half_support")]
impl Into<[half::f16; 6]> for u96 {
    fn into(self) -> [half::f16; 6] {
        return self.as_f16_array();
    }
}

impl From<[u24; 4]> for u96 {
    fn from(value: [u24; 4]) -> Self {
        let n : [[u8; 3]; 4] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[1][0],
            n[1][1],
            n[1][2],
            n[2][0],
            n[2][1],
            n[2][2],
            n[3][0],
            n[3][1],
            n[3][2],
        ]);
    }
}

impl Into<[u24; 4]> for u96 {
    fn into(self) -> [u24; 4] {
        return self.as_u24_array();
    }
}

impl From<[u32; 3]> for u96 {
    fn from(value: [u32; 3]) -> Self {
        let n : [[u8; 4]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[2][0],
            n[2][1],
            n[2][2],
            n[2][3],
        ]);
    }
}

impl Into<[u32; 3]> for u96 {
    fn into(self) -> [u32; 3] {
        return self.as_u32_array();
    }
}

impl From<[i32; 3]> for u96 {
    fn from(value: [i32; 3]) -> Self {
        let n : [[u8; 4]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[2][0],
            n[2][1],
            n[2][2],
            n[2][3],
        ]);
    }
}

impl Into<[i32; 3]> for u96 {
    fn into(self) -> [i32; 3] {
        return self.as_i32_array();
    }
}

impl From<[f32; 3]> for u96 {
    fn from(value: [f32; 3]) -> Self {
        let n : [[u8; 4]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[2][0],
            n[2][1],
            n[2][2],
            n[2][3],
        ]);
    }
}

impl Into<[f32; 3]> for u96 {
    fn into(self) -> [f32; 3] {
        return self.as_f32_array();
    }
}

impl From<[u48; 2]> for u96 {
    fn from(value: [u48; 2]) -> Self {
        let n : [[u8; 6]; 2] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[0][4],
            n[0][5],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[1][4],
            n[1][5],
        ]);
    }
}

impl Into<[u48; 2]> for u96 {
    fn into(self) -> [u48; 2] {
        return self.as_u48_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u104(pub(crate) [u8; 13]);

impl u104 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u104([a, b, c, d, e, f, g, h, i, j, k, l, m]) = self;
        u128::from_ne_bytes([0,0,0,a, b, c, d, e, f, g, h, i, j, k, l, m])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 13] {
        let u104([a, b, c,d,e, f, g, h, i, j, k, l, m]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8,
            k as i8,
            l as i8,
            m as i8
        ];
    }

}

impl From<[i8; 13]> for u104 {
    fn from(value: [i8; 13]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
            value[10] as u8,
            value[11] as u8,
            value[12] as u8,
        ]);
    }
}

impl Into<[i8; 13]> for u104 {
    fn into(self) -> [i8; 13] {
        return self.as_i8_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u112(pub(crate) [u8; 14]);

impl u112 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u112([a, b, c, d, e, f, g, h, i, j, k, l, m, n]) = self;
        u128::from_ne_bytes([0,0, a, b, c, d, e, f, g, h, i, j, k, l, m, n])
    }
    ///
    /// Unwraps the type into the next largest aligned type.
    ///

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

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 14] {
        let u112([a, b, c,d,e, f, g, h, i, j, k, l,m,n]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8,
            k as i8,
            l as i8,
            m as i8,
            n as i8
        ];
    }

    #[inline]
    pub const fn as_u16_array(self) -> [u16; 7] {
        let u112([a, b, c,d,e, f, g, h, i, j, k, l,m,n]) = self;
        return [
            u16::from_ne_bytes([a,b]),
            u16::from_ne_bytes([c,d]),
            u16::from_ne_bytes([e,f]),
            u16::from_ne_bytes([g,h]),
            u16::from_ne_bytes([i,j]),
            u16::from_ne_bytes([k,l]),
            u16::from_ne_bytes([m,n]),
        ];
    }

    #[inline]
    pub const fn as_i16_array(self) -> [i16; 7] {
        let u112([a, b, c,d,e, f, g, h, i, j, k, l,m,n]) = self;
        return [
            i16::from_ne_bytes([a,b]),
            i16::from_ne_bytes([c,d]),
            i16::from_ne_bytes([e,f]),
            i16::from_ne_bytes([g,h]),
            i16::from_ne_bytes([i,j]),
            i16::from_ne_bytes([k,l]),
            i16::from_ne_bytes([m,n]),
        ];
    }

    #[cfg(feature = "half_support")]
    #[inline]
    pub const fn as_f16_array(self) -> [half::f16; 7] {
        let u112([a, b, c,d,e, f, g, h, i, j, k, l,m,n]) = self;
        return [
            half::f16::from_ne_bytes([a,b]),
            half::f16::from_ne_bytes([c,d]),
            half::f16::from_ne_bytes([e,f]),
            half::f16::from_ne_bytes([g,h]),
            half::f16::from_ne_bytes([i,j]),
            half::f16::from_ne_bytes([k,l]),
            half::f16::from_ne_bytes([m,n]),
        ];
    }

    #[inline]
    pub const fn as_u56_array(self) -> [u56; 2] {
        let u112([a, b, c,d,e, f, g, h, i, j, k, l,m,n]) = self;
        return [
            u56::from_ne_bytes([a,b,c,d,e,f,g]),
            u56::from_ne_bytes([h,i,j,k,l,m,n])
        ];
    }
}

impl From<[i8; 14]> for u112 {
    fn from(value: [i8; 14]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
            value[10] as u8,
            value[11] as u8,
            value[12] as u8,
            value[13] as u8,
        ]);
    }
}

impl Into<[i8; 14]> for u112 {
    fn into(self) -> [i8; 14] {
        return self.as_i8_array();
    }
}

impl From<[u16; 7]> for u112 {
    fn from(value: [u16; 7]) -> Self {
        let n : [[u8; 2]; 7] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
            value[6].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
            n[6][0],
            n[6][1],
        ]);
    }
}

impl Into<[u16; 7]> for u112 {
    fn into(self) -> [u16; 7] {
        return self.as_u16_array();
    }
}

impl From<[i16; 7]> for u112 {
    fn from(value: [i16; 7]) -> Self {
        let n : [[u8; 2]; 7] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
            value[6].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
            n[6][0],
            n[6][1],
        ]);
    }
}

impl Into<[i16; 7]> for u112 {
    fn into(self) -> [i16; 7] {
        return self.as_i16_array();
    }
}

#[cfg(feature = "half_support")]
impl From<[half::f16; 7]> for u112 {
    fn from(value: [half::f16; 7]) -> Self {
        let n : [[u8; 2]; 7] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
            value[5].to_ne_bytes(),
            value[6].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[1][0],
            n[1][1],
            n[2][0],
            n[2][1],
            n[3][0],
            n[3][1],
            n[4][0],
            n[4][1],
            n[5][0],
            n[5][1],
            n[6][0],
            n[6][1],
        ]);
    }
}


#[cfg(feature = "half_support")]
impl Into<[half::f16; 7]> for u112 {
    fn into(self) -> [half::f16; 7] {
        return self.as_f16_array();
    }
}

impl From<[u56; 2]> for u112 {
    fn from(value: [u56; 2]) -> Self {
        let n : [[u8; 7]; 2] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
        ];
        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[0][4],
            n[0][5],
            n[0][6],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[1][4],
            n[1][5],
            n[1][6],
        ]);
    }
}

impl Into<[u56; 2]> for u112 {
    fn into(self) -> [u56; 2] {
        return self.as_u56_array();
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

#[derive(Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[repr(transparent)]
pub struct u120(pub(crate) [u8; 15]);

impl u120 {
    ///
    /// Unwraps the type into the next largest aligned type.
    ///
    #[cfg(target_endian = "big")]
    #[inline]
    pub const fn as_num(self) -> u128 {
        let u120([a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]) = self;
        u128::from_ne_bytes([0, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o])
    }

    ///
    /// Unwraps the type into the next largest aligned type.
    ///
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
        u120([b, c, d, e, f, g, h, i, j, k, l, m, n, o, p])
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 15]) -> [u8; 15] {
        [data[14], data[13], data[12], data[11], data[10], data[9], data[8], data[7], data[6], data[5], data[4], data[3], data[2], data[1], data[0]]
    }

    #[inline]
    pub const fn as_i8_array(self) -> [i8; 15] {
        let u120([a, b, c,d,e, f, g, h, i, j, k, l,m,n, o]) = self;
        return [
            a as i8,
            b as i8,
            c as i8,
            d as i8,
            e as i8,
            f as i8,
            g as i8,
            h as i8,
            i as i8,
            j as i8,
            k as i8,
            l as i8,
            m as i8,
            n as i8,
            o as i8
        ];
    }

    #[inline]
    pub const fn as_u24_array(self) -> [u24; 5] {
        let u120([a, b, c,d,e, f, g, h, i, j, k, l,m,n, o]) = self;
        return [
            u24::from_ne_bytes([a,b,c]),
            u24::from_ne_bytes([d,e,f]),
            u24::from_ne_bytes([g,h,i]),
            u24::from_ne_bytes([j,k,l]),
            u24::from_ne_bytes([m,n,o]),
        ];
    }

    #[inline]
    pub const fn as_u40_array(self) -> [u40; 3] {
        let u120([a, b, c,d,e, f, g, h, i, j, k, l,m,n, o]) = self;
        return [
            u40::from_ne_bytes([a,b,c,d,e]),
            u40::from_ne_bytes([f,g,h,i,j]),
            u40::from_ne_bytes([k,l,m,n,o]),
        ];
    }
}

impl From<[i8; 15]> for u120 {
    fn from(value: [i8; 15]) -> Self {
        return Self([
            value[0] as u8,
            value[1] as u8,
            value[2] as u8,
            value[3] as u8,
            value[4] as u8,
            value[5] as u8,
            value[6] as u8,
            value[7] as u8,
            value[8] as u8,
            value[9] as u8,
            value[10] as u8,
            value[11] as u8,
            value[12] as u8,
            value[13] as u8,
            value[14] as u8,
        ]);
    }
}

impl Into<[i8; 15]> for u120 {
    fn into(self) -> [i8; 15] {
        return self.as_i8_array();
    }
}

impl From<[u24; 5]> for u120 {
    fn from(value: [u24; 5]) -> Self {
        let n : [[u8; 3]; 5] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
            value[3].to_ne_bytes(),
            value[4].to_ne_bytes(),
        ];

        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[1][0],
            n[1][1],
            n[1][2],
            n[2][0],
            n[2][1],
            n[2][2],
            n[3][0],
            n[3][1],
            n[3][2],
            n[4][0],
            n[4][1],
            n[4][2],
        ]);
    }
}

impl Into<[u24; 5]> for u120 {
    fn into(self) -> [u24; 5] {
        return self.as_u24_array();
    }
}

impl From<[u40; 3]> for u120 {
    fn from(value: [u40; 3]) -> Self {
        let n : [[u8; 5]; 3] = [
            value[0].to_ne_bytes(),
            value[1].to_ne_bytes(),
            value[2].to_ne_bytes(),
        ];

        return Self([
            n[0][0],
            n[0][1],
            n[0][2],
            n[0][3],
            n[0][4],
            n[1][0],
            n[1][1],
            n[1][2],
            n[1][3],
            n[1][4],
            n[2][0],
            n[2][1],
            n[2][2],
            n[2][3],
            n[2][4],
        ]);
    }
}

impl Into<[u40; 3]> for u120 {
    fn into(self) -> [u40; 3] {
        return self.as_u40_array();
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


