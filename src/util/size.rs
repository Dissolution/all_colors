use crate::util::Point;
use anyhow::anyhow;
use num::traits::Euclid;
use std::fmt::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        Size { width, height }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn validate_xy(&self, x: isize, y: isize) -> anyhow::Result<Point> {
        let x = usize::try_from(x)?;
        if x >= self.width {
            return Err(anyhow!("X must be less than Width"));
        }
        let y = usize::try_from(y)?;
        if y >= self.height {
            return Err(anyhow!("Y must be less than Height"));
        }
        Ok(Point::new(x, y))
    }

    pub fn validate_point(&self, point: Point) -> anyhow::Result<Point> {
        if point.x >= self.width {
            Err(anyhow!("X must be less than Width"))
        } else if point.y >= self.height {
            Err(anyhow!("Y must be less than Height"))
        } else {
            Ok(point)
        }
    }

    pub fn wrap_xy(&self, x: isize, y: isize) -> Point {
        let width = self.width as isize;
        let new_x = if x < 0 || x >= width {
            Euclid::rem_euclid(&x, &width)
        } else {
            x
        };
        let height = self.height as isize;
        let new_y = if y < 0 || y >= height {
            Euclid::rem_euclid(&y, &height)
        } else {
            y
        };
        Point::new(new_x as usize, new_y as usize)
    }

    pub fn wrap_point(&self, point: Point) -> Point {
        let x = point.x;
        let width = self.width;
        let new_x = if x >= width {
            Euclid::rem_euclid(&x, &width)
        } else {
            x
        };
        let y = point.y;
        let height = self.height;
        let new_y = if y >= height {
            Euclid::rem_euclid(&y, &height)
        } else {
            y
        };
        Point::new(new_x, new_y)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}x{}]", self.width, self.height)
    }
}
