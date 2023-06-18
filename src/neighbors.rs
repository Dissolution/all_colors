use crate::prelude::*;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;

pub trait NeighborManager: Display {
    fn set_neighbors(&self, grid: &mut Grid);
}

pub struct RandNeighborManager {
    pub seed: u64,
    pub wrap: bool,
}
impl RandNeighborManager {
    pub fn new(seed: u64, wrap: bool) -> Self {
        Self { seed, wrap }
    }
}
impl Display for RandNeighborManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "RandNeighborManager(seed: {}, wrap: {})",
            self.seed, self.wrap
        )
    }
}

impl NeighborManager for RandNeighborManager {
    fn set_neighbors(&self, grid: &mut Grid) {
        let (width, height) = (grid.size.width, grid.size.height);
        let mut pos;

        let mut rand = StdRng::seed_from_u64(self.seed);

        for y in 0..height {
            for x in 0..width {
                pos = Point::new(x, y);
                let mut pos_neighbors = Vec::with_capacity(8); // max 8 neighbors
                let neighbor_count = rand.gen_range(1..=8);
                let mut offsets = OffsetNeighborManager::STD_OFFSETS;
                offsets.shuffle(&mut rand);
                for n in 0..neighbor_count {
                    let offset = offsets[n];
                    let new_x = x as isize + offset.0;
                    let new_y = y as isize + offset.1;
                    if let Some(pt) = grid.validate_xy(new_x, new_y, self.wrap) {
                        pos_neighbors.push(pt);
                    }
                }
                grid.set_neighbors(&pos, pos_neighbors);
            }
        }
    }
}

pub struct OffsetNeighborManager {
    offsets: Vec<(isize, isize)>,
    wrap: bool,
}
impl OffsetNeighborManager {
    pub const STD_OFFSETS: [(isize, isize); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    pub const PLUS_OFFSETS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    pub const X_OFFSETS: [(isize, isize); 4] = [(-1, -1), (1, -1), (-1, 1), (1, 1)];

    pub fn new(offsets: &[(isize, isize)], wrap: bool) -> Self {
        Self {
            offsets: offsets.to_owned(),
            wrap,
        }
    }
}
impl Display for OffsetNeighborManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ONM:")?;
        if self.wrap {
            write!(f, "w")?;
        }
        let mut text: String = String::new();
        for point in self.offsets.iter() {
            text.push('(');
            text.push_str(point.0.to_string().deref());
            text.push(',');
            text.push_str(point.1.to_string().deref());
            text.push(')')
        }
        // todo: FIX THIS, no more base64!
        write_base64(&text, f)
    }
}
impl NeighborManager for OffsetNeighborManager {
    fn set_neighbors(&self, grid: &mut Grid) {
        let (width, height) = (grid.size.width, grid.size.height);
        let offsets = &self.offsets;
        let mut pos;

        for y in 0..height {
            for x in 0..width {
                pos = Point::new(x, y);
                let mut pos_neighbors = Vec::with_capacity(offsets.len());
                for (x_offset, y_offset) in offsets.iter() {
                    let new_x = x as isize + x_offset;
                    let new_y = y as isize + y_offset;
                    if let Some(pt) = grid.validate_xy(new_x, new_y, self.wrap) {
                        pos_neighbors.push(pt);
                    }
                }
                grid.set_neighbors(&pos, pos_neighbors);
            }
        }
    }
}
