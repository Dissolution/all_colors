use crate::prelude::*;
use bmp::{Image, Pixel};
use num::traits::*;
use std::fmt::{Display, Formatter};
use std::num::TryFromIntError;
use std::sync::Arc;

#[derive(Debug)]
pub struct PixelGrid {
    colors: Arc<[Option<Color>]>,
    pub size: Size,
}
impl PixelGrid {
    pub fn new(size: Size) -> Self {
        let count = size.area();
        let colors: Arc<[Option<Color>]> = Arc::from(vec![None; count]);
        assert_eq!(count, colors.len());
        PixelGrid { size, colors }
    }

    fn get_xy_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.size.width)
    }
    fn get_point_index(&self, pos: Point) -> usize {
        pos.x + (pos.y * self.size.width)
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

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.size.width)
    }

    pub fn get_color(&self, point: Point) -> Option<Color> {
        let index = self.get_index(point.x, point.y);
        self.colors[index]
    }
    pub fn set_color(&mut self, point: Point, color: Color) {
        let index = self.get_index(point.x, point.y);
        self.colors[index] = Some(color)
    }

    pub fn get_available_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let index = self.get_index(x, y);
                if self.colors[index].is_none() {
                    points.push(Point::new(x, y));
                }
            }
        }
        points
    }
}

impl TryInto<Image> for PixelGrid {
    type Error = AllColorsError;

    fn try_into(self) -> Result<Image, Self::Error> {
        let width: u32 = self.size.width.try_into()?;
        let height: u32 = self.size.height.try_into()?;
        let mut image = Image::new(width, height);
        let mut pt;
        for y in 0..height {
            for x in 0..width {
                pt = Point::new(x as usize, y as usize);
                let color = self.get_color(pt);
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
