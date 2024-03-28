use std::mem::size_of;
use getrandom::getrandom;
use uintx::u24::u24;

type UnderTest = u24;
struct TestData {
    random_numbers : Vec<UnderTest>,
}

pub fn make_test_data() -> TestData {
    let mut dta = Vec::new();
    for i in 0 .. 0xFFFF {
        let mut inner : Vec<u8> = vec![0; size_of::<UnderTest>()];
        getrandom(inner.as_mut_slice()).expect("GETRANDOM");
        let copy: UnderTest = unsafe {(*inner.as_ptr().cast::<UnderTest>()).clone()};
        dta.push(copy);
    }
    return TestData{ random_numbers: dta};
}

fn add_sub(num: &UnderTest) {
    let base = num.clone();
    let bnum = num.as_num();
    if bnum == 0 || bnum >= UnderTest::MAX_VALUE - 32 {
        return;
    }

    let u = base + 1;
    let b = base + UnderTest::from(1);
    assert_eq!(u, b);
    assert_ne!(base, b);
    assert_eq!(UnderTest::from(bnum+1), b);
    let x = base - 1;
    let y = base - UnderTest::from(1);
    assert_eq!(x, y);
    assert_ne!(base, x);
    assert_eq!(UnderTest::from(bnum-1), x);
    assert_eq!(x + 2, u);
    assert_eq!(y + UnderTest::from(2) , b);
    let mut z = base;
    z+= UnderTest::from(16);
    assert_eq!(z, UnderTest::from(bnum+16));
    z+= 16;
    assert_eq!(z, UnderTest::from(bnum+32));
    z-= 16;
    assert_eq!(z, UnderTest::from(bnum+16));
    z-= UnderTest::from(16);
    assert_eq!(z, base);
}

fn mul_div(num: &UnderTest) {
    let mut base = num.clone();
    base -= base % 3;
    if base >= UnderTest::MAX_VALUE / 3 {
        return;
    }
    if base == 0 {
        return;
    }

    let u = base * 3;
    let b = base * UnderTest::from(3);
    assert_eq!(u, b);
    assert_ne!(base, b);
    let x = base / 3;
    let y = base / UnderTest::from(3);
    assert_eq!(x, y);
    assert_ne!(base, x);
    assert_eq!(x * 9, u, "{} - {} - {}", base, x, u);
    assert_eq!(y * UnderTest::from(9) , b);
    let mut z = base;
    z*=3;
    assert_eq!(z, u);
    z/=3;
    assert_eq!(z, base);

}

fn rem(num: &UnderTest) {
    let base = num.clone();
    let bnum = num.as_num();

    let div_base = (UnderTest::MAX_VALUE-UnderTest::MAX_VALUE) + 41;

    for idx in 0 .. (size_of::<UnderTest>() * 8) {
        let div = div_base << idx;
        if div >= UnderTest::MAX_VALUE {
            return;
        }
        let mut z = base;
        let mut z2 = base;
        z%= div;
        z2 %= div;
        assert_eq!(base % div, bnum % div, "{} - {}", bnum, div);
        assert_eq!(base % UnderTest::from(div), bnum % div, "{} - {}", bnum, div);
        assert_eq!(z, bnum % div, "{} - {}", bnum, div);
        assert_eq!(z2, bnum % div, "{} - {}", bnum, div);
    }
}

fn and_or_xor(num: &UnderTest) {
    let base = num.clone();
    let bnum = num.as_num();
    let mask = 0b1010u32;

    for idx in 0 .. (size_of::<UnderTest>() * 8) {
        let base_mask = mask << idx;
        if base_mask > UnderTest::MAX_VALUE {
            return;
        }

        let mut z = base;
        let mut z2 = base;
        z&= base_mask;
        z2&= UnderTest::from(base_mask);
        assert_eq!(base & base_mask, bnum & base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(base & UnderTest::from(base_mask), bnum & base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z, bnum & base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z2, bnum & base_mask, "{} - {}", bnum, base_mask);

        let mut z = base;
        let mut z2 = base;
        z|= base_mask;
        z2|= UnderTest::from(base_mask);
        assert_eq!(base | base_mask, bnum | base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(base | UnderTest::from(base_mask), bnum | base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z, bnum | base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z2, bnum | base_mask, "{} - {}", bnum, base_mask);

        let mut z = base;
        let mut z2 = base;
        z^= base_mask;
        z2^= UnderTest::from(base_mask);
        assert_eq!(base ^ base_mask, bnum ^ base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(base ^ UnderTest::from(base_mask), bnum ^ base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z, bnum ^ base_mask, "{} - {}", bnum, base_mask);
        assert_eq!(z2, bnum ^ base_mask, "{} - {}", bnum, base_mask);
    }
}

pub fn shift(num: &UnderTest) {
    
}

#[test]
fn manual() {
    let num = u24::from(1761254);
    let x = num * 3;
    let y = num / 3;
    println!("{} - {} - {} - {}", num, x, y, y*9);
}

#[test]
fn run() {
    unsafe {
        let tdata = make_test_data();
        for nr in &tdata.random_numbers {
            rem(nr);
            and_or_xor(nr);
            add_sub(nr);
            mul_div(nr);
        }
    }
}

macro_rules! test_type {
    ($under_test:ty, $tt:ident) => {
        struct $tt {

        }

        impl $tt {
        }


    }
}