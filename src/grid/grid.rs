use crate::colors::*;
use crate::errors::AllColorsError;
use crate::grid::*;
use bmp::{Image, Pixel};
use num::traits::*;
use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct Grid {
    colors: Vec<Option<Color>>,
    neighbors: Vec<Vec<Point>>,
    pub size: Size,
}
impl Grid {
    pub fn new(size: Size) -> Self {
        let count = size.area();
        let mut colors = Vec::with_capacity(count);
        let mut neighbors = Vec::with_capacity(count);
        for y in 0..size.height {
            for x in 0..size.width {
                colors.push(None);
                neighbors.push(Vec::with_capacity(8));
            }
        }
        debug_assert_eq!(count, colors.len());
        debug_assert_eq!(count, neighbors.len());
        Grid {
            size,
            colors,
            neighbors,
        }
    }

    #[inline(always)]
    fn get_xy_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.size.width)
    }
    #[inline(always)]
    fn get_point_index(&self, pos: &Point) -> usize {
        pos.x + (pos.y * self.size.width)
    }

    pub fn validate_xy(&self, x: isize, y: isize, wrap: bool) -> Option<Point> {
        if !wrap {
            if x < 0 || (x as usize >= self.width()) || y < 0 || (y as usize >= self.height()) {
                None
            } else {
                Some(Point::new(x as usize, y as usize))
            }
        } else {
            let new_x = if x < 0 || (x as usize >= self.width()) {
                x.rem_euclid(self.width() as isize)
            } else {
                x
            };
            let new_y = if y < 0 || (y as usize >= self.height()) {
                y.rem_euclid(self.height() as isize)
            } else {
                y
            };
            Some(Point::new(new_x as usize, new_y as usize))
        }
    }

    pub fn width(&self) -> usize {
        self.size.width
    }
    pub fn height(&self) -> usize {
        self.size.height
    }
    pub fn area(&self) -> usize {
        self.size.area()
    }

    fn try_get_index(&self, point: &Point) -> Result<usize, AllColorsError> {
        if point.x >= self.size.width {
            Err(AllColorsError::InvalidPoint(*point))
        } else if point.y >= self.size.height {
            Err(AllColorsError::InvalidPoint(*point))
        } else {
            Ok(point.x + (point.y * self.size.width))
        }
    }

    pub fn get_neighbors(&self, point: &Point) -> &[Point] {
        let index = self.try_get_index(point).unwrap();
        self.neighbors[index].as_slice()
    }
    pub fn set_neighbors(&mut self, point: &Point, neighbors: Vec<Point>) {
        let index = self.try_get_index(point).unwrap();
        self.neighbors[index] = neighbors
    }

    pub fn get_color(&self, point: &Point) -> Option<Color> {
        let index = self.try_get_index(point).unwrap();
        self.colors[index]
    }
    pub fn set_color(&mut self, point: &Point, color: &Color) {
        let index = self.try_get_index(point).unwrap();
        self.colors[index] = Some(*color)
    }
    pub fn get_neighbor_colors(&self, point: &Point) -> Vec<Color> {
        self.get_neighbors(point)
            .iter()
            .filter_map(|n| self.get_color(n))
            .collect()
    }
}

impl TryInto<Image> for Grid {
    type Error = AllColorsError;

    fn try_into(self) -> Result<Image, Self::Error> {
        let width: u32 = self.size.width.try_into()?;
        let height: u32 = self.size.height.try_into()?;
        let mut image = Image::new(width, height);
        let mut pt;
        for y in 0..height {
            for x in 0..width {
                pt = Point::new(x as usize, y as usize);
                let color = self.get_color(&pt);
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
