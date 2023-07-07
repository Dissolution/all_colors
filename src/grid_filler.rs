use crate::prelude::*;
use crate::grid::{Cell, Grid};
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
        N: NeighborComponent,
        F: PixelFitter + Sync + Send,
    {
        // Create grids
        let mut pixelgrid = Grid::new(config.image_size);
        let nc = &config.neighbors;
        for cell in pixelgrid.cells.iter_mut() {
            cell.neighbors = Box::from(nc.get_pos_neighbors(pixelgrid.size, cell.position));
        }
        //let mut neighborgrid = Neighborhood::create(config.image_size, &config.neighbors);

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
            pixelgrid.get_cell_mut(pt).color = Some(colors[i]);
            //add empty neighbors to available
            pixelgrid
                .get_neighbors(pt)
                .iter()
                .filter_map(|n| {
                    if n.color.is_none() {
                        Some(n.position)
                    } else {
                        None
                    }
                })
                .for_each(|n| {
                    available.insert(n);
                });
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
                let empty = pixelgrid.get_uncolored_points();
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
                        let fit = pixel_fitter.calculate_fit(&pixelgrid, pt, color);
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
            pixelgrid.get_cell_mut(&best_pos).color = Some(*color);

            // adjust available
            available.remove(&best_pos);

            //add empty neighbors to available
            pixelgrid
                .get_neighbors(&best_pos)
                .iter()
                .filter_map(|n| {
                    if n.color.is_none() {
                        Some(n.position)
                    } else {
                        None
                    }
                })
                .for_each(|n| {
                    available.insert(n);
                });
        }

        println!("{:.2}% -- queue: {}", 100_f32, available.len());

        assert_eq!(available.len(), 0);

        // Finished!
        pixelgrid
    }
}
