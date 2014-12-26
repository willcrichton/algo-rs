use std::fmt::Show;
use std::num::{Float, ToPrimitive};
use std::iter::range_inclusive;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use rustc::util::nodemap::FnvHasher;

use point::Point;
use FnvMap;

pub trait ClosestPair<T: Float> {
    fn closest_pair<'a>(&self, points: &'a [Point<T>]) -> (&'a Point<T>, &'a Point<T>);
}

pub struct SarielHarPeled<T>;

struct Grid<'a, T: 'a> {
    table: FnvMap<(int, int), Vec<&'a Point<T>>>,
    min_dist: T,
    min_pair: (&'a Point<T>, &'a Point<T>),
}


impl<T: Float + Show + ToPrimitive> SarielHarPeled<T> {
    fn boxify(&self, p: &Point<T>, r: T) -> (int, int) {
        ((p.x / r).floor().to_int().unwrap(), (p.y / r).floor().to_int().unwrap())
    }

    fn insert_point<'a>(&self, grid: &mut Grid<'a, T>, p: &'a Point<T>) {
        match grid.table.entry(self.boxify(p, grid.min_dist)) {
            Occupied(mut entry) => {
                entry.get_mut().push(p);
            },
            Vacant(space) => {
                space.set(vec![p]);
            }
        }
    }

    fn make_grid<'a>(&self, p: &'a Point<T>, q: &'a Point<T>) -> Grid<'a, T> {
        let mut table = HashMap::with_hasher(FnvHasher);
        let r = p.distance(q);

        let mut grid = Grid {
            table: table,
            min_dist: r,
            min_pair: (p, q),
        };

        // what if p and q are in the same box?
        self.insert_point(&mut grid, p);
        self.insert_point(&mut grid, q);

        grid
    }

    fn lookup<'a>(&self, grid: &Grid<'a, T>, p: &'a Point<T>) -> Option<(&'a Point<T>, T)> {
        let (x, y) = self.boxify(p, grid.min_dist);
        let (mut q, mut r) = (p, grid.min_dist);
        for nx in range_inclusive(x - 1, x + 1) {
            for ny in range_inclusive(y - 1, y + 1) {
                match grid.table.find(&(nx, ny)) {
                    Some(pts) => {
                        let (index, dist) = pts
                            .iter().enumerate()
                            .map(|(i, q)| (i, p.distance(*q)))
                            .fold((0, r), |(i, acc), (j, dist)| {
                                if dist < acc {
                                    (j, dist)
                                } else {
                                    (i, acc)
                                }
                            });

                        if dist < r {
                            r = dist;
                            q = pts[index];
                        }
                    },
                    None => {}
                }
            }
        }

        if r < grid.min_dist {
            Some((q, r))
        } else {
            None
        }
    }

    fn insert<'a>(&self, grid: &mut Grid<'a, T>, p: &'a Point<T>) {
        match self.lookup(grid, p) {
            Some((q, r)) => {
                let points = grid.table.values().fold(Vec::new(), |mut acc, v| {
                    acc.push_all(v.as_slice());
                    acc
                });

                grid.table.clear();
                grid.min_dist = r;
                grid.min_pair = (p, q);

                for point in points.into_iter() {
                    self.insert_point(grid, p);
                }
            },
            None => {
                self.insert_point(grid, p);
            }
        };
    }
}

impl<T: Float + Show + ToPrimitive> ClosestPair<T> for SarielHarPeled<T> {
    fn closest_pair<'a>(&self, points: &'a [Point<T>]) -> (&'a Point<T>, &'a Point<T>) {
        let len = points.len();
        if len < 2 {
            panic!("need at least 2 points to find closest pair, have {}", points.len())
        } else if len == 2 {
            (&points[0], &points[1])
        } else {
            let mut grid = self.make_grid(&points[0], &points[1]);
            for i in range(2, points.len()) {
                self.insert(&mut grid, &points[i]);
                if i == points.len() - 1 {
                    return grid.min_pair;
                }
            }

            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;
    use std::num::Float;

    #[test]
    fn simple() {
        let points = vec![
            Point { x: 0.0f32, y: 0.0 },
            Point { x: 10.0, y: 10.0 },
            Point { x: 7.0, y: 7.0 },
            ];

        let (p, q) = SarielHarPeled.closest_pair(points.as_slice());
        assert!((p.distance(q) - points[1].distance(&points[2])).abs() < 0.001);
    }
}