use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl From<i8> for I8Japanese {
    fn from(value: i8) -> Self {
        // SAFETY: the enum is exhaustive
        unsafe { core::mem::transmute(value) }
    }
}

impl Into<i8> for I8Japanese {
    fn into(self) -> i8 {
        self as i8
    }
}

impl Add for I8Japanese {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self as i8 + rhs as i8).into()
    }
}

impl Sub for I8Japanese {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as i8 - rhs as i8).into()
    }
}

impl Mul for I8Japanese {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self as i8 * rhs as i8).into()
    }
}

impl Div for I8Japanese {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self as i8 / rhs as i8).into()
    }
}

impl Shl for I8Japanese {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        ((self as i8).shl(rhs as i8)).into()
    }
}

impl Shr for I8Japanese {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        ((self as i8).shr(rhs as i8)).into()
    }
}

impl BitAnd for I8Japanese {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ((self as i8).bitand(rhs as i8)).into()
    }
}

impl BitOr for I8Japanese {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ((self as i8).bitor(rhs as i8)).into()
    }
}

impl BitXor for I8Japanese {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        ((self as i8).bitxor(rhs as i8)).into()
    }
}

impl Rem for I8Japanese {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ((self as i8).rem(rhs as i8)).into()
    }
}

impl AddAssign for I8Japanese {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl SubAssign for I8Japanese {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl MulAssign for I8Japanese {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

impl DivAssign for I8Japanese {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs)
    }
}

impl ShlAssign for I8Japanese {
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.shl(rhs)
    }
}

impl ShrAssign for I8Japanese {
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.shr(rhs)
    }
}

impl BitAndAssign for I8Japanese {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs)
    }
}

impl BitOrAssign for I8Japanese {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs)
    }
}

impl BitXorAssign for I8Japanese {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs)
    }
}

