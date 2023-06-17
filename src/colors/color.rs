#![allow(dead_code, unused_imports)]

use bmp::Pixel;
use std::cmp::Ordering;
use std::fmt::*;

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Color {
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };

    pub const fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    /// [Wikipedia](https://en.wikipedia.org/wiki/Color_difference)
    /// Euclidean distance
    pub fn euclidean_distance(&self, other: &Color) -> f32 {
        let red_dist = u8::abs_diff(self.red, other.red) as usize;
        let green_dist = u8::abs_diff(self.green, other.green) as usize;
        let blue_dist = u8::abs_diff(self.blue, other.blue) as usize;
        let dist =
            ((red_dist * red_dist) + (green_dist * green_dist) + (blue_dist * blue_dist)) as f32;
        f32::sqrt(dist)
    }
    /// Euclidean distance (simple)
    pub const fn dist(&self, other: &Color) -> usize {
        let red_dist = u8::abs_diff(self.red, other.red) as usize;
        let green_dist = u8::abs_diff(self.green, other.green) as usize;
        let blue_dist = u8::abs_diff(self.blue, other.blue) as usize;
        (red_dist * red_dist) + (green_dist * green_dist) + (blue_dist * blue_dist)
    }
    pub fn redmean(&self, other: &Color) -> f32 {
        let red_dist = u8::abs_diff(self.red, other.red) as usize;
        let green_dist = u8::abs_diff(self.green, other.green) as usize;
        let blue_dist = u8::abs_diff(self.blue, other.blue) as usize;
        let r = 0.5 * (self.red as f32 + other.red as f32);
        let red_part = (2.0 + (r / 256.0)) * ((red_dist * red_dist) as f32);
        let green_part = 4.0 * ((green_dist * green_dist) as f32);
        let blue_part = (2.0 + ((255.0 - r) / 256.0)) * ((blue_dist * blue_dist) as f32);
        f32::sqrt(red_part + green_part + blue_part)
    }

    /// fast get Hue
    pub fn hue(&self) -> usize {
        let (r, g, b) = (self.red, self.green, self.blue);
        let min = r.min(g.min(b));
        let max = r.max(g.max(b));

        if min == max {
            return 0;
        }

        let range = (max - min) as f32;
        let mut hue: f32;

        if max == self.red {
            hue = (g as f32 - b as f32) / range;
        } else if max == self.green {
            hue = 2.0 + ((b as f32 - r as f32) / range);
        } else if max == self.blue {
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

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[{},{},{}]", self.red, self.green, self.blue)
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl LowerHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

impl From<Pixel> for Color {
    fn from(pixel: Pixel) -> Self {
        Color::new(pixel.r, pixel.g, pixel.b)
    }
}
impl From<Color> for Pixel {
    fn from(color: Color) -> Self {
        Pixel::new(color.red, color.green, color.blue)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        //let a = (value << 24) as u8;
        let r = (value << 16) as u8;
        let g = (value << 8) as u8;
        let b = (value << 0) as u8;
        Color::new(r, g, b)
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        let mut value: u32 = 0;
        //value |= (self.alpha as u32) << 24;
        value |= (self.red as u32) << 16;
        value |= (self.green as u32) << 8;
        value |= self.blue as u32;
        value
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist(&Color::WHITE)
            .partial_cmp(&other.dist(&Color::WHITE))
    }
}
impl Ord for Color {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist(&Color::WHITE).cmp(&other.dist(&Color::WHITE))
    }
}
