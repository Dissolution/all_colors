use crate::color::Color;
use crate::colorspace::ColorSpace;
use crate::neighbor_picker::*;
use crate::pixel_fitter::PixelFitter;
use crate::pixel_grid::PixelGrid;
use crate::stopwatch::Stopwatch;
use crate::traits::*;
use bmp::*;
use rayon::prelude::*;
use rustc_hash::FxHashSet;

pub struct ImageCreator;
impl ImageCreator {
    pub fn create_image<S, N, F>(
        colorspace: &ColorSpace,
        color_sorter: &mut S,
        neighbor_picker: &mut N,
        _: &F,
    ) -> Image
    where
        S: ColorSorter,
        N: NeighborPicker,
        F: PixelFitter,
    {
        // time this
        let mut stopwatch = Stopwatch::start_new();

        let mut colors = colorspace.generate_colors();
        let color_count = colors.len();
        color_sorter.sort_colors(&mut colors);
        let mut grid = PixelGrid::create::<N>(colorspace, neighbor_picker);
        let mut available = FxHashSet::default();
        let start_pos = colorspace.start_pos;

        println!("Setup: {:?}", stopwatch.restart_elapsed());

        // set our starting pixel and all its neighbors
        grid.set_point_color(&start_pos, &colors[0]);
        let all_neighbors = grid.get_point_neighbors(&start_pos);
        //let all_neighbors = FnNeighborPicker::STD.get_neighbors(&grid.size(), &start_pos);
        for neighbor in all_neighbors.iter() {
            available.insert(*neighbor);
        }

        // process all other colors
        for (i, color) in colors.into_iter().skip(1).enumerate() {
            // update on progress every so often
            if i % 512 == 0 {
                println!(
                    "{:.2}% -- queue: {}",
                    (i as f32 / color_count as f32) * 100_f32,
                    available.len()
                );
            }

            // Find the best from available
            let best_pos = available
                .par_iter()
                .map(|pt| {
                    let fit = F::calculate_fit(colorspace, &grid, pt, &color);
                    (fit, pt)
                })
                .min_by(|l, r| l.0.cmp(&r.0))
                .unwrap()
                .1
                .to_owned();

            // set the pixel
            grid.set_point_color(&best_pos, &color);
            // adjust available
            available.remove(&best_pos);
            //add empty neighbors to available
            for neighbor in grid.get_point_neighbors(&best_pos) {
                if grid.get_point_color(neighbor).is_none() {
                    available.insert(*neighbor);
                }
            }
        }

        println!("{:.2}% -- queue: {}", 100_f32, available.len());

        assert_eq!(available.len(), 0);

        println!("Pixel fill: {:?}", stopwatch.restart_elapsed());

        let width: u32 = colorspace.width as u32;
        let height: u32 = colorspace.height as u32;
        let default_color: Color = colorspace.default_color;
        let mut image = Image::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let color = grid
                    .get_color(x as usize, y as usize)
                    .unwrap_or(&default_color)
                    .to_owned();
                image.set_pixel(x, y, color.into());
            }
        }
        image
    }
}
