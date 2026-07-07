// float.rs
//
// Copyright (c) 2021  Douglas P Lau
//
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

// TODO: Num should be moved out of this file or the file should be renamed
pub trait Num:
    num_traits::Num
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Sub<Output = Self>
    + PartialOrd
    + Debug
    + Default
    + Copy
    + Clone
    + Sized
{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min_value() -> Self;
    fn max_value() -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;/*  {
        crate::clamp(self, min, max)
    } */
}

impl Num for f32 {
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min_value() -> Self {
        Self::MIN
    }

    fn max_value() -> Self {
        Self::MAX
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        num_traits::clamp(self, min, max)
    }
}

impl Num for f64 {
    fn min(self, other: Self) -> Self {
        self.min(other)
    }

    fn max(self, other: Self) -> Self {
        self.max(other)
    }

    fn min_value() -> Self {
        Self::MIN
    }

    fn max_value() -> Self {
        Self::MAX
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        num_traits::clamp(self, min, max)
    }
}

impl Num for i32 {
    fn min(self, other: Self) -> Self {
        Ord::min(self, other)
    }

    fn max(self, other: Self) -> Self {
        Ord::max(self, other)
    }

    fn min_value() -> Self {
        Self::MIN
    }

    fn max_value() -> Self {
        Self::MAX
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        num_traits::clamp(self, min, max)
    }
}

/// Floating point component type
pub trait Float:
    num_traits::Float
    + num_traits::FloatConst
    + Num
{
    /// Calculate linear interpolation of two values
    ///
    /// The t value should be between 0 and 1.
    fn lerp(self, rhs: Self, t: Self) -> Self {
        rhs + (self - rhs) * t
    }
}

impl Float for f32 {}
impl Float for f64 {}
