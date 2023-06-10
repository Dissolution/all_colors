use crate::colors::*;
use crate::grid::*;
use crate::math;

pub struct ColorSpaceGenerator {
    pub red_channel: ColorChannel,
    pub green_channel: ColorChannel,
    pub blue_channel: ColorChannel,
}
impl ColorSpaceGenerator {
    pub fn new(
        red_channel: ColorChannel,
        blue_channel: ColorChannel,
        green_channel: ColorChannel,
    ) -> Self {
        ColorSpaceGenerator {
            red_channel,
            green_channel,
            blue_channel,
        }
    }
    pub fn equal_channels(channel: ColorChannel) -> Self {
        ColorSpaceGenerator {
            red_channel: channel,
            green_channel: channel,
            blue_channel: channel,
        }
    }

    pub fn closest_equal_channel_fit(size: Size) -> Option<(ColorSpaceGenerator, Size)> {
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
                    let cc = ColorChannel::new(depth as u8, ColorChannelSpread::Spaced);
                    let gen = ColorSpaceGenerator::equal_channels(cc);
                    let size = Size::new(fit.0, fit.1);
                    return Some((gen, size));
                }
            }
        }
    }

    pub fn color_count(&self) -> usize {
        (self.red_channel.value as usize)
            * (self.green_channel.value as usize)
            * (self.blue_channel.value as usize)
    }
}
impl ColorsGenerator for ColorSpaceGenerator {
    fn get_colors(&self) -> Vec<Color> {
        let total = self.color_count();
        let mut colors = Vec::with_capacity(total);
        for r in self.red_channel.get_values().into_iter() {
            for g in self.green_channel.get_values().into_iter() {
                for b in self.blue_channel.get_values().into_iter() {
                    let color = Color::new(r, g, b);
                    colors.push(color);
                }
            }
        }
        assert_eq!(colors.len(), total);
        colors
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ColorChannel {
    value: u8,
    spread: ColorChannelSpread,
}
impl ColorChannel {
    pub fn find_min_max_depths(size: Size) -> Option<(u8, u8)> {
        let area = (size.area()) as f32;
        for x in (0_usize..=255).rev() {
            let side = f32::sqrt(area / x as f32);
            if side.fract() == 0.0 {
                return Some((side as u8, x as u8));
            }
        }
        None
    }

    pub fn find_min_mid_max_depths(size: Size) -> Option<(u8, u8, u8)> {
        let area = size.area() as f32;
        for max in (0_usize..=255).rev() {
            let rem_area = area / max as f32;
            if rem_area.fract() == 0.0 {
                let factors = math::get_squared_up_factors(rem_area as usize);
                if let Some((short, long)) = factors.get(0) {
                    return Some((*short as u8, *long as u8, max as u8));
                }
            }
        }
        None
    }

    pub fn new(value: u8, spread: ColorChannelSpread) -> Self {
        ColorChannel { value, spread }
    }
    pub fn spaced(value: u8) -> Self {
        ColorChannel {
            value,
            spread: ColorChannelSpread::Spaced,
        }
    }
    pub fn get_values(&self) -> Vec<u8> {
        let mut values = Vec::with_capacity(self.value as usize);
        match self.spread {
            ColorChannelSpread::Spaced => {
                let count: f32 = self.value as f32;
                let mut step: f32 = 0.0;
                while step < count {
                    let value = (step / count) * 255.0;
                    values.push(value as u8);
                    step += 1.0;
                }
                values
            }
            ColorChannelSpread::Dark => {
                for value in 0..self.value {
                    values.push(value);
                }
                values
            }
            ColorChannelSpread::Light => {
                for value in 0..self.value {
                    values.push(255 - value as u8);
                }
                values
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ColorChannelSpread {
    Spaced,
    Light,
    Dark,
}
