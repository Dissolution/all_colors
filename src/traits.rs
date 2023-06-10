use crate::color::Color;
use rand::prelude::*;

pub trait ColorSorter {
    fn sort_colors(&mut self, colors: &mut [Color]);
    //fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T>;
}

pub struct FnColorSorter {
    func: fn(&mut [Color]),
}
impl FnColorSorter {
    pub fn new(func: fn(&mut [Color])) -> Self {
        FnColorSorter { func }
    }
}
impl ColorSorter for FnColorSorter {
    fn sort_colors(&mut self, colors: &mut [Color]) {
        (self.func)(colors)
    }
}

pub struct RngPixelShuffler<R>
where
    R: Rng,
{
    rng: R,
}

impl<R> RngPixelShuffler<R>
where
    R: Rng,
{
    #[allow(dead_code)]
    pub fn new(rng: R) -> Self {
        RngPixelShuffler { rng }
    }
}

impl<R> ColorSorter for RngPixelShuffler<R>
where
    R: Rng,
{
    fn sort_colors(&mut self, color: &mut [Color]) {
        color.shuffle(&mut self.rng)
    }
}

pub struct HuePixelSorter;

impl HuePixelSorter {
    pub fn fast_hue(color: &Color) -> usize {
        let (r, g, b) = (color.red, color.green, color.blue);
        let min = r.min(g.min(b));
        let max = r.max(g.max(b));

        if min == max {
            return 0;
        }

        let range = (max - min) as f32;
        let mut hue: f32;

        if max == r {
            hue = (g as f32 - b as f32) / range;
        } else if max == g {
            hue = 2.0 + ((b as f32 - r as f32) / range);
        } else if max == b {
            hue = 4.0 + ((r as f32 - g as f32) / range);
        } else {
            unreachable!();
        }

        hue *= 60.0;
        while hue < 0.0 {
            hue += 360.0;
        }

        f32::round(hue) as usize
    }
}

impl ColorSorter for HuePixelSorter {
    fn sort_colors(&mut self, pixels: &mut [Color]) {
        pixels.sort_by_key(HuePixelSorter::fast_hue)
    }
}

pub struct NoSorter {}
impl ColorSorter for NoSorter {
    fn sort_colors(&mut self, _: &mut [Color]) {
        // do nothing
    }
}
