use crate::colors::*;
use crate::grid::*;
use crate::pixel_fitter::PixelFitter;
use rayon::prelude::*;
use rustc_hash::*;

pub struct ColorPlacer;
impl ColorPlacer {
    pub fn fill_grid<F>(
        grid: &mut Grid,
        available: &mut FxHashSet<Point>,
        colors: &[Color],
        pixel_fitter: &F,
    ) where
        F: PixelFitter + Sync + Send,
    {
        let clf = colors.len() as f32;
        let update_interval = f32::floor(0.01 * clf) as usize;
        let percent_multi = 100.0 / clf;

        // process all colors
        for (i, color) in colors.into_iter().enumerate() {
            // update on progress every so often
            if i % update_interval == 0 {
                println!(
                    "{:.2}% -- queue: {}",
                    (i as f32 * percent_multi),
                    available.len()
                );
            }

            // Find the best from available
            let best_pos = available
                .par_iter()
                .map(|pt| {
                    let fit = pixel_fitter.calculate_fit(&grid, pt, &color);
                    (fit, pt)
                })
                .min_by(|l, r| l.0.cmp(&r.0))
                .unwrap()
                .1
                .to_owned();

            // set the pixel
            grid.set_color(&best_pos, &color);
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
