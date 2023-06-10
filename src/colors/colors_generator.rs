use crate::colors::*;
use rand::prelude::*;
use std::ops::*;

pub trait ColorsGenerator {
    fn get_colors(&self) -> Vec<Color>;
}

pub struct RandColorsGenerator {
    pub count: Range<usize>,
}
impl RandColorsGenerator {
    pub fn new(count: Range<usize>) -> Self {
        RandColorsGenerator { count }
    }
}
impl ColorsGenerator for RandColorsGenerator {
    fn get_colors(&self) -> Vec<Color> {
        let mut rng = rand::thread_rng();
        let max = (self.count.end - self.count.start) + 1;
        let to_generate = rng.gen_range(0..max);
        let mut colors = Vec::with_capacity(to_generate);
        for r in 0..=to_generate {
            let color_value = rng.gen_range(0..Color::TOTAL_COUNT);
            let color = Color::try_from(color_value as u32).expect("why?");
            colors.push(color);
        }

        colors
    }
}
