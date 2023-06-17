#![allow(unused, dead_code)]
use crate::prelude::*;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result};

pub trait ColorSpace: Display {
    fn get_colors(&self) -> Vec<Color>;
}

pub struct RandColorSpace {
    pub count: usize,
    colors: Vec<Color>,
}
impl RandColorSpace {
    pub fn new(rand: &mut dyn RngCore, count: usize) -> Self {
        let mut colors = Vec::with_capacity(count);
        for _ in 0..count {
            let u = rand.next_u32();
            colors.push(u.into());
        }
        RandColorSpace { count, colors }
    }
}
impl ColorSpace for RandColorSpace {
    fn get_colors(&self) -> Vec<Color> {
        self.colors.clone()
    }
}
impl Display for RandColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CS[r{}]", self.count)
    }
}

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
                let factors = math::get_squared_up_factors(rem_area as usize);
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
            factors = math::get_factors(pixel_count);
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
        write!(f, "{}{}", self.count, self.spacing)
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

pub struct ChannelColorSpace {
    pub red: ColorChannel,
    pub green: ColorChannel,
    pub blue: ColorChannel,
}
impl ChannelColorSpace {
    pub fn new(red: ColorChannel, green: ColorChannel, blue: ColorChannel) -> Self {
        ChannelColorSpace { red, green, blue }
    }

    pub fn color_count(&self) -> usize {
        (self.red.count as usize) * (self.green.count as usize) * (self.blue.count as usize)
    }
}

impl ColorSpace for ChannelColorSpace {
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
impl Display for ChannelColorSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "CS[{}|{}|{}]", self.red, self.green, self.blue)
    }
}
