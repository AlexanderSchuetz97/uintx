use crate::{type_conversion, type_impl, u104, u112, u120, u24, u40, u48, u72, u80, u88, u96};


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
type_conversion!(u24::u24, u64, u56);
type_conversion!(u40::u40, u64, u56);
type_conversion!(u48::u48, u64, u56);
type_conversion!(u72::u72, u64, u56);
type_conversion!(u80::u80, u64, u56);
type_conversion!(u88::u88, u64, u56);
type_conversion!(u96::u96, u64, u56);
type_conversion!(u104::u104, u64, u56);
type_conversion!(u112::u112, u64, u56);
type_conversion!(u120::u120, u64, u56);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_vec: Vec<u56> = Vec::with_capacity(512);
        my_vec.push(u56::from(u56::MAX_VALUE-10));
        my_vec.push(u56::from(4u32));
        my_vec.push(u56::from(4u32));
        println!("{}", my_vec[0]);
        println!("{}", my_vec[1]);
        let z2 = unsafe{my_vec[0].fetch_unsafe()};

        let n = unsafe {my_vec[0].unsafe_add_with_aligned_into_aligned(6u64)};
        println!("{:?}", n);
        println!("{:?}", u56::MAX_VALUE-n);


    }
}