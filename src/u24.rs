use crate::{type_conversion, type_impl, u104, u112, u120, u40, u48, u56, u72, u80, u88, u96};


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

    #[allow(dead_code)]
    #[inline]
    pub(crate) const fn swap_data_copy(data: &[u8; 3]) -> [u8; 3] {
        [data[2], data[1], data[0]]
    }


}


type_impl!(u24, u32, 3);
type_conversion!(u40::u40, u32, u24);
type_conversion!(u48::u48, u32, u24);
type_conversion!(u56::u56, u32, u24);
type_conversion!(u72::u72, u32, u24);
type_conversion!(u80::u80, u32, u24);
type_conversion!(u88::u88, u32, u24);
type_conversion!(u96::u96, u32, u24);
type_conversion!(u104::u104, u32, u24);
type_conversion!(u112::u112, u32, u24);
type_conversion!(u120::u120, u32, u24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut my_vec: Vec<u24> = Vec::with_capacity(512);
        my_vec.push(u24::from(u24::MAX_VALUE-10));
        my_vec.push(u24::from(4u32));
        my_vec.push(u24::from(4u32));
        println!("{}", my_vec[0]);
        println!("{}", my_vec[1]);
        let z2 = unsafe{my_vec[0].fetch_unsafe()};

        let n = unsafe {my_vec[1].unsafe_add_with_aligned_into_aligned(6u32)};
        println!("{:?}", n);
        println!("{:?}", u24::MAX_VALUE-n);


    }
}