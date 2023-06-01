use crate::point::{Point, Size};
#[allow(unused_imports)]
use num::traits::*;
use rand::Rng;

pub trait NeighborPicker {
    fn get_wrapping(&self) -> bool;
    fn set_wrapping(&mut self, wrap: bool);
    fn get_neighbors(&mut self, grid: &Size, pos: &Point) -> Vec<Point>;

    fn validate_point(&self, width: isize, height: isize, x: isize, y: isize) -> Option<Point> {
        let wrapping = self.get_wrapping();
        if !wrapping {
            if x < 0 || x >= width || y < 0 || y >= height {
                None
            } else {
                Some(Point::new(x as usize, y as usize))
            }
        } else {
            let new_x = if x < 0 || x >= width {
                x.rem_euclid(width)
            } else {
                x
            };
            let new_y = if y < 0 || y >= height {
                y.rem_euclid(height)
            } else {
                y
            };
            Some(Point::new(new_x as usize, new_y as usize))
        }
    }
}

pub struct RandNeighborPicker<R>
where
    R: Rng,
{
    pub wrapping: bool,
    pub rand: R,
}
impl<R> RandNeighborPicker<R>
where
    R: Rng,
{
    #[allow(dead_code)]
    pub fn new(rand: R) -> RandNeighborPicker<R>
    where
        R: Rng,
    {
        RandNeighborPicker {
            wrapping: false,
            rand,
        }
    }
}
impl<R> NeighborPicker for RandNeighborPicker<R>
where
    R: Rng,
{
    fn get_wrapping(&self) -> bool {
        self.wrapping
    }

    fn set_wrapping(&mut self, wrap: bool) {
        self.wrapping = wrap
    }

    fn get_neighbors(&mut self, grid: &Size, pos: &Point) -> Vec<Point> {
        let all_neighbors = FnNeighborPicker::get_standard().get_neighbors(grid, pos);
        let mut neighbors = Vec::with_capacity(8);
        for n in all_neighbors.into_iter() {
            if self.rand.gen_range(0..8) <= 3 {
                neighbors.push(n);
            }
        }
        neighbors
    }
}

pub struct FnNeighborPicker {
    pub wrapping: bool,
    pub get_offsets: fn() -> Vec<(isize, isize)>,
}
impl FnNeighborPicker {
    #[allow(dead_code)]
    pub fn get_standard() -> FnNeighborPicker {
        FnNeighborPicker::new(|| {
            let mut neighbors = Vec::with_capacity(8);
            for dy in -1..=1_isize {
                for dx in -1..=1_isize {
                    if dx == 0 && dy == 0 {
                        continue;
                    } else {
                        neighbors.push((dx, dy));
                    }
                }
            }
            neighbors
        })
    }

    #[allow(dead_code)]
    pub fn get_plus() -> FnNeighborPicker {
        FnNeighborPicker::new(|| vec![(0, -1), (-1, 0), (1, 0), (0, 1)])
    }

    #[allow(dead_code)]
    pub fn get_x() -> FnNeighborPicker {
        FnNeighborPicker::new(|| vec![(-1, -1), (1, -1), (-1, 1), (1, 1)])
    }

    /// Construct a new `FnNeighborPicker` with a series of +/- x/y offsets
    pub fn new(get_offsets: fn() -> Vec<(isize, isize)>) -> Self {
        FnNeighborPicker {
            wrapping: false,
            get_offsets,
        }
    }
}

impl NeighborPicker for FnNeighborPicker {
    fn get_wrapping(&self) -> bool {
        self.wrapping
    }

    fn set_wrapping(&mut self, wrap: bool) {
        self.wrapping = wrap
    }

    fn get_neighbors(&mut self, grid: &Size, pos: &Point) -> Vec<Point> {
        let (width, height) = (grid.width as isize, grid.height as isize);
        let mut neighbors = Vec::with_capacity(8);
        let pos_x = pos.x as isize;
        let pos_y = pos.y as isize;
        let offsets = (self.get_offsets)();
        for (x_offset, y_offset) in offsets.iter() {
            let new_x = pos_x + x_offset;
            let new_y = pos_y + y_offset;
            if let Some(pt) = self.validate_point(width, height, new_x, new_y) {
                neighbors.push(pt)
            }
        }
        neighbors
    }
}
