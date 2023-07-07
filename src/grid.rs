use crate::prelude::*;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct Cell {
    pub position: Point,
    pub color: Option<Color>,
    pub neighbors: Box<[Point]>,
}
impl Cell {
    fn new(pos: Point) -> Self {
        Cell {
            position: pos,
            color: None,
            neighbors: Box::from([]),
        }
    }
}
impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}: {:?}  [{:?}]",
            self.position, self.color, self.neighbors
        )
    }
}
impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}: {:?}", self.position, self.color)
    }
}
#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub struct Grid {
    pub size: Size,
    pub(crate) cells: Box<[Cell]>,
}
impl Grid {
    pub fn new(size: Size) -> Self {
        let mut cells = Vec::with_capacity(size.area());
        for y in 0..size.height {
            for x in 0..size.width {
                cells.push(Cell::new(Point::new(x, y)));
            }
        }
        Grid {
            size,
            cells: Box::from(cells),
        }
    }

    fn get_index(&self, pos: &Point) -> usize {
        pos.x + (pos.y * self.size.width)
    }

    pub fn get_cell(&self, pos: &Point) -> &Cell {
        let index = self.get_index(pos);
        self.cells.get(index).expect("Invalid Position")
    }
    pub fn get_cell_mut(&mut self, pos: &Point) -> &mut Cell {
        let index = self.get_index(pos);
        self.cells.get_mut(index).expect("Invalid Position")
    }

    pub fn get_uncolored_points(&self) -> Vec<Point> {
        self.cells
            .iter()
            .filter(|c| c.color.is_none())
            .map(|c| c.position)
            .collect()
    }

    pub fn get_neighbor_colors(&self, pos: &Point) -> Vec<Color> {
        let neighbors = self.get_cell(pos).neighbors.as_ref();
        neighbors
            .iter()
            .filter_map(|n| self.get_cell(n).color)
            .collect()
    }

    pub fn get_neighbors(&self, pos: &Point) -> Vec<&Cell> {
        self.get_cell(pos)
            .neighbors
            .iter()
            .map(|p| self.get_cell(p))
            .collect()
    }

    // pub fn iter_neighbor_cells(&self, pos: &Point) -> dyn Iterator<Item = &Cell> {
    //     self.get_cell(pos)
    //         .neighbors
    //         .iter()
    //         .map(|p| self.get_cell(p))
    // }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Grid: {}", self.size)
    }
}

impl TryInto<bmp::Image> for Grid {
    type Error = AllColorsError;

    fn try_into(self) -> Result<bmp::Image, Self::Error> {
        let width: u32 = self.size.width.try_into()?;
        let height: u32 = self.size.height.try_into()?;
        let mut image = bmp::Image::new(width, height);
        let mut pt;
        for y in 0..height {
            for x in 0..width {
                pt = Point::new(x as usize, y as usize);
                let color = self.get_cell(&pt).color;
                match color {
                    Some(c) => {
                        image.set_pixel(x, y, c.into());
                    }
                    None => {
                        return Err(AllColorsError::MissingColor(pt));
                    }
                }
            }
        }
        Ok(image)
    }
}
