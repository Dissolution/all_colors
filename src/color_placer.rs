use crate::colors::*;
use crate::grid::*;
use crate::pixel_fitter::PixelFitter;
use rayon::prelude::*;
use rustc_hash::*;
use std::slice::Iter;

pub struct ColorPlacer;
impl ColorPlacer {
    pub fn fill_grid<F>(grid: &mut Grid, start_points: &[Point], colors: &[Color], pixel_fitter: &F)
    where
        F: PixelFitter + Sync + Send,
    {
        let clf = colors.len() as f32;
        let update_interval = f32::floor(0.01 * clf) as usize;
        let percent_multi = 100.0 / clf;

        // Fill all starting points immediately
        assert!(start_points.len() <= colors.len());
        let mut available = FxHashSet::default();
        for (i, pt) in start_points.iter().enumerate() {
            grid.set_color(pt, &colors[i]);
            //add empty neighbors to available
            for neighbor in grid.get_neighbors(pt) {
                if grid.get_color(neighbor).is_none() {
                    available.insert(*neighbor);
                }
            }
        }
        for pt in start_points.iter() {
            available.remove(pt);
        }

        // process all colors
        for (i, color) in colors.iter().enumerate().skip(start_points.len()) {
            // update on progress every so often
            if i % update_interval == 0 {
                println!(
                    "{:.0}% -- queue: {}",
                    (i as f32 * percent_multi),
                    available.len()
                );
            }

            // Find the best from available
            let best_pos = available
                // in parallel!
                .par_iter()
                .map(|pt| {
                    // attach a fit to the point
                    let fit = pixel_fitter.calculate_fit(grid, pt, color);
                    (fit, pt)
                })
                // get the lowest/best fit
                .min_by_key(|t| t.0)
                .expect("There aren't enough available pixels!")
                // only the point, we no longer care about fit
                .1
                // copy so we no longer are referencing available
                .to_owned();

            // set the pixel
            grid.set_color(&best_pos, color);

            // adjust available
            available.remove(&best_pos);

            //add empty neighbors to available
            for neighbor in grid.get_neighbors(&best_pos) {
                if grid.get_color(neighbor).is_none() {
                    available.insert(*neighbor);
                }
            }
        }

        println!("{:.2}% -- queue: {}", 100_f32, available.len());

        assert_eq!(available.len(), 0);
    }
}
