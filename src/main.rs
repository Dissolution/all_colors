mod color;
#[allow(unused)]
#[allow(dead_code)]
mod colorspace;
mod image_creator;
mod neighbor_picker;
mod pixel_fitter;
mod pixel_grid;
mod point;
mod stopwatch;
mod traits;

use crate::color::Color;
use crate::colorspace::*;
use crate::image_creator::ImageCreator;
use crate::neighbor_picker::*;
use crate::pixel_fitter::*;
use crate::point::Point;
use crate::traits::*;
use chrono::*;
use rand::prelude::*;
use rand::rngs::StdRng;

fn find_color_heavy_space(width: usize, height: usize) -> Option<(u8, u8)> {
    let area = (width * height) as f32;
    for x in (0_usize..=255).rev() {
        let side = f32::sqrt(area / x as f32);
        if side.fract() == 0.0 {
            let big = x as u8;
            let small = side as u8;
            return Some((big, small));
        }
    }
    None
}

fn main() {
    let color_hvy_space = find_color_heavy_space(1584, 396);
    let (blue, red_green) = color_hvy_space.expect("whoops");
    let red_selection = ColorSelection::new(red_green as usize, ColorRange::Spaced);
    let green_selection = ColorSelection::new(red_green as usize, ColorRange::Spaced);
    let blue_selection = ColorSelection::new(blue as usize, ColorRange::Spaced);

    let mut colorspace =
        ColorSpace::new([red_selection, green_selection, blue_selection], 1584, 396);

    //let mut colorspace = ColorSpace::closest_fit(1584, 396);
    //let mut color_selections = colorspace.colors;
    colorspace.start_pos = Point::new(
        colorspace.width / 8,
        colorspace.height - (colorspace.height / 30),
    );

    // configure our colorspace
    //let colorspace = ColorSpace::closest_square([ColorSelection::new(40, ColorRange::Spaced); 3]);
    const SEED: u64 = 147;
    let mut sorter = FnColorSorter::new(|colors| {
        let mut rand = StdRng::seed_from_u64(SEED);
        colors.shuffle(&mut rand);
        colors.sort_by_key(|color| {
            // demon fire
            //((color.red) << 16) + ((color.blue) << 8) + (color.green)
            //((color.red) << 16) + (color.blue) + (color.green)

            // white -> blue/black
            let dist_from_white = Color::WHITE.dist(color);
            let dist_from_blue = Color::MAX_DIST - color.dist(&Color::new(0, 0, 255));
            dist_from_white + dist_from_blue

            //(color.blue * color.blue) + color.red + color.green

            /*(255 + 255 + 255) -*/
            //(color.red) + (color.blue) + (color.green)

            // let min = color.red.min(color.green.min(color.blue));
            // let max = color.red.max(color.green.max(color.blue));
            // max - min

            //((color.red + color.green + color.blue) as f32 / 3.0) as usize

            // usize::abs_diff(color.red, color.green)
            //     + usize::abs_diff(color.red, color.blue)
            //     + usize::abs_diff(color.green, color.blue)
        });
    });

    let mut neighbor_picker = FnNeighborPicker::get_standard();
    //neighbor_picker.wrapping = true;
    let fitter = ColorDistPixelFitter;
    let image = ImageCreator::create_image(&colorspace, &mut sorter, &mut neighbor_picker, &fitter);
    let now = Local::now();

    let file_name = format!("rust_{}.bmp", now.format("%Y%m%d_%H%M%S"));

    let path = format!("c:\\temp\\all_colors\\{}", file_name);
    let saved = image.save(path);
    saved.expect("Should have saved");
}
