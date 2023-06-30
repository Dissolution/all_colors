use crate::prelude::*;
use chrono::Local;
use rand::prelude::*;
use std::arch::asm;
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::Deref;
use std::thread::Thread;
use std::time::Instant;

pub struct ColorPlacerConfig<C, S, N, F>
where
    C: ColorSource,
    S: ColorSorter,
    N: NeighborComponent,
    F: PixelFitter,
{
    pub seed: u64,
    pub image_size: Size,
    pub initial_points: Vec<Point>,
    pub colorspace: C,
    pub sorter: S,
    pub neighbors: N,
    pub fitter: F,
}
impl<C, S, N, F> ColorPlacerConfig<C, S, N, F>
where
    C: ColorSource,
    S: ColorSorter,
    N: NeighborComponent,
    F: PixelFitter,
{
    pub fn new(
        seed: u64,
        image_size: Size,
        colorspace: C,
        color_sorter: S,
        initial_points: Vec<Point>,
        neighbors: N,
        fitter: F,
    ) -> Self {
        let mut initial_points = initial_points;
        assert!(image_size.width > 0);
        assert!(image_size.height > 0);
        assert_eq!(colorspace.get_colors().len(), image_size.area());
        if initial_points.is_empty() {
            initial_points.push(Point::new(image_size.width / 2, image_size.height / 2));
        }
        Self {
            seed,
            image_size,
            colorspace,
            sorter: color_sorter,
            initial_points,
            neighbors,
            fitter,
        }
    }
}
impl<C, S, N, F> Display for ColorPlacerConfig<C, S, N, F>
where
    C: ColorSource,
    S: ColorSorter,
    N: NeighborComponent,
    F: PixelFitter,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        /* We want to have a way to display a particular ColorPlacerConfig
         * so that we can encode that information in a file name (or some other place)
         * in order to recreate images in the future.
         * All of the options that can be set should be reflected in the output
         */

        // Seed for RNG
        write!(f, "{}_", self.seed)?;
        // Image size (this should already be known by looking at the image?)
        write!(f, "{}x{}_", self.image_size.width, self.image_size.height)?;
        // Starting points
        let mut text: String = String::new();
        for point in self.initial_points.iter() {
            text.push('(');
            text.push_str(point.x.to_string().deref());
            text.push(',');
            text.push_str(point.y.to_string().deref());
            text.push(')')
        }
        write!(f, "{}", text)?;

        // ColorSpace
        write!(f, "{}_", self.colorspace)?;
        // Neighbor Manager
        write!(f, "{}_", self.neighbors)?;
        // Pixel Fitter
        write!(f, "{}_", self.fitter)?;

        // Timestamp
        write!(f, "{}", Local::now().format("%Y%m%d_%H%M%S"))
    }
}
