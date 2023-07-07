use crate::prelude::*;
use crate::grid::{Cell, Grid};
use std::fmt::{Display, Formatter, Result};

#[inline(always)]
fn color_distance(first: &Color, second: &Color) -> usize {
    // 'true' implementation would return the square root of the below
    let r_diff = u8::abs_diff(first.red, second.red) as usize;
    let g_diff = u8::abs_diff(first.green, second.green) as usize;
    let b_diff = u8::abs_diff(first.blue, second.blue) as usize;
    (r_diff * r_diff) + (g_diff * g_diff) + (b_diff * b_diff)
}

#[inline(always)]
fn point_distance(start_pos: &Point, pos: &Point) -> usize {
    let x_diff = usize::abs_diff(start_pos.x, pos.x);
    let y_diff = usize::abs_diff(start_pos.y, pos.y);
    (x_diff * x_diff) + (y_diff * y_diff)
    //(x_diff + (y_diff * x_diff))
    //x_diff * y_diff
}

pub trait PixelFitter: Display {
    fn calculate_fit(&self, grid: &Grid, pos: &Point, color: &Color) -> usize;
}

pub struct ColorDistPixelFitter;
impl Display for ColorDistPixelFitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CDPF")
    }
}
impl PixelFitter for ColorDistPixelFitter {
    fn calculate_fit(&self, grid: &Grid, pos: &Point, color: &Color) -> usize {
        grid.get_neighbor_colors(pos)
            .iter()
            .map(|c| color_distance(color, &c))
            .min()
            .unwrap_or(usize::MAX)
    }
}

pub struct ColorAndPixelDistPixelFitter {
    pub start_pos: Point,
}
impl Display for ColorAndPixelDistPixelFitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CAPDPF:{}", self.start_pos)
    }
}
impl ColorAndPixelDistPixelFitter {
    pub fn new(start_pos: Point) -> Self {
        Self { start_pos }
    }
}
impl PixelFitter for ColorAndPixelDistPixelFitter {
    fn calculate_fit(&self, grid: &Grid, pos: &Point, color: &Color) -> usize {
        let start_pos = self.start_pos;
        grid.get_neighbors(pos)
            .iter()
            .filter_map(|nc| {
                if let Some(cell_color) = nc.color {
                    let dist = color_distance(&cell_color, color)
                        + point_distance(&start_pos, &nc.position);
                    Some(dist)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(usize::MAX)
    }
}
