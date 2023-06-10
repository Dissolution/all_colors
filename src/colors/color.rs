use bmp::Pixel;
use std::fmt::*;

#[derive(Debug, Default, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl Color {
    #[allow(dead_code)]
    pub const WHITE: Color = Color {
        red: 255,
        green: 255,
        blue: 255,
    };
    #[allow(dead_code)]
    pub const BLACK: Color = Color {
        red: 0,
        green: 0,
        blue: 0,
    };
    #[allow(dead_code)]
    pub const MAX_DIST: usize = Color::WHITE.dist(&Color::BLACK);
    pub const TOTAL_COUNT: usize = (256 * 256 * 256);

    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color { red, green, blue }
    }

    pub const fn dist(&self, other: &Color) -> usize {
        let red_dist = u8::abs_diff(self.red, other.red) as usize;
        let green_dist = u8::abs_diff(self.green, other.green) as usize;
        let blue_dist = u8::abs_diff(self.blue, other.blue) as usize;
        (red_dist * red_dist) + (green_dist * green_dist) + (blue_dist * blue_dist)
    }

    #[allow(dead_code)]
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
    fn from(value: Pixel) -> Self {
        Color {
            red: value.r,
            green: value.g,
            blue: value.b,
        }
    }
}
impl Into<Pixel> for Color {
    fn into(self) -> Pixel {
        Pixel {
            r: self.red,
            g: self.green,
            b: self.blue,
        }
    }
}

impl TryFrom<u32> for Color {
    type Error = ();

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        let a = (value << 24) as u8;
        if a != 0 {
            return Err(());
        }
        let r = (value << 16) as u8;
        let g = (value << 8) as u8;
        let b = (value << 0) as u8;
        Ok(Color::new(r, g, b))
    }
}
impl Into<u32> for Color {
    fn into(self) -> u32 {
        let mut value: u32 = 0;
        value |= (self.red as u32) << 16;
        value |= (self.green as u32) << 8;
        value |= self.blue as u32;
        value
    }
}
