use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl From<u8> for U8Japanese {
    fn from(value: u8) -> Self {
        // SAFETY: the enum is exhaustive
        unsafe { core::mem::transmute(value) }
    }
}

impl Into<u8> for U8Japanese {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Add for U8Japanese {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self as u8 + rhs as u8).into()
    }
}

impl Sub for U8Japanese {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as u8 - rhs as u8).into()
    }
}

impl Mul for U8Japanese {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self as u8 * rhs as u8).into()
    }
}

impl Div for U8Japanese {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self as u8 / rhs as u8).into()
    }
}

impl Shl for U8Japanese {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        ((self as u8).shl(rhs as u8)).into()
    }
}

impl Shr for U8Japanese {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        ((self as u8).shr(rhs as u8)).into()
    }
}

impl BitAnd for U8Japanese {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ((self as u8).bitand(rhs as u8)).into()
    }
}

impl BitOr for U8Japanese {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ((self as u8).bitor(rhs as u8)).into()
    }
}

impl BitXor for U8Japanese {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        ((self as u8).bitxor(rhs as u8)).into()
    }
}

impl Rem for U8Japanese {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ((self as u8).rem(rhs as u8)).into()
    }
}

impl AddAssign for U8Japanese {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl SubAssign for U8Japanese {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl MulAssign for U8Japanese {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

impl DivAssign for U8Japanese {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs)
    }
}

impl ShlAssign for U8Japanese {
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.shl(rhs)
    }
}

impl ShrAssign for U8Japanese {
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.shr(rhs)
    }
}

impl BitAndAssign for U8Japanese {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs)
    }
}

impl BitOrAssign for U8Japanese {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs)
    }
}

impl BitXorAssign for U8Japanese {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs)
    }
}

impl RemAssign for U8Japanese {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum U8Japanese {
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
    百二十八 = 128,
    百二十九 = 129,
    百三十 = 130,
    百三十一 = 131,
    百三十二 = 132,
    百三十三 = 133,
    百三十四 = 134,
    百三十五 = 135,
    百三十六 = 136,
    百三十七 = 137,
    百三十八 = 138,
    百三十九 = 139,
    百四十 = 140,
    百四十一 = 141,
    百四十二 = 142,
    百四十三 = 143,
    百四十四 = 144,
    百四十五 = 145,
    百四十六 = 146,
    百四十七 = 147,
    百四十八 = 148,
    百四十九 = 149,
    百五十 = 150,
    百五十一 = 151,
    百五十二 = 152,
    百五十三 = 153,
    百五十四 = 154,
    百五十五 = 155,
    百五十六 = 156,
    百五十七 = 157,
    百五十八 = 158,
    百五十九 = 159,
    百六十 = 160,
    百六十一 = 161,
    百六十二 = 162,
    百六十三 = 163,
    百六十四 = 164,
    百六十五 = 165,
    百六十六 = 166,
    百六十七 = 167,
    百六十八 = 168,
    百六十九 = 169,
    百七十 = 170,
    百七十一 = 171,
    百七十二 = 172,
    百七十三 = 173,
    百七十四 = 174,
    百七十五 = 175,
    百七十六 = 176,
    百七十七 = 177,
    百七十八 = 178,
    百七十九 = 179,
    百八十 = 180,
    百八十一 = 181,
    百八十二 = 182,
    百八十三 = 183,
    百八十四 = 184,
    百八十五 = 185,
    百八十六 = 186,
    百八十七 = 187,
    百八十八 = 188,
    百八十九 = 189,
    百九十 = 190,
    百九十一 = 191,
    百九十二 = 192,
    百九十三 = 193,
    百九十四 = 194,
    百九十五 = 195,
    百九十六 = 196,
    百九十七 = 197,
    百九十八 = 198,
    百九十九 = 199,
    二百 = 200,
    二百一 = 201,
    二百二 = 202,
    二百三 = 203,
    二百四 = 204,
    二百五 = 205,
    二百六 = 206,
    二百七 = 207,
    二百八 = 208,
    二百九 = 209,
    二百十 = 210,
    二百十一 = 211,
    二百十二 = 212,
    二百十三 = 213,
    二百十四 = 214,
    二百十五 = 215,
    二百十六 = 216,
    二百十七 = 217,
    二百十八 = 218,
    二百十九 = 219,
    二百二十 = 220,
    二百二十一 = 221,
    二百二十二 = 222,
    二百二十三 = 223,
    二百二十四 = 224,
    二百二十五 = 225,
    二百二十六 = 226,
    二百二十七 = 227,
    二百二十八 = 228,
    二百二十九 = 229,
    二百三十 = 230,
    二百三十一 = 231,
    二百三十二 = 232,
    二百三十三 = 233,
    二百三十四 = 234,
    二百三十五 = 235,
    二百三十六 = 236,
    二百三十七 = 237,
    二百三十八 = 238,
    二百三十九 = 239,
    二百四十 = 240,
    二百四十一 = 241,
    二百四十二 = 242,
    二百四十三 = 243,
    二百四十四 = 244,
    二百四十五 = 245,
    二百四十六 = 246,
    二百四十七 = 247,
    二百四十八 = 248,
    二百四十九 = 249,
    二百五十 = 250,
    二百五十一 = 251,
    二百五十二 = 252,
    二百五十三 = 253,
    二百五十四 = 254,
    二百五十五 = 255,
}
