use std::fmt::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        Size {
            width,
            height,
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
