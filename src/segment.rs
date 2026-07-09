// segment.rs    2D Line Segments
//
// Copyright (c) 2020-2025  Douglas P Lau
//
use crate::bbox::{BBox, Bounded, Bounds};
use crate::number::{Float, Num};
use crate::line::Line;
use crate::point::Pt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Segment of a line between two endpoints
///
/// ```rust
/// use pointy::Seg;
///
/// let seg = Seg::new((10.0, 15.0), (0.0, 2.0));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seg<N>
where
    N: Num,
{
    /// First endpoint
    pub p0: Pt<N>,

    /// Second endpoint
    pub p1: Pt<N>,
}

trait Intersect {
    fn intersects(self, rhs: Self) -> bool;
}

impl<N> Bounded<N> for Seg<N>
where
    N: Num,
    Seg<N>: Intersect,
{
    fn bounded_by(self, bbox: BBox<N>) -> bool {
        let p0 = bbox.check(self.p0.x, self.p0.y);
        let p1 = bbox.check(self.p1.x, self.p1.y);
        match (p0, p1) {
            (Bounds::Within, _) | (_, Bounds::Within) => true,
            // both opposite horizontally
            (Bounds::Left, Bounds::Right) => true,
            (Bounds::Right, Bounds::Left) => true,
            // both opposite vertically
            (Bounds::Below, Bounds::Above) => true,
            (Bounds::Above, Bounds::Below) => true,
            (
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
                Bounds::Left | Bounds::BelowLeft | Bounds::AboveLeft,
            ) => false,
            (
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
                Bounds::Right | Bounds::BelowRight | Bounds::AboveRight,
            ) => false,
            (
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
                Bounds::Below | Bounds::BelowLeft | Bounds::BelowRight,
            ) => false,
            (
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
                Bounds::Above | Bounds::AboveLeft | Bounds::AboveRight,
            ) => false,
            (Bounds::Left, _) | (_, Bounds::Left) => {
                self.intersects(bbox.x_min_edge())
            }
            (Bounds::Right, _) | (_, Bounds::Right) => {
                self.intersects(bbox.x_max_edge())
            }
            (Bounds::BelowLeft, _) | (_, Bounds::BelowLeft) => {
                self.intersects(bbox.x_min_edge())
                    || self.intersects(bbox.y_min_edge())
            }
            (Bounds::AboveLeft, _) | (_, Bounds::AboveLeft) => {
                self.intersects(bbox.x_min_edge())
                    || self.intersects(bbox.y_max_edge())
            }
            (Bounds::BelowRight, _) | (_, Bounds::BelowRight) => {
                self.intersects(bbox.x_max_edge())
                    || self.intersects(bbox.y_min_edge())
            }
            (Bounds::AboveRight, _) | (_, Bounds::AboveRight) => {
                self.intersects(bbox.x_max_edge())
                    || self.intersects(bbox.y_max_edge())
            }
        }
    }
}

impl<N> Seg<N>
where
    N: Num,
{
    /// Create a new line segment
    pub fn new<P0, P1>(p0: P0, p1: P1) -> Self
    where
        P0: Into<Pt<N>>,
        P1: Into<Pt<N>>,
    {
        Self {
            p0: p0.into(),
            p1: p1.into(),
        }
    }
}

// Since we can't do mutually exclusive trait for Integer and Float, this should be repeted for all Integer types
// It may be better to do a macro that generates this code for each Integer type if we want more types
impl Intersect for Seg<i32>
{
    /// Check if segment intersects with another segment
    fn intersects(self, rhs: Self) -> bool {
        Seg::<f64>::new(
            Pt::<f64>::new(self.p0.x.into(), self.p0.y.into()),
            Pt::<f64>::new(self.p1.x.into(), self.p1.y.into()),
        ).intersection(Seg::<f64>::new(
            Pt::<f64>::new(rhs.p0.x.into(), rhs.p0.y.into()),
            Pt::<f64>::new(rhs.p1.x.into(), rhs.p1.y.into()),
        )).is_some()
    }
}

impl<F> Intersect for Seg<F>
where
    F: Float
{
    /// Check if segment intersects with another segment
    fn intersects(self, rhs: Self) -> bool {
        self.intersection(rhs).is_some()
    }
}

