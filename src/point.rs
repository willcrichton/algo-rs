//! Custom 2D point implementation.

use std::num::Float;

#[deriving(Show, Clone)]
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

impl<'a, 'b, T: Add<T, T> + Clone> Add<&'b Point<T>, Point<T>> for &'a Point<T> {
    fn add(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone()
        }
    }
}

impl<T: Add<T, T>> Add<Point<T>, Point<T>> for Point<T> {
    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a, 'b, T: Sub<T, T> + Clone> Sub<&'b Point<T>, Point<T>> for &'a Point<T> {
    fn sub(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() - other.x.clone(),
            y: self.y.clone() - other.y.clone()
        }
    }
}

impl<T: Sub<T, T>> Sub<Point<T>, Point<T>> for Point<T> {
    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<'a, 'b, T: Mul<T, T> + Clone> Mul<&'b Point<T>, Point<T>> for &'a Point<T> {
    fn mul(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() * other.x.clone(),
            y: self.y.clone() * other.y.clone()
        }
    }
}

impl<T: Mul<T, T>> Mul<Point<T>, Point<T>> for Point<T> {
    fn mul(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl<'a, 'b, T: Div<T, T> + Clone> Div<&'b Point<T>, Point<T>> for &'a Point<T> {
    fn div(self, other: &'b Point<T>) -> Point<T> {
        Point {
            x: self.x.clone() / other.x.clone(),
            y: self.y.clone() / other.y.clone()
        }
    }
}

impl<T: Div<T, T>> Div<Point<T>, Point<T>> for Point<T> {
    fn div(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl<'a, T: Neg<T> + Clone> Neg<Point<T>> for &'a Point<T> {
    fn neg(self) -> Point<T> {
        Point {
            x: -self.x.clone(),
            y: -self.y.clone(),
        }
    }
}

impl<T: Neg<T>> Neg<Point<T>> for Point<T> {
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
        let r = Point {x: 1u, y: 1} + Point {x: 2, y: 3};
        assert_eq!(r.x, 3);
        assert_eq!(r.y, 4);
    }

    #[test]
    fn sub() {
        let r = Point {x: 2u, y: 2} - Point {x: 0, y: 1};
        assert_eq!(r.x, 2);
        assert_eq!(r.y, 1);
    }

    #[test]
    fn mul() {
        let r = Point {x: 2u, y: 2} * Point {x: 0, y: 1};
        assert_eq!(r.x, 0);
        assert_eq!(r.y, 2);
    }

    #[test]
    fn div() {
        let r = Point {x: 2u, y: 2} / Point {x: 2, y: 1};
        assert_eq!(r.x, 1);
        assert_eq!(r.y, 2);
    }

    #[test]
    #[should_fail]
    fn div_fail() {
        Point {x: 2u, y: 2} / Point {x: 0, y: 1};
    }

    #[test]
    fn neg() {
        let r = -(Point {x: 1i, y: 2});
        assert_eq!(r.x, -1);
        assert_eq!(r.y, -2);
    }
}