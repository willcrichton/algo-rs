use std::num::Float;

use point::Point;

pub trait ClosestPair<T: Float> {
    fn closest_pair<'a>(&self, points: &'a [Point<T>]) -> (Point<T>, Point<T>);
}

pub struct Grids;

impl<T: Float> ClosestPair<T> for Grids {
    fn closest_pair<'a>(&self, points: &'a [Point<T>]) -> (Point<T>, Point<T>) {
        panic!("fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn simple() {
        let points = vec![
            Point { x: 0.0f32, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 7.0, y: 7.0 },
            ];

        Grids.closest_pair(points.as_slice());
    }
}