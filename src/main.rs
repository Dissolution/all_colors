#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate lazy_static;

mod colors;
mod config;
mod grid_filler;
mod neighbors;
mod pixel_fitter;
mod pixel_grid;
mod prelude;
mod util;

use crate::prelude::*;

use bmp::Image;
use chrono::*;
use rand::prelude::*;
use rand::rngs::StdRng;
use rustc_hash::FxHashSet;
use std::fmt::{Display, Formatter, Result};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;

fn main() {
    // timing
    let mut stopwatch = Stopwatch::start_new();

    // Seed for all random operations, for reproducibility
    let seed: u64 = 555;

    // The size of image we are creating
    // Mutable as certain ColorSpace operations might change this for fit
    #[allow(unused_mut)]
    let mut image_size = Size::new(256, 256);
    // LinkedIn header image = Size::new(1584, 396);

    // The Colorspace is the configuration for how we're getting the colors
    // we are placing. This is one of the ways we really change how an image looks
    let color_source;

    // Example: min/max
    // let min_max = ColorChannels::find_min_min_max_counts(image_size)
    //     .expect("!!!"); // todo: implement closest fit variant!
    // let red_channel = ColorChannel::equidistant(min_max.0);
    // let green_channel = ColorChannel::equidistant(min_max.0);
    // let blue_channel = ColorChannel::equidistant(min_max.1);

    // Example: min/mid/max
    //let min_mid_max = ColorChannels::find_min_mid_max_counts(image_size).expect("!!!"); // todo: implement closest fit variant!
    //let red_channel = ColorChannel::equidistant(min_mid_max.0);
    //let green_channel = ColorChannel::equidistant(min_mid_max.1);
    //let blue_channel = ColorChannel::equidistant(min_mid_max.2);

    // Setup the colorspace and verify it can fill the image _exactly_
    // Todo: partial image fills? overfills?
    //color_source = ColorSpace::new(red_channel, green_channel, blue_channel);

    // Always verify colors == area
    // Todo: But what if not?
    //assert_eq!(color_source.color_count(), image_size.area());

    // We can also do completely random colors!
    color_source = RandColorSource::new(seed, image_size.area());

    // The sorter
    let color_sorter;

    // Just shuffle
    color_sorter = RandColorSorter::new(seed);

    // Functional!
    // color_sorter = FnColorSorter::new(|colors| {
    //     colors.sort_by_key(|color| {
    //         // demon fire
    //         //((color.red as usize) << 16) + ((color.blue as usize) << 8) + (color.green as usize)
    //         //((color.red) << 16) + (color.blue) + (color.green)
    //
    //         // white -> blue/black
    //         let dist_from_white = Color::WHITE.dist(color);
    //         let dist_from_black = Color::BLACK.dist(color);
    //         let dist_from_blue = /* Color::MAX_DIST - */ color.dist(&Color::new(0, 0, 255));
    //         dist_from_black + dist_from_blue
    //
    //         //(color.blue * color.blue) + color.red + color.green
    //
    //         /*(255 + 255 + 255) -*/
    //         //(color.red) + (color.blue) + (color.green)
    //
    //         // let min = color.red.min(color.green.min(color.blue));
    //         // let max = color.red.max(color.green.max(color.blue));
    //         // max - min
    //
    //         //((color.red + color.green + color.blue) as f32 / 3.0) as usize
    //
    //         // usize::abs_diff(color.red, color.green)
    //         //     + usize::abs_diff(color.red, color.blue)
    //         //     + usize::abs_diff(color.green, color.blue)
    //     });
    // });

    // Starting point(s)
    let mut start_points = Vec::new();

    // Just the center
    let center_pos = Point::new(image_size.width / 2, image_size.height / 2);
    start_points.push(center_pos);

    // // A bunch of points at random
    // {
    //     let mut rand = StdRng::seed_from_u64(seed);
    //
    //     for _ in 0..16 {
    //         let x = rand.gen_range(0..image_size.width);
    //         let y = rand.gen_range(0..image_size.height);
    //         let pt = Point::new(x, y);
    //         start_points.push(pt);
    //     }
    // }

    // # Neighbor Manager
    let neighbor_component;

    // offsets
    // There are presets on OffsetNeighborManager
    neighbor_component = OffsetNeighborComponent::new(&OffsetNeighborComponent::STD_OFFSETS, false);

    // Random!
    //neighbor_manager = RandNeighborManager::new(seed, false);

    // # Fitter
    let fitter;

    // simple color dist
    fitter = ColorDistPixelFitter;

    // color and x/y dist (have to specify the x/y center dist)
    // todo: ways of using all starting points?
    //fitter = ColorAndPixelDistPixelFitter::new(center_pos);

    let config = ColorPlacerConfig::new(
        seed,
        image_size,
        color_source,
        color_sorter,
        start_points,
        neighbor_component,
        fitter,
    );

    println!("Configuration Setup: {:?}", stopwatch.restart_elapsed());

    let grid = GridFiller::create_grid(&config);

    println!("Grid Filled: {:?}", stopwatch.restart_elapsed());

    let image: Image = grid.try_into().expect("missing pixel!");

    let now = Local::now();

    let config_display = format!("{}", config);

    let bmp_filename = format!("rust_{}.bmp", now.format("%Y%m%d_%H%M%S"));
    let image_path = format!("c:\\temp\\all_colors\\{}", bmp_filename);
    image.save(&image_path).expect("Should have saved");

    let json_file_path = "c:\\temp\\all_colors\\info.json".to_string();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&json_file_path)
        .expect("Cannot create or open info.json file!");
    let mut logline = String::new();
    logline.push_str(&image_path);
    logline.push_str(": ");
    logline.push_str(&json_file_path);
    file.write_all(logline.as_bytes())
        .expect("Couldn't write to info.json file!");

    println!(
        "Image created and saved: \n{}\n{:?}",
        config_display,
        stopwatch.restart_elapsed()
    );
}

pub struct TestColorSpace {
    pub pixel_count: usize,
}
impl Display for TestColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CS:Test")
    }
}
impl ColorSource for TestColorSpace {
    fn get_colors(&self) -> Vec<Color> {
        let pixel_count = self.pixel_count;
        let mut colors = Vec::with_capacity(pixel_count);
        let mut i: isize = 0;
        while colors.len() < pixel_count {
            colors.push(Color::new(i as u8, 0, 0));
            colors.push(Color::new(0, i as u8, 0));
            colors.push(Color::new(0, 0, i as u8));
            i += 1;
            if i > (u8::MAX as isize) {
                i = 0;
            }
        }

        while colors.len() > pixel_count {
            colors.remove(colors.len() - 1);
        }

        colors
    }
}
