use crate::errors::*;
use crate::grid::*;
use num::traits::*;

type GetOffsets = fn() -> Vec<(isize, isize)>;

pub struct NeighborManager;
impl NeighborManager {
    pub fn standard_offsets() -> Vec<(isize, isize)> {
        vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }

    pub fn plus_offsets() -> Vec<(isize, isize)> {
        vec![(0, -1), (-1, 0), (1, 0), (0, 1)]
    }

    pub fn x_offsets() -> Vec<(isize, isize)> {
        vec![(-1, -1), (1, -1), (-1, 1), (1, 1)]
    }

    pub fn set_neighbors(grid: &mut Grid, offsets: Vec<(isize, isize)>, wrap: bool) {
        let (width, height) = (grid.size.width, grid.size.height);
        let mut pos;

        for y in 0..height {
            for x in 0..width {
                pos = Point::new(x, y);
                let mut pos_neighbors = Vec::with_capacity(8);
                for (x_offset, y_offset) in offsets.iter() {
                    let new_x = x as isize + *x_offset;
                    let new_y = y as isize + *y_offset;
                    if let Some(pt) = grid.validate_xy(new_x, new_y, wrap) {
                        pos_neighbors.push(pt);
                    }
                }
                grid.set_neighbors(&pos, pos_neighbors);
            }
        }
    }
}
