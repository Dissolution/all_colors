use crate::color::Color;
use crate::colorspace::ColorSpace;
use crate::pixel_grid::PixelGrid;
use crate::point::Point;

#[inline(always)]
fn color_distance(first: &Color, second: &Color) -> usize {
    // 'true' implementation would return the square root of the below
    let r_diff = usize::abs_diff(first.red, second.red);
    let g_diff = usize::abs_diff(first.green, second.green);
    let b_diff = usize::abs_diff(first.blue, second.blue);
    (r_diff * r_diff) + (g_diff * g_diff) + (b_diff * b_diff)
}

#[inline(always)]
fn point_distance(start_pos: &Point, pos: &Point) -> usize {
    let x_diff = usize::abs_diff(start_pos.x, pos.x);
    let y_diff = usize::abs_diff(start_pos.y, pos.y);
    //(x_diff * x_diff) + (y_diff * y_diff)
    //(x_diff + (y_diff * x_diff))
    x_diff * y_diff
}

pub trait PixelFitter {
    fn calculate_fit(
        colorspace: &ColorSpace,
        grid: &PixelGrid,
        pos: &Point,
        color: &Color,
    ) -> usize;
}

pub struct ColorDistPixelFitter;
impl PixelFitter for ColorDistPixelFitter {
    fn calculate_fit(_: &ColorSpace, grid: &PixelGrid, pos: &Point, color: &Color) -> usize {
        grid.get_point_neighbors(pos)
            .iter()
            .filter_map(|neighbor| {
                grid.get_point_color(neighbor)
                    .map(|pixel| color_distance(pixel, color))
            })
            .min()
            .unwrap_or(usize::MAX)
    }
}

pub struct ColorAndPixelDistPixelFitter;
impl PixelFitter for ColorAndPixelDistPixelFitter {
    fn calculate_fit(
        colorspace: &ColorSpace,
        grid: &PixelGrid,
        pos: &Point,
        color: &Color,
    ) -> usize {
        let start_pos = &colorspace.start_pos;
        grid.get_point_neighbors(pos)
            .iter()
            .filter_map(|neighbor| {
                grid.get_point_color(neighbor)
                    .map(|pixel| color_distance(pixel, color) + point_distance(start_pos, neighbor))
            })
            .min()
            .unwrap_or(usize::MAX)
    }
}
