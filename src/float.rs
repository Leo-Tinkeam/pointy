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
    + Debug
    + Default
    + Copy
    + Clone
    + Sized
{
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

impl Num for f32 {
    fn min(self, other: f32) -> f32 {
        self.min(other)
    }

    fn max(self, other: f32) -> f32 {
        self.max(other)
    }
}

impl Num for f64 {
    fn min(self, other: f64) -> f64 {
        self.min(other)
    }

    fn max(self, other: f64) -> f64 {
        self.max(other)
    }
}

impl Num for i32 {
    fn min(self, other: i32) -> i32 {
        Ord::min(self, other)
    }

    fn max(self, other: i32) -> i32 {
        Ord::max(self, other)
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
