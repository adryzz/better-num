use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

impl From<u8> for U8English {
    fn from(value: u8) -> Self {
        // SAFETY: the enum is exhaustive
        unsafe { core::mem::transmute(value) }
    }
}

impl Into<u8> for U8English {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Add for U8English {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self as u8 + rhs as u8).into()
    }
}

impl Sub for U8English {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as u8 - rhs as u8).into()
    }
}

impl Mul for U8English {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self as u8 * rhs as u8).into()
    }
}

impl Div for U8English {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self as u8 / rhs as u8).into()
    }
}

impl Shl for U8English {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        ((self as u8).shl(rhs as u8)).into()
    }
}

impl Shr for U8English {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        ((self as u8).shr(rhs as u8)).into()
    }
}

impl BitAnd for U8English {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ((self as u8).bitand(rhs as u8)).into()
    }
}

impl BitOr for U8English {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ((self as u8).bitor(rhs as u8)).into()
    }
}

impl BitXor for U8English {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        ((self as u8).bitxor(rhs as u8)).into()
    }
}

impl Rem for U8English {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ((self as u8).rem(rhs as u8)).into()
    }
}

impl AddAssign for U8English {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl SubAssign for U8English {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs)
    }
}

impl MulAssign for U8English {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

impl DivAssign for U8English {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs)
    }
}

impl ShlAssign for U8English {
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.shl(rhs)
    }
}

impl ShrAssign for U8English {
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.shr(rhs)
    }
}

impl BitAndAssign for U8English {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.bitand(rhs)
    }
}

impl BitOrAssign for U8English {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.bitor(rhs)
    }
}

impl BitXorAssign for U8English {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.bitxor(rhs)
    }
}

