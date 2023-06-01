use crate::color::Color;
use crate::colorspace::ColorSpace;
use crate::neighbor_picker::NeighborPicker;
use crate::point::{Point, Size};

pub struct PixelGrid {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Option<Color>>,
    neighbors: Vec<Vec<Point>>,
}
impl PixelGrid {
    #[inline(always)]
    fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }
    #[inline(always)]
    fn get_point_index(&self, pos: &Point) -> usize {
        pos.x + (pos.y * self.width)
    }

    pub fn create<N>(colorspace: &ColorSpace, neighbor_picker: &mut N) -> PixelGrid
    where
        N: NeighborPicker,
    {
        let (width, height) = (colorspace.width, colorspace.height);
        let size = colorspace.size();
        let area = size.area();
        let mut neighbors = Vec::with_capacity(area);
        let mut pt;
        for y in 0..height {
            for x in 0..width {
                pt = Point::new(x, y);
                let pos_neighbors = neighbor_picker.get_neighbors(&size, &pt);
                neighbors.push(pos_neighbors);
            }
        }
        PixelGrid {
            width,
            height,
            pixels: vec![None; area],
            neighbors,
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    pub fn get_color(&self, x: usize, y: usize) -> Option<&Color> {
        let index = self.get_index(x, y);
        let px = self.pixels.get(index)?;
        px.as_ref()
    }

    pub fn get_point_color(&self, pos: &Point) -> Option<&Color> {
        let index = self.get_point_index(pos);
        let px = self.pixels.get(index)?;
        px.as_ref()
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, x: usize, y: usize, color: &Color) {
        let index = self.get_index(x, y);
        self.pixels[index] = Some(*color);
    }

    pub fn set_point_color(&mut self, pos: &Point, color: &Color) {
        let index = self.get_point_index(pos);
        self.pixels[index] = Some(*color);
    }

    #[allow(dead_code)]
    pub fn get_neighbors(&self, x: usize, y: usize) -> &[Point] {
        let index = self.get_index(x, y);
        self.neighbors[index].as_ref()
    }

    pub fn get_point_neighbors(&self, pos: &Point) -> &[Point] {
        let index = self.get_point_index(pos);
        self.neighbors[index].as_ref()
    }
}
