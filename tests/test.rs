use getrandom::getrandom;
use std::mem::size_of;
use uintx::u104;
use uintx::u112;
use uintx::u120;
use uintx::u24;
use uintx::u40;
use uintx::u48;
use uintx::u56;
use uintx::u72;
use uintx::u80;
use uintx::u88;
use uintx::u96;

const TEST_SET_SIZE: usize = if cfg!(miri) { 0xF } else if cfg!(target_endian = "big") { 0xF} else { 0xFFFF };

macro_rules! test_type {
    ($under_test:ty, $tt:ident) => {
        struct $tt {
            random_numbers: Vec<$under_test>,
        }

        impl $tt {
            fn run() {
                let tdata = Self::make_test_data();
                for nr in &tdata.random_numbers {
                    Self::rem(nr);
                    Self::shift(nr);
                    Self::and_or_xor(nr);
                    Self::add_sub(nr);
                    Self::mul_div(nr);
                }
                Self::unsafe_fetch(&tdata.random_numbers);
            }

            fn make_test_data() -> $tt {
                let mut dta = Vec::new();
                for _ in 0..TEST_SET_SIZE {
                    let mut inner: Vec<u8> = vec![0; size_of::<$under_test>()];
                    getrandom(inner.as_mut_slice()).expect("GETRANDOM");
                    let copy: $under_test =
                        unsafe { (*inner.as_ptr().cast::<$under_test>()).clone() };
                    dta.push(copy);
                }
                dta.push(<$under_test>::MAX);
                dta.push(<$under_test>::MIN);

                return $tt {
                    random_numbers: dta,
                };
            }

            fn add_sub(num: &$under_test) {
                let base = num.clone();
                let bnum = num.as_num();
                if bnum == 0 || bnum >= <$under_test>::MAX_VALUE - 32 {
                    return;
                }

                let u = base + 1;
                let b = base + <$under_test>::from(1);
                assert_eq!(u, b);
                assert_ne!(base, b);
                assert_eq!(<$under_test>::from(bnum + 1), b);
                let x = base - 1;
                let y = base - <$under_test>::from(1);
                assert_eq!(x, y);
                assert_ne!(base, x);
                assert_eq!(<$under_test>::from(bnum - 1), x);
                assert_eq!(x + 2, u);
                assert_eq!(y + <$under_test>::from(2), b);
                let mut z = base;
                z += <$under_test>::from(16);
                assert_eq!(z, <$under_test>::from(bnum + 16));
                z += 16;
                assert_eq!(z, <$under_test>::from(bnum + 32));
                z -= 16;
                assert_eq!(z, <$under_test>::from(bnum + 16));
                z -= <$under_test>::from(16);
                assert_eq!(z, base);
            }

            fn mul_div(num: &$under_test) {
                let mut base = num.clone();
                base -= base % 3;
                if base >= <$under_test>::MAX_VALUE / 3 {
                    return;
                }
                if base == 0 {
                    return;
                }

                let u = base * 3;
                let b = base * <$under_test>::from(3);
                assert_eq!(u, b);
                assert_ne!(base, b);
                let x = base / 3;
                let y = base / <$under_test>::from(3);
                assert_eq!(x, y);
                assert_ne!(base, x);
                assert_eq!(x * 9, u, "{} - {} - {}", base, x, u);
                assert_eq!(y * <$under_test>::from(9), b);
                let mut z = base;
                z *= 3;
                assert_eq!(z, u);
                z /= 3;
                assert_eq!(z, base);
            }

            fn rem(num: &$under_test) {
                let base = num.clone();
                let bnum = num.as_num();

                let div_base = (<$under_test>::MAX_VALUE - <$under_test>::MAX_VALUE) + 41;

                for idx in 0..(size_of::<$under_test>() * 8) {
                    let div = div_base << idx;
                    if div >= <$under_test>::MAX_VALUE {
                        return;
                    }
                    let mut z = base;
                    let mut z2 = base;
                    z %= div;
                    z2 %= div;
                    assert_eq!(base % div, bnum % div, "{} - {}", bnum, div);
                    assert_eq!(
                        base % <$under_test>::from(div),
                        bnum % div,
                        "{} - {}",
                        bnum,
                        div
                    );
                    assert_eq!(z, bnum % div, "{} - {}", bnum, div);
                    assert_eq!(z2, bnum % div, "{} - {}", bnum, div);
                }
            }

            fn and_or_xor(num: &$under_test) {
                let base = num.clone();
                let bnum = num.as_num();
                let mask = 0b1010;

                for idx in 0..(size_of::<$under_test>() * 8) {
                    let base_mask = mask << idx;
                    if base_mask > <$under_test>::MAX_VALUE {
                        return;
                    }

                    let mut z = base;
                    let mut z2 = base;
                    z &= base_mask;
                    z2 &= <$under_test>::from(base_mask);
                    assert_eq!(
                        base & base_mask,
                        bnum & base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(
                        base & <$under_test>::from(base_mask),
                        bnum & base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(z, bnum & base_mask, "{} - {}", bnum, base_mask);
                    assert_eq!(z2, bnum & base_mask, "{} - {}", bnum, base_mask);

                    let mut z = base;
                    let mut z2 = base;
                    z |= base_mask;
                    z2 |= <$under_test>::from(base_mask);
                    assert_eq!(
                        base | base_mask,
                        bnum | base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(
                        base | <$under_test>::from(base_mask),
                        bnum | base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(z, bnum | base_mask, "{} - {}", bnum, base_mask);
                    assert_eq!(z2, bnum | base_mask, "{} - {}", bnum, base_mask);

                    let mut z = base;
                    let mut z2 = base;
                    z ^= base_mask;
                    z2 ^= <$under_test>::from(base_mask);
                    assert_eq!(
                        base ^ base_mask,
                        bnum ^ base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(
                        base ^ <$under_test>::from(base_mask),
                        bnum ^ base_mask,
                        "{} - {}",
                        bnum,
                        base_mask
                    );
                    assert_eq!(z, bnum ^ base_mask, "{} - {}", bnum, base_mask);
                    assert_eq!(z2, bnum ^ base_mask, "{} - {}", bnum, base_mask);
                }
            }

            #[cfg(not(feature = "unsafe_fetch"))]
            fn unsafe_fetch(_: &Vec<$under_test>) {}

            #[cfg(feature = "unsafe_fetch")]
            fn unsafe_fetch(data: &Vec<$under_test>) {
                let mut clone = data.clone();
                clone.push(<$under_test>::default());
                let mut v: Vec<$under_test> = Vec::new();
                v.push(<$under_test>::MAX);
                v.push(<$under_test>::from(69));
                v.push(<$under_test>::MAX);

                let clone_ptr = clone.as_ptr();
                let mid_ptr = unsafe { v.as_ptr().add(1) };

                for i in 0..data.len() {
                    let the_ptr = unsafe { clone_ptr.add(i) };
                    let cp = unsafe { <$under_test>::fetch_unsafe(the_ptr) };
                    let cp2 = unsafe { <$under_test>::fetch_unsafe_clamped(the_ptr) };
                    assert_eq!(data[i].as_num(), cp2);
                    assert_eq!(cp & <$under_test>::MAX_VALUE, cp2);

                    if data[i].as_num() + 69 <= <$under_test>::MAX_VALUE {
                        let cp = unsafe {
                            <$under_test>::unsafe_add_with_aligned_into_unaligned(the_ptr, 69)
                        };
                        assert_eq!(cp, data[i] + 69);
                        let cp =
                            unsafe { <$under_test>::unsafe_add_into_unaligned(the_ptr, mid_ptr) };
                        assert_eq!(cp, data[i] + 69);
                    }
                    let cp = unsafe { <$under_test>::unsafe_add_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() + 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_add_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() + 69);

                    if data[i].as_num() > 69 {
                        let cp = unsafe {
                            <$under_test>::unsafe_sub_with_aligned_into_unaligned(the_ptr, 69)
                        };
                        assert_eq!(cp, data[i] - 69);
                        let cp =
                            unsafe { <$under_test>::unsafe_sub_into_unaligned(the_ptr, mid_ptr) };
                        assert_eq!(cp, data[i] - 69);
                        let cp =
                            unsafe { <$under_test>::unsafe_sub_into_aligned(the_ptr, mid_ptr) };
                        assert_eq!(cp, data[i].as_num() - 69);
                        let cp = unsafe {
                            <$under_test>::unsafe_sub_with_aligned_into_aligned(the_ptr, 69)
                        };
                        assert_eq!(cp, data[i].as_num() - 69);
                    }

                    if data[i].as_num() * 69 <= <$under_test>::MAX_VALUE {
                        let cp = unsafe {
                            <$under_test>::unsafe_mul_with_aligned_into_unaligned(the_ptr, 69)
                        };
                        assert_eq!(cp, data[i] * 69);
                        let cp =
                            unsafe { <$under_test>::unsafe_mul_into_unaligned(the_ptr, mid_ptr) };
                        assert_eq!(cp, data[i] * 69);
                    }
                    let cp = unsafe { <$under_test>::unsafe_mul_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() * 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_mul_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() * 69);

                    let cp = unsafe {
                        <$under_test>::unsafe_div_with_aligned_into_unaligned(the_ptr, 69)
                    };
                    assert_eq!(cp, data[i] / 69);
                    let cp = unsafe { <$under_test>::unsafe_div_into_unaligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i] / 69);
                    let cp = unsafe { <$under_test>::unsafe_div_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() / 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_div_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() / 69);

                    let cp = unsafe {
                        <$under_test>::unsafe_rem_with_aligned_into_unaligned(the_ptr, 69)
                    };
                    assert_eq!(cp, data[i] % 69, "{}", data[i]);
                    let cp = unsafe { <$under_test>::unsafe_rem_into_unaligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i] % 69);
                    let cp = unsafe { <$under_test>::unsafe_rem_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() % 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_rem_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() % 69);

                    let cp = unsafe {
                        <$under_test>::unsafe_and_with_aligned_into_unaligned(the_ptr, 69)
                    };
                    assert_eq!(cp, data[i] & 69, "{}", data[i]);
                    let cp = unsafe { <$under_test>::unsafe_and_into_unaligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i] & 69);
                    let cp = unsafe { <$under_test>::unsafe_and_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() & 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_and_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() & 69);

                    let cp = unsafe {
                        <$under_test>::unsafe_or_with_aligned_into_unaligned(the_ptr, 69)
                    };
                    assert_eq!(cp, data[i] | 69, "{}", data[i]);
                    let cp = unsafe { <$under_test>::unsafe_or_into_unaligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i] | 69);
                    let cp = unsafe { <$under_test>::unsafe_or_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() | 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_or_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() | 69);

                    let cp = unsafe {
                        <$under_test>::unsafe_xor_with_aligned_into_unaligned(the_ptr, 69)
                    };
                    assert_eq!(cp, data[i] ^ 69, "{}", data[i]);
                    let cp = unsafe { <$under_test>::unsafe_xor_into_unaligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i] ^ 69);
                    let cp = unsafe { <$under_test>::unsafe_xor_into_aligned(the_ptr, mid_ptr) };
                    assert_eq!(cp, data[i].as_num() ^ 69);
                    let cp =
                        unsafe { <$under_test>::unsafe_xor_with_aligned_into_aligned(the_ptr, 69) };
                    assert_eq!(cp, data[i].as_num() ^ 69);
                }
            }

            pub fn shift(num: &$under_test) {
                let base = num.clone();
                let bnum = num.as_num();

                for idx in 0..(size_of::<$under_test>() * 8) {
                    assert_eq!(
                        base >> (idx as usize),
                        (bnum >> (idx as usize)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as u8),
                        (bnum >> (idx as u8)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as u16),
                        (bnum >> (idx as u16)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as u32),
                        (bnum >> (idx as u32)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as u64),
                        (bnum >> (idx as u64)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as u128),
                        (bnum >> (idx as u128)) & <$under_test>::MAX_VALUE
                    );
                    if idx <= i8::MAX as usize {
                        assert_eq!(
                            base >> (idx as i8),
                            (bnum >> (idx as i8)) & <$under_test>::MAX_VALUE
                        );
                    }
                    assert_eq!(
                        base >> (idx as isize),
                        (bnum >> (idx as isize)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as i16),
                        (bnum >> (idx as i16)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as i32),
                        (bnum >> (idx as i32)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as i64),
                        (bnum >> (idx as i64)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base >> (idx as i128),
                        (bnum >> (idx as i128)) & <$under_test>::MAX_VALUE
                    );

                    assert_eq!(
                        base << (idx as usize),
                        (bnum << (idx as usize)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as u8),
                        (bnum << (idx as u8)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as u16),
                        (bnum << (idx as u16)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as u32),
                        (bnum << (idx as u32)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as u64),
                        (bnum << (idx as u64)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as u128),
                        (bnum << (idx as u128)) & <$under_test>::MAX_VALUE
                    );
                    if idx <= i8::MAX as usize {
                        assert_eq!(
                            base << (idx as i8),
                            (bnum << (idx as i8)) & <$under_test>::MAX_VALUE
                        );
                    }
                    assert_eq!(
                        base << (idx as isize),
                        (bnum << (idx as isize)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as i16),
                        (bnum << (idx as i16)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as i32),
                        (bnum << (idx as i32)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as i64),
                        (bnum << (idx as i64)) & <$under_test>::MAX_VALUE
                    );
                    assert_eq!(
                        base << (idx as i128),
                        (bnum << (idx as i128)) & <$under_test>::MAX_VALUE
                    );

                    let mut z = base;
                    z <<= idx as usize;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as u8;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as u16;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as u32;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as u64;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as u128;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as isize;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    if idx <= i8::MAX as usize {
                        let mut z = base;
                        z <<= idx as i8;
                        assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    }
                    let mut z = base;
                    z <<= idx as i16;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as i32;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as i64;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z <<= idx as i128;
                    assert_eq!(z, (bnum << idx) & <$under_test>::MAX_VALUE);

                    let mut z = base;
                    z >>= idx as usize;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as u8;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as u16;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as u32;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as u64;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as u128;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as isize;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    if idx <= i8::MAX as usize {
                        let mut z = base;
                        z >>= idx as i8;
                        assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    }
                    let mut z = base;
                    z >>= idx as i16;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as i32;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as i64;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                    let mut z = base;
                    z >>= idx as i128;
                    assert_eq!(z, (bnum >> idx) & <$under_test>::MAX_VALUE);
                }
            }
        }
    };
}

#[test]
fn test() {
    let mut num : u24 = u24::from(12u32);
    num += 1u32;
    num = num + 1u32;
    num += u24::from(4);
    num = num + u24::from(4);

    assert_eq!(num, 22)
}

test_type!(u24, U24T);
#[test]
fn test_u24() {
    U24T::run();
}

test_type!(u40, U40T);
#[test]
fn test_u40() {
    U40T::run();
}

test_type!(u48, U48T);
#[test]
fn test_u48() {
    U48T::run();
}

test_type!(u56, U56T);
#[test]
fn test_u56() {
    U56T::run();
}

test_type!(u72, U72T);
#[test]
fn test_u72() {
    U72T::run();
}

test_type!(u80, U80T);
#[test]
fn test_u80() {
    U80T::run();
}

test_type!(u88, U88T);
#[test]
fn test_u88() {
    U88T::run();
}

test_type!(u96, U96T);
#[test]
fn test_u96() {
    U96T::run();
}

test_type!(u104, U104T);
#[test]
fn test_u104() {
    U104T::run();
}

test_type!(u112, U112T);
#[test]
fn test_u112() {
    U112T::run();
}

test_type!(u120, U120T);
#[test]
fn test_u120() {
    U120T::run();
}
