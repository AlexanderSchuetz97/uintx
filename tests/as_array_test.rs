use uintx::{u112, u48, u80, u96};

#[test]
pub fn test_u48_u16() {
    let n : u48 = u48::from(0x112233445566u64);
    let arr = n.as_u16_array();

    #[cfg(target_endian = "little")]
    {
        assert_eq!(arr[0], 0x5566);
        assert_eq!(arr[1], 0x3344);
        assert_eq!(arr[2], 0x1122);
    }

    #[cfg(target_endian = "big")]
    {
        assert_eq!(arr[2], 0x5566);
        assert_eq!(arr[1], 0x3344);
        assert_eq!(arr[0], 0x1122);
    }
}

#[test]
pub fn test_u80_u16() {
    let n : u80 = u80::from(0x112233445566778899AAu128);
    let arr = n.as_u16_array();

    #[cfg(target_endian = "little")]
    {
        assert_eq!(arr[0], 0x99AA);
        assert_eq!(arr[1], 0x7788);
        assert_eq!(arr[2], 0x5566);
        assert_eq!(arr[3], 0x3344);
        assert_eq!(arr[4], 0x1122);
    }

    #[cfg(target_endian = "big")]
    {
        assert_eq!(arr[4], 0x99AA);
        assert_eq!(arr[3], 0x7788);
        assert_eq!(arr[2], 0x5566);
        assert_eq!(arr[1], 0x3344);
        assert_eq!(arr[0], 0x1122);
    }
}

#[test]
pub fn test_u96_u16() {
    let n : u96 = u96::from(0x112233445566778899AABBCCu128);
    let arr = n.as_u16_array();

    #[cfg(target_endian = "little")]
    {
        assert_eq!(arr[0], 0xBBCC);
        assert_eq!(arr[1], 0x99AA);
        assert_eq!(arr[2], 0x7788);
        assert_eq!(arr[3], 0x5566);
        assert_eq!(arr[4], 0x3344);
        assert_eq!(arr[5], 0x1122);
    }

    #[cfg(target_endian = "big")]
    {
        assert_eq!(arr[5], 0xBBCC);
        assert_eq!(arr[4], 0x99AA);
        assert_eq!(arr[3], 0x7788);
        assert_eq!(arr[2], 0x5566);
        assert_eq!(arr[1], 0x3344);
        assert_eq!(arr[0], 0x1122);
    }
}

#[test]
pub fn test_u112_u16() {
    let n : u112 = u112::from(0x112233445566778899AABBCCEEFFu128);
    let arr = n.as_u16_array();

    #[cfg(target_endian = "little")]
    {
        assert_eq!(arr[0], 0xEEFF);
        assert_eq!(arr[1], 0xBBCC);
        assert_eq!(arr[2], 0x99AA);
        assert_eq!(arr[3], 0x7788);
        assert_eq!(arr[4], 0x5566);
        assert_eq!(arr[5], 0x3344);
        assert_eq!(arr[6], 0x1122);
    }

    #[cfg(target_endian = "big")]
    {
        assert_eq!(arr[6], 0xEEFF);
        assert_eq!(arr[5], 0xBBCC);
        assert_eq!(arr[4], 0x99AA);
        assert_eq!(arr[3], 0x7788);
        assert_eq!(arr[2], 0x5566);
        assert_eq!(arr[1], 0x3344);
        assert_eq!(arr[0], 0x1122);
    }
}

#[test]
pub fn test_u96_u32() {
    let n : u96 = u96::from(0x112233445566778899AABBCCEEFFu128);
    let arr = n.as_u32_array();

    #[cfg(target_endian = "little")]
    {
        assert_eq!(arr[0], 0xBBCCEEFF);
        assert_eq!(arr[1], 0x778899AA);
        assert_eq!(arr[2], 0x33445566);
    }

    #[cfg(target_endian = "big")]
    {
        assert_eq!(arr[2], 0xBBCCEEFF);
        assert_eq!(arr[1], 0x778899AA);
        assert_eq!(arr[0], 0x33445566);
    }
}