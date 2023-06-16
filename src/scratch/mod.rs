use crate::colors::*;
use crate::grid::*;
use crate::pixel_fitter::PixelFitter;
use rand::prelude::*;
use std::arch::asm;
use std::fmt::{Display, Formatter, Result};
use std::thread::Thread;

struct ColorPlacerConfig<C, N, F>
where
    C: ColorSpace,
    N: NeighborManager,
    F: PixelFitter,
{
    pub seed: u64,
    rand: Box<dyn RngCore>,
    pub image_size: Size,
    pub initial_points: Vec<Point>,
    pub colorspace: C,
    pub neighbors: N,
    pub fitter: F,
}
impl<C, N, F> ColorPlacerConfig<C, N, F>
where
    C: ColorSpace,
    N: NeighborManager,
    F: PixelFitter,
{
    pub fn new(
        seed: u64,
        image_size: Size,
        colorspace: C,
        initial_points: Vec<Point>,
        neighbors: N,
        fitter: F,
    ) -> Self {
        let mut rand = StdRng::seed_from_u64(seed);
        let mut initial_points = initial_points;
        assert!(image_size.width > 0);
        assert!(image_size.height > 0);
        assert_eq!(colorspace.get_colors().len(), image_size.area());
        if initial_points.is_empty() {
            initial_points.push(Point::new(image_size.width / 2, image_size.height / 2));
        }
        Self {
            seed,
            rand: Box::new(rand),
            image_size,
            colorspace,
            initial_points,
            neighbors,
            fitter,
        }
    }
}
impl<C, N, F> Display for ColorPlacerConfig<C, N, F>
where
    C: ColorSpace,
    N: NeighborManager,
    F: PixelFitter,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}_", self.seed)?;
        write!(f, "{}x{}_", self.image_size.width, self.image_size.height)?;

        // Todo: Base64 encode?

        write!(
            f,
            "{}_{}_{}",
            self.seed, self.image_size, self.colorspace, self.neighbors, self.fitter
        )
    }
}

pub trait ColorSpace: Display {
    fn get_colors(&self) -> Vec<Color>;
}

pub trait NeighborManager {
    fn set_neighbors(&mut self, grid: &mut Grid);
}

pub struct OffsetNeighborManager {
    offsets: Vec<(isize, isize)>,
    wrap: bool,
}
impl OffsetNeighborManager {
    pub fn new(offsets: &[(isize, isize)], wrap: bool) -> Self {
        Self {
            offsets: offsets.to_owned(),
            wrap,
        }
    }
}
impl NeighborManager for OffsetNeighborManager {
    fn set_neighbors(&mut self, grid: &mut Grid) {
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