impl RemAssign for I8Japanese {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum I8Japanese {
    マイナス百二十八 = -128,
    マイナス百二十七 = -127,
    マイナス百二十六 = -126,
    マイナス百二十五 = -125,
    マイナス百二十四 = -124,
    マイナス百二十三 = -123,
    マイナス百二十二 = -122,
    マイナス百二十一 = -121,
    マイナス百二十 = -120,
    マイナス百十九 = -119,
    マイナス百十八 = -118,
    マイナス百十七 = -117,
    マイナス百十六 = -116,
    マイナス百十五 = -115,
    マイナス百十四 = -114,
    マイナス百十三 = -113,
    マイナス百十二 = -112,
    マイナス百十一 = -111,
    マイナス百十 = -110,
    マイナス百九 = -109,
    マイナス百八 = -108,
    マイナス百七 = -107,
    マイナス百六 = -106,
    マイナス百五 = -105,
    マイナス百四 = -104,
    マイナス百三 = -103,
    マイナス百二 = -102,
    マイナス百一 = -101,
    マイナス百 = -100,
    マイナス九十九 = -99,
    マイナス九十八 = -98,
    マイナス九十七 = -97,
    マイナス九十六 = -96,
    マイナス九十五 = -95,
    マイナス九十四 = -94,
    マイナス九十三 = -93,
    マイナス九十二 = -92,
    マイナス九十一 = -91,
    マイナス九十 = -90,
    マイナス八十九 = -89,
    マイナス八十八 = -88,
    マイナス八十七 = -87,
    マイナス八十六 = -86,
    マイナス八十五 = -85,
    マイナス八十四 = -84,
    マイナス八十三 = -83,
    マイナス八十二 = -82,
    マイナス八十一 = -81,
    マイナス八十 = -80,
    マイナス七十九 = -79,
    マイナス七十八 = -78,
    マイナス七十七 = -77,
    マイナス七十六 = -76,
    マイナス七十五 = -75,
    マイナス七十四 = -74,
    マイナス七十三 = -73,
    マイナス七十二 = -72,
    マイナス七十一 = -71,
    マイナス七十 = -70,
    マイナス六十九 = -69,
    マイナス六十八 = -68,
    マイナス六十七 = -67,
    マイナス六十六 = -66,
    マイナス六十五 = -65,
    マイナス六十四 = -64,
    マイナス六十三 = -63,
    マイナス六十二 = -62,
    マイナス六十一 = -61,
    マイナス六十 = -60,
    マイナス五十九 = -59,
    マイナス五十八 = -58,
    マイナス五十七 = -57,
    マイナス五十六 = -56,
    マイナス五十五 = -55,
    マイナス五十四 = -54,
    マイナス五十三 = -53,
    マイナス五十二 = -52,
    マイナス五十一 = -51,
    マイナス五十 = -50,
    マイナス四十九 = -49,
    マイナス四十八 = -48,
    マイナス四十七 = -47,
    マイナス四十六 = -46,
    マイナス四十五 = -45,
    マイナス四十四 = -44,
    マイナス四十三 = -43,
    マイナス四十二 = -42,
    マイナス四十一 = -41,
    マイナス四十 = -40,
    マイナス三十九 = -39,
    マイナス三十八 = -38,
    マイナス三十七 = -37,
    マイナス三十六 = -36,
    マイナス三十五 = -35,
    マイナス三十四 = -34,
    マイナス三十三 = -33,
    マイナス三十二 = -32,
    マイナス三十一 = -31,
    マイナス三十 = -30,
    マイナス二十九 = -29,
    マイナス二十八 = -28,
    マイナス二十七 = -27,
    マイナス二十六 = -26,
    マイナス二十五 = -25,
    マイナス二十四 = -24,
    マイナス二十三 = -23,
    マイナス二十二 = -22,
    マイナス二十一 = -21,
    マイナス二十 = -20,
    マイナス十九 = -19,
    マイナス十八 = -18,
    マイナス十七 = -17,
    マイナス十六 = -16,
    マイナス十五 = -15,
    マイナス十四 = -14,
    マイナス十三 = -13,
    マイナス十二 = -12,
    マイナス十一 = -11,
    マイナス十 = -10,
    マイナス九 = -9,
    マイナス八 = -8,
    マイナス七 = -7,
    マイナス六 = -6,
    マイナス五 = -5,
    マイナス四 = -4,
    マイナス三 = -3,
    マイナス二 = -2,
    マイナス一 = -1,
    零 = 0,
    一 = 1,
    二 = 2,
    三 = 3,
    四 = 4,
    五 = 5,
    六 = 6,
    七 = 7,
    八 = 8,
    九 = 9,
    十 = 10,
    十一 = 11,
    十二 = 12,
    十三 = 13,
    十四 = 14,
    十五 = 15,
    十六 = 16,
    十七 = 17,
    十八 = 18,
    十九 = 19,
    二十 = 20,
    二十一 = 21,
    二十二 = 22,
    二十三 = 23,
    二十四 = 24,
    二十五 = 25,
    二十六 = 26,
    二十七 = 27,
    二十八 = 28,
    二十九 = 29,
    三十 = 30,
    三十一 = 31,
    三十二 = 32,
    三十三 = 33,
    三十四 = 34,
    三十五 = 35,
    三十六 = 36,
    三十七 = 37,
    三十八 = 38,
    三十九 = 39,
    四十 = 40,
    四十一 = 41,
    四十二 = 42,
    四十三 = 43,
    四十四 = 44,
    四十五 = 45,
    四十六 = 46,
    四十七 = 47,
    四十八 = 48,
    四十九 = 49,
    五十 = 50,
    五十一 = 51,
    五十二 = 52,
    五十三 = 53,
    五十四 = 54,
    五十五 = 55,
    五十六 = 56,
    五十七 = 57,
    五十八 = 58,
    五十九 = 59,
    六十 = 60,
    六十一 = 61,
    六十二 = 62,
    六十三 = 63,
    六十四 = 64,
    六十五 = 65,
    六十六 = 66,
    六十七 = 67,
    六十八 = 68,
    六十九 = 69,
    七十 = 70,
    七十一 = 71,
    七十二 = 72,
    七十三 = 73,
    七十四 = 74,
    七十五 = 75,
    七十六 = 76,
    七十七 = 77,
    七十八 = 78,
    七十九 = 79,
    八十 = 80,
    八十一 = 81,
    八十二 = 82,
    八十三 = 83,
    八十四 = 84,
    八十五 = 85,
    八十六 = 86,
    八十七 = 87,
    八十八 = 88,
    八十九 = 89,
    九十 = 90,
    九十一 = 91,
    九十二 = 92,
    九十三 = 93,
    九十四 = 94,
    九十五 = 95,
    九十六 = 96,
    九十七 = 97,
    九十八 = 98,
    九十九 = 99,
    百 = 100,
    百一 = 101,
    百二 = 102,
    百三 = 103,
    百四 = 104,
    百五 = 105,
    百六 = 106,
    百七 = 107,
    百八 = 108,
    百九 = 109,
    百十 = 110,
    百十一 = 111,
    百十二 = 112,
    百十三 = 113,
    百十四 = 114,
    百十五 = 115,
    百十六 = 116,
    百十七 = 117,
    百十八 = 118,
    百十九 = 119,
    百二十 = 120,
    百二十一 = 121,
    百二十二 = 122,
    百二十三 = 123,
    百二十四 = 124,
    百二十五 = 125,
    百二十六 = 126,
    百二十七 = 127,
}
