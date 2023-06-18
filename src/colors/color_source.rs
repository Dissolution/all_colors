#![allow(unused, dead_code)]

use crate::prelude::*;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

pub trait ColorSource: Display {
    fn get_colors(&self) -> Vec<Color>;
}

// RandColorSource

pub struct RandColorSource {
    pub seed: u64,
    pub count: usize,
}
impl RandColorSource {
    pub fn new(seed: u64, count: usize) -> Self {
        Self { seed, count }
    }
}
impl ColorSource for RandColorSource {
    fn get_colors(&self) -> Vec<Color> {
        let mut colors = Vec::new();
        let mut rand = StdRng::seed_from_u64(self.seed);
        for _ in 0..self.count {
            let r = rand.next_u32();
            let color = r.into();
            colors.push(color);
        }
        // overkill!
        //colors.shuffle(&mut rand);
        colors
    }
}
impl Display for RandColorSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "RandColorSource(seed: {}, count: {})",
            self.seed, self.count
        )
    }
}

// ColorChannels

pub struct ColorChannels;
impl ColorChannels {
    pub fn find_equal_counts(size: Size) -> Option<u8> {
        todo!()
    }
    pub fn find_min_min_max_counts(size: Size) -> Option<(u8, u8)> {
        let area = size.area() as f32;
        for max in (u8::MIN..=u8::MAX).rev() {
            let min = f32::sqrt(area / max as f32);
            if min.fract() == 0.0 {
                return Some((min as u8, max));
            }
        }
        None
    }
    pub fn find_min_mid_max_counts(size: Size) -> Option<(u8, u8, u8)> {
        let area = size.area() as f32;
        for max in (u8::MIN..=u8::MAX).rev() {
            let rem_area = area / max as f32;
            if rem_area.fract() == 0.0 {
                let factors = get_squared_up_factors(rem_area as usize);
                if let Some((min, mid)) = factors.first() {
                    return Some((*min as u8, *mid as u8, max));
                }
            }
        }
        None
    }

    pub fn find_closest_equal_counts(size: Size) -> Option<(Size, u8)> {
        let (width, height) = (size.width, size.height);
        let area = size.area();
        let mut depth = f64::floor(f64::cbrt(area as f64)) as usize;
        assert!(depth <= u8::MAX as usize);
        let mut pixel_count;
        let mut factors;
        let mut best_fit;
        loop {
            pixel_count = depth * depth * depth;
            factors = get_factors(pixel_count);
            // Do any fit?
            best_fit = factors
                .iter()
                .map(|f| {
                    if height < width {
                        (f.1, f.0)
                    } else {
                        (f.0, f.1)
                    }
                })
                .filter(|f| f.0 <= width && f.1 <= height)
                .min_by_key(|f| (width - f.0) + height - f.1);

            match best_fit {
                None => {
                    if depth == 0 {
                        return None;
                    } else {
                        depth -= 1;
                        continue;
                    }
                }
                Some(fit) => {
                    let size = Size::new(fit.0, fit.1);
                    return Some((size, depth as u8));
                }
            }
        }
    }
    pub fn find_closest_min_min_max_counts(size: Size) -> Option<(Size, u8)> {
        todo!()
    }
    pub fn find_closest_min_mid_max_counts(size: Size) -> Option<(Size, u8)> {
        todo!()
    }
}

pub struct ColorChannel {
    pub count: u8,
    pub spacing: ChannelSpacing,
}
impl ColorChannel {
    pub fn new(count: u8, spacing: ChannelSpacing) -> Self {
        Self { count, spacing }
    }
    pub fn equidistant(count: u8) -> Self {
        Self {
            count,
            spacing: ChannelSpacing::Equidistant,
        }
    }

    pub fn get_values(&self) -> Vec<u8> {
        let count = self.count;
        let mut values = Vec::with_capacity(count as usize);
        match self.spacing {
            ChannelSpacing::Equidistant => {
                let count: f32 = count as f32;
                let mut step: f32 = 0.0;
                while step < count {
                    let value = (step / count) * 255.0;
                    values.push(value as u8);
                    step += 1.0;
                }
            }
            ChannelSpacing::Light => {
                for v in 0..count {
                    values.push(v);
                }
            }
            ChannelSpacing::Dark => {
                for v in 0..count {
                    values.push(u8::MAX - v);
                }
            }
        }
        values
    }
}
impl Display for ColorChannel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}x{}", self.count, self.spacing)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChannelSpacing {
    Equidistant,
    Light,
    Dark,
}
impl Display for ChannelSpacing {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ChannelSpacing::Equidistant => {
                write!(f, "E")
            }
            ChannelSpacing::Light => {
                write!(f, "L")
            }
            ChannelSpacing::Dark => {
                write!(f, "D")
            }
        }
    }
}

pub struct ColorSpace {
    pub red: ColorChannel,
    pub green: ColorChannel,
    pub blue: ColorChannel,
}
impl ColorSpace {
    pub fn new(red: ColorChannel, green: ColorChannel, blue: ColorChannel) -> Self {
        ColorSpace { red, green, blue }
    }

    pub fn color_count(&self) -> usize {
        (self.red.count as usize) * (self.green.count as usize) * (self.blue.count as usize)
    }
}

impl ColorSource for ColorSpace {
    fn get_colors(&self) -> Vec<Color> {
        let total = self.color_count();
        let mut colors = Vec::with_capacity(total);
        for r in self.red.get_values().into_iter() {
            for g in self.green.get_values().into_iter() {
                for b in self.blue.get_values().into_iter() {
                    let color = Color::new(r, g, b);
                    colors.push(color);
                }
            }
        }
        assert_eq!(colors.len(), total);
        colors
    }
}
impl Display for ColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ColorSpace(red: {}, green: {}, blue: {})",
            self.red, self.green, self.blue
        )
    }
}