impl RemAssign for U8English {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.rem(rhs);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum U8English {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
    TwentyFour = 24,
    TwentyFive = 25,
    TwentySix = 26,
    TwentySeven = 27,
    TwentyEight = 28,
    TwentyNine = 29,
    Thirty = 30,
    ThirtyOne = 31,
    ThirtyTwo = 32,
    ThirtyThree = 33,
    ThirtyFour = 34,
    ThirtyFive = 35,
    ThirtySix = 36,
    ThirtySeven = 37,
    ThirtyEight = 38,
    ThirtyNine = 39,
    Forty = 40,
    FortyOne = 41,
    FortyTwo = 42,
    FortyThree = 43,
    FortyFour = 44,
    FortyFive = 45,
    FortySix = 46,
    FortySeven = 47,
    FortyEight = 48,
    FortyNine = 49,
    Fifty = 50,
    FiftyOne = 51,
    FiftyTwo = 52,
    FiftyThree = 53,
    FiftyFour = 54,
    FiftyFive = 55,
    FiftySix = 56,
    FiftySeven = 57,
    FiftyEight = 58,
    FiftyNine = 59,
    Sixty = 60,
    SixtyOne = 61,
    SixtyTwo = 62,
    SixtyThree = 63,
    SixtyFour = 64,
    SixtyFive = 65,
    SixtySix = 66,
    SixtySeven = 67,
    SixtyEight = 68,
    SixtyNine = 69,
    Seventy = 70,
    SeventyOne = 71,
    SeventyTwo = 72,
    SeventyThree = 73,
    SeventyFour = 74,
    SeventyFive = 75,
    SeventySix = 76,
    SeventySeven = 77,
    SeventyEight = 78,
    SeventyNine = 79,
    Eighty = 80,
    EightyOne = 81,
    EightyTwo = 82,
    EightyThree = 83,
    EightyFour = 84,
    EightyFive = 85,
    EightySix = 86,
    EightySeven = 87,
    EightyEight = 88,
    EightyNine = 89,
    Ninety = 90,
    NinetyOne = 91,
    NinetyTwo = 92,
    NinetyThree = 93,
    NinetyFour = 94,
    NinetyFive = 95,
    NinetySix = 96,
    NinetySeven = 97,
    NinetyEight = 98,
    NinetyNine = 99,
    OneHundred = 100,
    OneHundredOne = 101,
    OneHundredTwo = 102,
    OneHundredThree = 103,
    OneHundredFour = 104,
    OneHundredFive = 105,
    OneHundredSix = 106,
    OneHundredSeven = 107,
    OneHundredEight = 108,
    OneHundredNine = 109,
    OneHundredTen = 110,
    OneHundredEleven = 111,
    OneHundredTwelve = 112,
    OneHundredThirteen = 113,
    OneHundredFourteen = 114,
    OneHundredFifteen = 115,
    OneHundredSixteen = 116,
    OneHundredSeventeen = 117,
    OneHundredEighteen = 118,
    OneHundredNineteen = 119,
    OneHundredTwenty = 120,
    OneHundredTwentyOne = 121,
    OneHundredTwentyTwo = 122,
    OneHundredTwentyThree = 123,
    OneHundredTwentyFour = 124,
    OneHundredTwentyFive = 125,
    OneHundredTwentySix = 126,
    OneHundredTwentySeven = 127,
    OneHundredTwentyEight = 128,
    OneHundredTwentyNine = 129,
    OneHundredThirty = 130,
    OneHundredThirtyOne = 131,
    OneHundredThirtyTwo = 132,
    OneHundredThirtyThree = 133,
    OneHundredThirtyFour = 134,
    OneHundredThirtyFive = 135,
    OneHundredThirtySix = 136,
    OneHundredThirtySeven = 137,
    OneHundredThirtyEight = 138,
    OneHundredThirtyNine = 139,
    OneHundredForty = 140,
    OneHundredFortyOne = 141,
    OneHundredFortyTwo = 142,
    OneHundredFortyThree = 143,
    OneHundredFortyFour = 144,
    OneHundredFortyFive = 145,
    OneHundredFortySix = 146,
    OneHundredFortySeven = 147,
    OneHundredFortyEight = 148,
    OneHundredFortyNine = 149,
    OneHundredFifty = 150,
    OneHundredFiftyOne = 151,
    OneHundredFiftyTwo = 152,
    OneHundredFiftyThree = 153,
    OneHundredFiftyFour = 154,
    OneHundredFiftyFive = 155,
    OneHundredFiftySix = 156,
    OneHundredFiftySeven = 157,
    OneHundredFiftyEight = 158,
    OneHundredFiftyNine = 159,
    OneHundredSixty = 160,
    OneHundredSixtyOne = 161,
    OneHundredSixtyTwo = 162,
    OneHundredSixtyThree = 163,
    OneHundredSixtyFour = 164,
    OneHundredSixtyFive = 165,
    OneHundredSixtySix = 166,
    OneHundredSixtySeven = 167,
    OneHundredSixtyEight = 168,
    OneHundredSixtyNine = 169,
    OneHundredSeventy = 170,
    OneHundredSeventyOne = 171,
    OneHundredSeventyTwo = 172,
    OneHundredSeventyThree = 173,
    OneHundredSeventyFour = 174,
    OneHundredSeventyFive = 175,
    OneHundredSeventySix = 176,
    OneHundredSeventySeven = 177,
    OneHundredSeventyEight = 178,
    OneHundredSeventyNine = 179,
    OneHundredEighty = 180,
    OneHundredEightyOne = 181,
    OneHundredEightyTwo = 182,
    OneHundredEightyThree = 183,
    OneHundredEightyFour = 184,
    OneHundredEightyFive = 185,
    OneHundredEightySix = 186,
    OneHundredEightySeven = 187,
    OneHundredEightyEight = 188,
    OneHundredEightyNine = 189,
    OneHundredNinety = 190,
    OneHundredNinetyOne = 191,
    OneHundredNinetyTwo = 192,
    OneHundredNinetyThree = 193,
    OneHundredNinetyFour = 194,
    OneHundredNinetyFive = 195,
    OneHundredNinetySix = 196,
    OneHundredNinetySeven = 197,
    OneHundredNinetyEight = 198,
    OneHundredNinetyNine = 199,
    TwoHundred = 200,
    TwoHundredOne = 201,
    TwoHundredTwo = 202,
    TwoHundredThree = 203,
    TwoHundredFour = 204,
    TwoHundredFive = 205,
    TwoHundredSix = 206,
    TwoHundredSeven = 207,
    TwoHundredEight = 208,
    TwoHundredNine = 209,
    TwoHundredTen = 210,
    TwoHundredEleven = 211,
    TwoHundredTwelve = 212,
    TwoHundredThirteen = 213,
    TwoHundredFourteen = 214,
    TwoHundredFifteen = 215,
    TwoHundredSixteen = 216,
    TwoHundredSeventeen = 217,
    TwoHundredEighteen = 218,
    TwoHundredNineteen = 219,
    TwoHundredTwenty = 220,
    TwoHundredTwentyOne = 221,
    TwoHundredTwentyTwo = 222,
    TwoHundredTwentyThree = 223,
    TwoHundredTwentyFour = 224,
    TwoHundredTwentyFive = 225,
    TwoHundredTwentySix = 226,
    TwoHundredTwentySeven = 227,
    TwoHundredTwentyEight = 228,
    TwoHundredTwentyNine = 229,
    TwoHundredThirty = 230,
    TwoHundredThirtyOne = 231,
    TwoHundredThirtyTwo = 232,
    TwoHundredThirtyThree = 233,
    TwoHundredThirtyFour = 234,
    TwoHundredThirtyFive = 235,
    TwoHundredThirtySix = 236,
    TwoHundredThirtySeven = 237,
    TwoHundredThirtyEight = 238,
    TwoHundredThirtyNine = 239,
    TwoHundredForty = 240,
    TwoHundredFortyOne = 241,
    TwoHundredFortyTwo = 242,
    TwoHundredFortyThree = 243,
    TwoHundredFortyFour = 244,
    TwoHundredFortyFive = 245,
    TwoHundredFortySix = 246,
    TwoHundredFortySeven = 247,
    TwoHundredFortyEight = 248,
    TwoHundredFortyNine = 249,
    TwoHundredFifty = 250,
    TwoHundredFiftyOne = 251,
    TwoHundredFiftyTwo = 252,
    TwoHundredFiftyThree = 253,
    TwoHundredFiftyFour = 254,
    TwoHundredFiftyFive = 255,
}
