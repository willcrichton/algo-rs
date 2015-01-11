//! Custom 2D point implementation.

use std::num::Float;
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Show, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Float> Point<T> {
    pub fn distance_squared(&self, other: &Point<T>) -> T {
        let diff = (self - other) * (self - other);
        diff.x + diff.y
    }

    /// Returns the distance between this and another point.
    ///
    /// ```rust
    /// # use algo::point::Point;
    /// let p: Point<f32> = Point { x: 1.0, y: 2.0 };
    /// let q = Point { x: 2.0, y: 1.0 };
    /// let r = p.distance(&q);
    /// ```
    pub fn distance(&self, other: &Point<T>) -> T {
        self.distance_squared(other).sqrt()
    }

    /// Returns the [dot product](http://en.wikipedia.org/wiki/Dot_product) of this and another point.
    ///
    /// ```rust
    /// # use algo::point::Point;
    /// let p: Point<f32> = Point { x: 1.0, y: 2.0 };
    /// let q = Point { x: 2.0, y: 1.0 };
    /// let r = p.dot(&q);
    /// ```
    pub fn dot(&self, other: &Point<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<'a, 'b, T: Add<Output = T> + Clone> Add<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn add(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone()
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a, 'b, T: Sub<Output = T> + Clone> Sub<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn sub(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() - other.x.clone(),
            y: self.y.clone() - other.y.clone()
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<'a, 'b, T: Mul<Output = T> + Clone> Mul<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn mul(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() * other.x.clone(),
            y: self.y.clone() * other.y.clone()
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Point<T>;

    fn mul(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<'a, 'b, T: Div<Output = T> + Clone> Div<&'b Point<T>> for &'a Point<T> {
    type Output = Point<T>;

    fn div(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() / other.x.clone(),
            y: self.y.clone() / other.y.clone()
        }
    }
}

impl<T: Div<Output = T>> Div for Point<T> {
    type Output = Point<T>;

    fn div(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl<'a, T: Neg<Output = T> + Clone> Neg for &'a Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point {
            x: -self.x.clone(),
            y: -self.y.clone(),
        }
    }
}

impl<T: Neg<Output = T>> Neg for Point<T> {
    type Output = Point<T>;

    fn neg(self) -> Point<T> {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let r = Point {x: 1us, y: 1} + Point {x: 2, y: 3};
        assert_eq!(r.x, 3);
        assert_eq!(r.y, 4);
    }

    #[test]
    fn sub() {
        let r = Point {x: 2us, y: 2} - Point {x: 0, y: 1};
        assert_eq!(r.x, 2);
        assert_eq!(r.y, 1);
    }

    #[test]
    fn mul() {
        let r = Point {x: 2us, y: 2} * Point {x: 0, y: 1};
        assert_eq!(r.x, 0);
        assert_eq!(r.y, 2);
    }

    #[test]
    fn div() {
        let r = Point {x: 2us, y: 2} / Point {x: 2, y: 1};
        assert_eq!(r.x, 1);
        assert_eq!(r.y, 2);
    }

    #[test]
    #[should_fail]
    fn div_fail() {
        Point {x: 2us, y: 2} / Point {x: 0, y: 1};
    }

    #[test]
    fn neg() {
        let r = -(Point {x: 1is, y: 2});
        assert_eq!(r.x, -1);
        assert_eq!(r.y, -2);
    }
}