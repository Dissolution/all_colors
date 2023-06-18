use crate::prelude::*;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

pub trait ColorSorter: Display {
    fn sort_colors(&self, colors: &mut [Color]);
}

pub struct FnColorSorter {
    func: fn(&mut [Color]),
}
impl FnColorSorter {
    pub fn new(func: fn(&mut [Color])) -> Self {
        FnColorSorter { func }
    }
}
impl Display for FnColorSorter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "FnColorSorter")
    }
}
impl ColorSorter for FnColorSorter {
    fn sort_colors(&self, colors: &mut [Color]) {
        (self.func)(colors)
    }
}

pub struct RandColorSorter {
    pub seed: u64,
}
impl RandColorSorter {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}
impl Display for RandColorSorter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "RandColorSorter(seed: {})", self.seed)
    }
}
impl ColorSorter for RandColorSorter {
    fn sort_colors(&self, colors: &mut [Color]) {
        let mut rand = StdRng::seed_from_u64(self.seed);
        colors.shuffle(&mut rand);
    }
}

pub struct HueColorSorter;
impl HueColorSorter {
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
impl Display for HueColorSorter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "HueColorSorter")
    }
}
impl ColorSorter for HueColorSorter {
    fn sort_colors(&self, pixels: &mut [Color]) {
        pixels.sort_by_key(HueColorSorter::fast_hue)
    }
}

pub struct NoSorter;
impl Display for NoSorter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "NoSorter")
    }
}
impl ColorSorter for NoSorter {
    fn sort_colors(&self, _: &mut [Color]) {
        // do nothing
    }
}
