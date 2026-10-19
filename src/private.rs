use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::math;

pub trait Sealed:
    Add<Output = Self>
    + Copy
    + Div<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + Sized
    + Sub<Output = Self>
{
    const ZERO: Self;
    const HALF: Self;
    const ONE: Self;
    const ONE_AND_A_HALF: Self;
    const TWO: Self;
    const THREE: Self;
    const FIVE: Self;
    const SEVEN: Self;
    const TEN: Self;

    fn log10(self) -> Self;
    fn exp10(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn is_finite(self) -> bool;
    fn from_usize(x: usize) -> Self;
    fn to_usize(self) -> usize;
}

impl Sealed for f32 {
    const ZERO: Self = 0.;
    const HALF: Self = 0.5;
    const ONE: Self = 1.;
    const ONE_AND_A_HALF: Self = 1.5;
    const TWO: Self = 2.;
    const THREE: Self = 3.;
    const FIVE: Self = 5.;
    const SEVEN: Self = 7.;
    const TEN: Self = 10.;

    fn log10(self) -> Self {
        math::log10f(self)
    }

    fn exp10(self) -> Self {
        math::exp10f(self)
    }

    fn floor(self) -> Self {
        math::floorf(self)
    }

    fn ceil(self) -> Self {
        math::ceilf(self)
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl Sealed for f64 {
    const ZERO: Self = 0.;
    const HALF: Self = 0.5;
    const ONE: Self = 1.;
    const ONE_AND_A_HALF: Self = 1.5;
    const TWO: Self = 2.;
    const THREE: Self = 3.;
    const FIVE: Self = 5.;
    const SEVEN: Self = 7.;
    const TEN: Self = 10.;

    fn log10(self) -> Self {
        math::log10(self)
    }

    fn exp10(self) -> Self {
        math::exp10(self)
    }

    fn floor(self) -> Self {
        math::floor(self)
    }

    fn ceil(self) -> Self {
        math::ceil(self)
    }

    fn is_finite(self) -> bool {
        self.is_finite()
    }

    fn from_usize(x: usize) -> Self {
        x as Self
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}
