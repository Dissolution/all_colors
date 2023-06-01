use crate::color::Color;
use crate::point::{Point, Size};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorSpace {
    pub colors: [ColorSelection; 3],
    pub width: usize,
    pub height: usize,
    pub start_pos: Point,
    pub default_color: Color,
}

impl ColorSpace {
    fn get_factors(n: usize) -> Vec<(usize, usize)> {
        let upper_bound = f64::floor(f64::sqrt(n as f64)) as usize;
        let mut factors = Vec::with_capacity(upper_bound);
        for i in 1..=upper_bound {
            if n % i == 0 {
                let f = (i, n / i);
                factors.push(f);
            }
        }
        factors
    }

    pub fn new(colors: [ColorSelection; 3], width: usize, height: usize) -> Self {
        assert_eq!(width * height, Self::colors_count(&colors));
        assert!(width <= 4096);
        assert!(height <= 4096);
        Self {
            colors,
            width,
            height,
            start_pos: Point::new(width / 2, height / 2),
            default_color: Color::default(),
        }
    }

    fn colors_count(colors: &[ColorSelection; 3]) -> usize {
        colors[0].count * colors[1].count * colors[2].count
    }

    pub fn closest_square(depths: [ColorSelection; 3]) -> ColorSpace {
        let area = depths[0].count * depths[1].count * depths[2].count;
        let perfect_side = f64::cbrt(area as f64) as usize;
        let mut factors = ColorSpace::get_factors(area);
        let min = factors.into_iter().min_by_key(|&f| {
            usize::abs_diff(perfect_side, f.0) + usize::abs_diff(perfect_side, f.1)
        });
        let (width, height) = min.expect("must be at least one factor");
        ColorSpace::new(depths, width, height)
    }

    pub fn closest_fit(width: usize, height: usize) -> ColorSpace {
        let area = width * height;
        let mut depth = f64::floor(f64::cbrt(area as f64)) as usize;
        assert!(depth <= u8::MAX as usize);
        let mut pixel_count;
        let mut factors;
        let mut best_fit;
        loop {
            pixel_count = depth * depth * depth;
            factors = ColorSpace::get_factors(pixel_count);
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
                    depth -= 1;
                    continue;
                }
                Some(fit) => {
                    return ColorSpace::new(
                        [ColorSelection::new(depth, ColorRange::Spaced); 3],
                        fit.0,
                        fit.1,
                    );
                }
            }
        }
    }

    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn generate_colors(&self) -> Vec<Color> {
        let red = self.colors[0];
        let green = self.colors[1];
        let blue = self.colors[2];
        let total = red.count * green.count * blue.count;
        let mut colors = Vec::with_capacity(total);
        for r in red.get_values().into_iter() {
            for g in green.get_values().into_iter() {
                for b in blue.get_values().into_iter() {
                    let color = Color::new(r, g, b);
                    colors.push(color);
                }
            }
        }
        assert_eq!(colors.len(), total);
        colors
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ColorSelection {
    count: usize,
    range: ColorRange,
}

impl ColorSelection {
    pub fn new(count: usize, range: ColorRange) -> Self {
        assert!(
            (1..=256).contains(&count),
            "Count must be between 1 and 256"
        );
        ColorSelection { count, range }
    }

    pub fn get_values(&self) -> Vec<u8> {
        let mut values = Vec::with_capacity(self.count);
        match self.range {
            ColorRange::Spaced => {
                let count: f32 = self.count as f32;
                let mut step: f32 = 0.0;
                while step < count {
                    let value = (step / count) * 255.0;
                    values.push(value as u8);
                    step += 1.0;
                }
                values
            }
            ColorRange::Dark => {
                for value in 0..self.count {
                    values.push(value as u8);
                }
                values
            }
            ColorRange::Light => {
                for value in 0..self.count {
                    values.push(255 - value as u8);
                }
                values
            }
            ColorRange::Random => {
                unimplemented!()
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorRange {
    Spaced,
    Dark,
    Light,
    Random,
}
