use std::fmt::*;
use std::hash::Hash;

pub type UPoint = Point<usize>;
pub type IPoint = Point<isize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point<T>
where
    T: Clone + Copy + Eq + PartialEq + Hash + Default,
{
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Clone + Copy + Eq + PartialEq + Hash + Default,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Display for Point<T>
where
    T: Clone + Copy + Eq + PartialEq + Hash + Default + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
