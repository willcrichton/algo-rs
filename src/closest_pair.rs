//! Implements [closest pair algorithms](http://en.wikipedia.org/wiki/Closest_pair_of_points_problem) on 2D points.

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::ops::{Add, Mul};

use point::Point;

/// Given a set of points P, returns the two points closest to each other.
///
/// Assumes |P| >= 2.
pub trait ClosestPair<T: Add<T> + Mul<T>> {
    // takes a set of points, return a pair of the closest points (no particular order)
    fn closest_pair<'a>(&self, points: &'a [Point<T>]) -> (&'a Point<T>, &'a Point<T>);
}

// see http://www.cs.cmu.edu/~15451/lectures/lec23/closest-pair.txt
// for description of the algorithm (not sure if there's a better name...)
pub struct SarielHarPeled;

struct Grid<'a, T: 'a> {
    table: HashMap<(i32, i32), Vec<&'a Point<T>>>,
    min_dist: T,
    min_pair: (&'a Point<T>, &'a Point<T>),
}


// Float for sqrt math and ToPrimitive so it can be converted to i32 indices
// Not guaranteed to work for edge cases with NaN/infinity/etc.
fn boxify(p: &Point<f32>, r: f32) -> (i32, i32) {
    ((p.x / r).floor() as i32, (p.y / r).floor() as i32)
}

fn insert_point<'a>(grid: &mut Grid<'a, f32>, p: &'a Point<f32>) {
    match grid.table.entry(boxify(p, grid.min_dist)) {
        Occupied(mut entry) => {
            entry.get_mut().push(p);
        },
        Vacant(space) => {
            space.insert(vec![p]);
        }
    }
}

fn make_grid<'a>(p: &'a Point<f32>, q: &'a Point<f32>) -> Grid<'a, f32> {
    let table = HashMap::new();
    let r = p.distance(q);

    let mut grid = Grid {
        table: table,
        min_dist: r,
        min_pair: (p, q),
    };

    // what if p and q are in the same box?
    insert_point(&mut grid, p);
    insert_point(&mut grid, q);

    grid
}

fn lookup<'a>(grid: &Grid<'a, f32>, p: &'a Point<f32>) -> Option<(&'a Point<f32>, f32)> {
    let (x, y) = boxify(p, grid.min_dist);
    let (mut q, mut r) = (p, grid.min_dist);
    for nx in (x - 1)..(x + 2) {
        for ny in (y - 1)..(y + 2) {
            match grid.table.get(&(nx, ny)) {
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

fn insert<'a>(grid: &mut Grid<'a, f32>, p: &'a Point<f32>) {
    match lookup(grid, p) {
        Some((q, r)) => {
            /*let points = grid.table.values().fold(Vec::new(), |mut acc, v| {
                acc.push_all(v.as_slice());
                acc
            });*/
            let copy = grid.table.clone();
            let points = copy.values().flat_map(|x| x).collect::<Vec<&&Point<f32>>>();

            grid.table.clear();
            grid.min_dist = r;
            grid.min_pair = (p, q);

            for point in points.into_iter() {
                insert_point(grid, point);
            }
        },
        None => {
            insert_point(grid, p);
        }
    };
}

impl ClosestPair<f32> for SarielHarPeled {
    fn closest_pair<'a>(&self, points: &'a [Point<f32>]) -> (&'a Point<f32>, &'a Point<f32>) {
        let len = points.len();
        if len < 2 {
            panic!("need at least 2 points to find closest pair, have {}", points.len())
        } else if len == 2 {
            (&points[0], &points[1])
        } else {
            let mut grid = make_grid(&points[0], &points[1]);
            for i in 2..points.len() {
                insert(&mut grid, &points[i]);
            }

            grid.min_pair
        }
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
            Point { x: 5.0, y: 5.0 },
            Point { x: 1.0, y: 1.0 },
            ];

        let (p, q) = SarielHarPeled.closest_pair(&points);
        assert!((p.distance(q) - points[0].distance(&points[4])).abs() < 0.001);
    }
}