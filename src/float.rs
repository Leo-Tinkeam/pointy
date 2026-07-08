// float.rs
//
// Copyright (c) 2021  Douglas P Lau
//
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

// TODO: Num should be moved out of this file or the file should be renamed
/// Number component type
pub trait Num:
    num_traits::Num
    + Add<Output = Self>
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
    /// Min type value
    fn min(self, other: Self) -> Self;

    /// Max type value
    fn max(self, other: Self) -> Self;

    /// Returns the minimum of two numbers
    fn min_value() -> Self;

    /// Returns the maximum of two numbers
    fn max_value() -> Self;

    /// A value bounded by a minimum and a maximum
    ///
    /// If input is less than min then this returns min. If input is greater than max then this returns max. Otherwise this returns input.
    fn clamp(self, min: Self, max: Self) -> Self;
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
    + Div<Output = Self>
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
