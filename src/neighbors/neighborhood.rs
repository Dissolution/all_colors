use crate::util::{Point, Size};
use anyhow::anyhow;
use num::traits::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::Arc;
type NeighborGroup = Arc<[Point]>;

#[derive(Debug)]
pub struct Neighborhood {
    neighbors: Arc<[NeighborGroup]>,
    pub size: Size,
}

impl Neighborhood {
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.size.width)
    }

    pub fn create<N>(size: Size, neighbor_component: &N) -> Self
    where
        N: NeighborComponent,
    {
        let count = size.area();
        let mut neighbors = Vec::with_capacity(count);
        for y in 0..size.height {
            for x in 0..size.width {
                let pt = Point::new(x, y);
                let ng = neighbor_component.get_pos_neighbors(size, pt);
                neighbors.push(Arc::from(ng));
            }
        }
        Neighborhood {
            size,
            neighbors: Arc::from(neighbors),
        }
    }

    pub fn get_neighbors(&self, point: &Point) -> &[Point] {
        let index = self.get_index(point.x, point.y);
        self.neighbors[index].as_ref()
    }
}
impl Display for Neighborhood {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Neighborhood:{}", self.size)
    }
}

pub trait NeighborComponent: Display {
    fn get_pos_neighbors(&self, size: Size, pos: Point) -> Vec<Point>;
}

pub struct OffsetNeighborComponent {
    offsets: Box<[(isize, isize)]>,
    wrap: bool,
}
impl OffsetNeighborComponent {
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
            offsets: Box::from(offsets),
            wrap,
        }
    }
}
impl Display for OffsetNeighborComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "OffsetNeighborComponent:")?;
        if self.wrap {
            write!(f, "w")?;
        }
        let mut text: String = String::new();
        for point in self.offsets.iter() {
            text.push('(');
            text.push_str(&point.0.to_string());
            text.push(',');
            text.push_str(&point.1.to_string());
            text.push(')')
        }
        write!(f, "{}", text)
    }
}
impl NeighborComponent for OffsetNeighborComponent {
    fn get_pos_neighbors(&self, size: Size, pos: Point) -> Vec<Point> {
        let offsets = self.offsets.as_ref();
        let (x, y) = (pos.x, pos.y);
        let mut pos_neighbors = Vec::with_capacity(offsets.len());
        for (x_offset, y_offset) in offsets.iter() {
            let new_x = x as isize + x_offset;
            let new_y = y as isize + y_offset;
            if self.wrap {
                pos_neighbors.push(size.wrap_xy(new_x, new_y));
            } else if let Ok(pt) = size.validate_xy(new_x, new_y) {
                pos_neighbors.push(pt);
            } else {
                // add nothing!
            }
        }
        pos_neighbors
    }
}
