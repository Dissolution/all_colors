use crate::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;
use rustc_hash::*;
use std::slice::Iter;

pub struct GridFiller;
impl GridFiller {
    pub fn create_grid<C, S, N, F>(config: &ColorPlacerConfig<C, S, N, F>) -> Grid
    where
        C: ColorSource,
        S: ColorSorter,
        N: NeighborManager,
        F: PixelFitter + Sync + Send,
    {
        // Create grid
        let mut grid = Grid::new(config.image_size);
        // Set neighbors
        config.neighbors.set_neighbors(&mut grid);

        // Get the colors
        let mut colors = config.colorspace.get_colors();
        // Sort them
        config.sorter.sort_colors(&mut colors);

        // Configure our update interval (~1%)
        let clf = colors.len() as f32;
        let update_interval = f32::floor(0.01 * clf) as usize;
        let percent_multi = 100.0 / clf;

        // Fill all starting points immediately
        let start_points = &config.initial_points;

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

        let pixel_fitter = &config.fitter;

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

            // Search for the best possible position to place this Color

            // if we have none available (weird neighbor config?)
            let best_pos = if available.is_empty() {
                // get all empty points
                let empty = grid.get_available_points();
                if empty.is_empty() {
                    panic!("WTF?");
                }
                // choose one at random
                let mut rand = StdRng::seed_from_u64(config.seed);
                let i = rand.gen_range(0..empty.len());
                println!("No points available, choosing one at random...");
                empty[i]
            } else {
                // Find the best from available
                available
                    // in parallel!
                    .par_iter()
                    .map(|pt| {
                        // attach a fit to the point
                        let fit = pixel_fitter.calculate_fit(&grid, pt, color);
                        (fit, pt)
                    })
                    // get the lowest/best fit
                    .min_by_key(|t| t.0)
                    .expect("There aren't enough available pixels!")
                    // only the point, we no longer care about fit
                    .1
                    // copy so we no longer are referencing available
                    .to_owned()
            };

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

        // Finished!
        grid
    }
}
