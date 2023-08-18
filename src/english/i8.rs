use core::ops::{Add, Rem, Mul, Sub, Shl, Shr, Div, BitOr, BitAnd, BitXor, AddAssign, SubAssign, MulAssign, DivAssign, ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign, RemAssign};

impl From<i8> for I8English {
    fn from(value: i8) -> Self {
        // SAFETY: the enum is exhaustive
        unsafe {core::mem::transmute(value)}
    }
}

impl Into<i8> for I8English {
    fn into(self) -> i8 {
		self as i8
    }
}

impl Add for I8English {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self as i8 + rhs as i8).into()
    }
}

impl Sub for I8English {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (self as i8 - rhs as i8).into()
    }
}

impl Mul for I8English {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (self as i8 * rhs as i8).into()
    }
}

impl Div for I8English {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (self as i8 / rhs as i8).into()
    }
}


impl Shl for I8English {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        ((self as i8).shl(rhs as i8)).into()
    }
}

impl Shr for I8English {
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        ((self as i8).shr(rhs as i8)).into()
    }
}

impl BitAnd for I8English {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ((self as i8).bitand(rhs as i8)).into()
    }
}

impl BitOr for I8English {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ((self as i8).bitor(rhs as i8)).into()
    }
}

impl BitXor for I8English {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        ((self as i8).bitxor(rhs as i8)).into()
    }
}

impl Rem for I8English {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        ((self as i8).rem(rhs as i8)).into()
    }
}

impl AddAssign for I8English {
    fn add_assign(&mut self, rhs: Self) {
		*self = self.add(rhs)
	}
}

impl SubAssign for I8English {
    fn sub_assign(&mut self, rhs: Self) {
		*self = self.sub(rhs)
	}
}

impl MulAssign for I8English {
    fn mul_assign(&mut self, rhs: Self) {
		*self = self.mul(rhs)
	}
}

impl DivAssign for I8English {
    fn div_assign(&mut self, rhs: Self) {
		*self = self.div(rhs)
	}
}

impl ShlAssign for I8English {
    fn shl_assign(&mut self, rhs: Self) {
		*self = self.shl(rhs)
	}
}

impl ShrAssign for I8English {
    fn shr_assign(&mut self, rhs: Self) {
		*self = self.shr(rhs)
	}
}

impl BitAndAssign for I8English {
    fn bitand_assign(&mut self, rhs: Self) {
		*self = self.bitand(rhs)
	}
}

impl BitOrAssign for I8English {
    fn bitor_assign(&mut self, rhs: Self) {
		*self = self.bitor(rhs)
	}
}

impl BitXorAssign for I8English {
    fn bitxor_assign(&mut self, rhs: Self) {
		*self = self.bitxor(rhs)
	}
}

impl RemAssign for I8English {
    fn rem_assign(&mut self, rhs: Self) {
		*self = self.rem(rhs);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i8)]
pub enum I8English {
	MinusOneHundredTwentyEight = -128,
	MinusOneHundredTwentySeven = -127,
	MinusOneHundredTwentySix = -126,
	MinusOneHundredTwentyFive = -125,
	MinusOneHundredTwentyFour = -124,
	MinusOneHundredTwentyThree = -123,
	MinusOneHundredTwentyTwo = -122,
	MinusOneHundredTwentyOne = -121,
	MinusOneHundredTwenty = -120,
	MinusOneHundredNineteen = -119,
	MinusOneHundredEighteen = -118,
	MinusOneHundredSeventeen = -117,
	MinusOneHundredSixteen = -116,
	MinusOneHundredFifteen = -115,
	MinusOneHundredFourteen = -114,
	MinusOneHundredThirteen = -113,
	MinusOneHundredTwelve = -112,
	MinusOneHundredEleven = -111,
	MinusOneHundredTen = -110,
	MinusOneHundredNine = -109,
	MinusOneHundredEight = -108,
	MinusOneHundredSeven = -107,
	MinusOneHundredSix = -106,
	MinusOneHundredFive = -105,
	MinusOneHundredFour = -104,
	MinusOneHundredThree = -103,
	MinusOneHundredTwo = -102,
	MinusOneHundredOne = -101,
	MinusOneHundred = -100,
	MinusNinetyNine = -99,
	MinusNinetyEight = -98,
	MinusNinetySeven = -97,
	MinusNinetySix = -96,
	MinusNinetyFive = -95,
	MinusNinetyFour = -94,
	MinusNinetyThree = -93,
	MinusNinetyTwo = -92,
	MinusNinetyOne = -91,
	MinusNinety = -90,
	MinusEightyNine = -89,
	MinusEightyEight = -88,
	MinusEightySeven = -87,
	MinusEightySix = -86,
	MinusEightyFive = -85,
	MinusEightyFour = -84,
	MinusEightyThree = -83,
	MinusEightyTwo = -82,
	MinusEightyOne = -81,
	MinusEighty = -80,
	MinusSeventyNine = -79,
	MinusSeventyEight = -78,
	MinusSeventySeven = -77,
	MinusSeventySix = -76,
	MinusSeventyFive = -75,
	MinusSeventyFour = -74,
	MinusSeventyThree = -73,
	MinusSeventyTwo = -72,
	MinusSeventyOne = -71,
	MinusSeventy = -70,
	MinusSixtyNine = -69,
	MinusSixtyEight = -68,
	MinusSixtySeven = -67,
	MinusSixtySix = -66,
	MinusSixtyFive = -65,
	MinusSixtyFour = -64,
	MinusSixtyThree = -63,
	MinusSixtyTwo = -62,
	MinusSixtyOne = -61,
	MinusSixty = -60,
	MinusFiftyNine = -59,
	MinusFiftyEight = -58,
	MinusFiftySeven = -57,
	MinusFiftySix = -56,
	MinusFiftyFive = -55,
	MinusFiftyFour = -54,
	MinusFiftyThree = -53,
	MinusFiftyTwo = -52,
	MinusFiftyOne = -51,
	MinusFifty = -50,
	MinusFortyNine = -49,
	MinusFortyEight = -48,
	MinusFortySeven = -47,
	MinusFortySix = -46,
	MinusFortyFive = -45,
	MinusFortyFour = -44,
	MinusFortyThree = -43,
	MinusFortyTwo = -42,
	MinusFortyOne = -41,
	MinusForty = -40,
	MinusThirtyNine = -39,
	MinusThirtyEight = -38,
	MinusThirtySeven = -37,
	MinusThirtySix = -36,
	MinusThirtyFive = -35,
	MinusThirtyFour = -34,
	MinusThirtyThree = -33,
	MinusThirtyTwo = -32,
	MinusThirtyOne = -31,
	MinusThirty = -30,
	MinusTwentyNine = -29,
	MinusTwentyEight = -28,
	MinusTwentySeven = -27,
	MinusTwentySix = -26,
	MinusTwentyFive = -25,
	MinusTwentyFour = -24,
	MinusTwentyThree = -23,
	MinusTwentyTwo = -22,
	MinusTwentyOne = -21,
	MinusTwenty = -20,
	MinusNineteen = -19,
	MinusEighteen = -18,
	MinusSeventeen = -17,
	MinusSixteen = -16,
	MinusFifteen = -15,
	MinusFourteen = -14,
	MinusThirteen = -13,
	MinusTwelve = -12,
	MinusEleven = -11,
	MinusTen = -10,
	MinusNine = -9,
	MinusEight = -8,
	MinusSeven = -7,
	MinusSix = -6,
	MinusFive = -5,
	MinusFour = -4,
	MinusThree = -3,
	MinusTwo = -2,
	MinusOne = -1,
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
}
