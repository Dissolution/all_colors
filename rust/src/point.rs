use std::fmt::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(x: usize, y: usize) -> Size {
        Size {
            width: x,
            height: y,
        }
    }
    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}x{}]", self.width, self.height)
    }
}