impl<F> Seg<F>
where
    F: Float,
{
    /// Get the distance from the line segment to a point
    pub fn distance<P>(self, pt: P) -> F
    where
        P: Into<Pt<F>>,
    {
        let pt = pt.into();
        // If the dot product of `v0` and `v1` is greater than zero,
        // then the nearest point on the segment is `p1`
        let v0 = self.p1 - self.p0;
        let v1 = pt - self.p1;
        if v0.dot(v1) > F::zero() {
            return v1.mag();
        }
        // If the dot product of `v2` and `v3` is greater than zero,
        // then the nearest point on the segment is `p0`
        let v2 = self.p0 - self.p1;
        let v3 = pt - self.p0;
        if v2.dot(v3) > F::zero() {
            return v3.mag();
        }
        // Otherwise, the nearest point on the segment is between
        // `p0` and `p1`, so calculate the point-line distance
        (v0 * v3).abs() / v0.mag()
    }

    /// Get the point where two segments intersect
    pub fn intersection(self, rhs: Self) -> Option<Pt<F>> {
        let l0 = Line::new(self.p0, self.p1);
        let l1 = Line::new(rhs.p0, rhs.p1);
        l0.intersection(l1)
            .filter(|p| p.bounded_by(BBox::new([rhs.p0, rhs.p1])))
    }

    /// Clip segment with a bounding box
    pub fn clip(mut self, bbox: BBox<F>) -> Option<Self> {
        if !self.bounded_by(bbox) {
            return None;
        }
        if let Some(p) = self.intersection(bbox.x_min_edge()) {
            let xmn = bbox.x_min();
            if self.p0.x < xmn {
                self.p0 = p;
            } else if self.p1.x < xmn {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.x_max_edge()) {
            let xmx = bbox.x_max();
            if self.p0.x > xmx {
                self.p0 = p;
            } else if self.p1.x > xmx {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.y_min_edge()) {
            let ymn = bbox.y_min();
            if self.p0.y < ymn {
                self.p0 = p;
            } else if self.p1.y < ymn {
                self.p1 = p;
            }
        }
        if let Some(p) = self.intersection(bbox.y_max_edge()) {
            let ymx = bbox.y_max();
            if self.p0.y > ymx {
                self.p0 = p;
            } else if self.p1.y > ymx {
                self.p1 = p;
            }
        }
        Some(self)
    }
}

// Private BBox helper functions
impl<N> BBox<N>
where
    N: Num,
{
    /// Get edge on X min side
    fn x_min_edge(self) -> Seg<N> {
        let xmn = self.x_min();
        Seg::new((xmn, self.y_min()), (xmn, self.y_max()))
    }

    /// Get edge on X max side
    fn x_max_edge(self) -> Seg<N> {
        let xmx = self.x_max();
        Seg::new((xmx, self.y_min()), (xmx, self.y_max()))
    }

    /// Get edge on Y min side
    fn y_min_edge(self) -> Seg<N> {
        let ymn = self.y_min();
        Seg::new((self.x_min(), ymn), (self.x_max(), ymn))
    }

    /// Get edge on Y max side
    fn y_max_edge(self) -> Seg<N> {
        let ymx = self.y_max();
        Seg::new((self.x_min(), ymx), (self.x_max(), ymx))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance() {
        let a = Seg::new((0.0, 0.0), (10.0, 0.0));
        assert_eq!(a.distance((0.0, 5.0)), 5.0);
        assert_eq!(a.distance((5.0, 5.0)), 5.0);
        assert_eq!(a.distance((10.0, 5.0)), 5.0);
        assert_eq!(a.distance((-5.0, 0.0)), 5.0);
        assert_eq!(a.distance((15.0, 0.0)), 5.0);
        assert_eq!(a.distance((0.0, -5.0)), 5.0);
        assert_eq!(a.distance((5.0, -5.0)), 5.0);
        assert_eq!(a.distance((10.0, -5.0)), 5.0);
    }

    #[test]
    fn intersection() {
        let a = Seg::new((0.0, 0.0), (1.0, 0.0));
        assert_eq!(a.intersection(a), None);
        let b = Seg::new((1.0, 1.0), (1.0, 0.0));
        assert_eq!(a.intersection(b), Some(Pt::new(1.0, 0.0)));
        let c = Seg::new((0.5, 1.0), (0.5, 10.0));
        assert_eq!(a.intersection(c), None);
        let d = Seg::new((0.5, 1.0), (0.5, -1.0));
        assert_eq!(a.intersection(d), Some(Pt::new(0.5, 0.0)));
    }

    #[test]
    fn bounded() {
        let b = BBox::new([(0.0, 0.0), (1.0, 1.0)]);
        assert!(Seg::new((0.0, 0.0), (1.0, 1.0)).bounded_by(b));
        assert!(Seg::new((1.0, 1.0), (2.0, 2.0)).bounded_by(b));
        assert!(Seg::new((0.0, 0.0), (-1.0, -1.0)).bounded_by(b));
        assert!(!Seg::new((2.0, 2.0), (3.0, 3.0)).bounded_by(b));
        assert!(!Seg::new((-1.0, -1.0), (-2.0, -2.0)).bounded_by(b));
        assert!(Seg::new((0.5, 0.5), (1.5, 0.5)).bounded_by(b));
        assert!(Seg::new((0.5, 0.5), (0.5, 1.5)).bounded_by(b));
        assert!(Seg::new((0.5, 1.5), (1.5, 0.5)).bounded_by(b));
        assert!(!Seg::new((0.5, 1.6), (1.6, 0.5)).bounded_by(b));
        assert!(Seg::new((-0.5, 0.5), (1.5, 0.5)).bounded_by(b));
        assert!(Seg::new((0.5, -0.5), (0.5, 1.5)).bounded_by(b));
    }
}
