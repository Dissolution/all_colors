use crate::prelude::*;

pub fn get_factors(n: usize) -> Vec<(usize, usize)> {
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
pub fn get_squared_up_factors(n: usize) -> Vec<(usize, usize)> {
    let mut factors = get_factors(n);
    let perfect_side = f32::sqrt(n as f32) as usize;
    factors
        .sort_by_key(|&f| usize::abs_diff(perfect_side, f.0) + usize::abs_diff(perfect_side, f.1));
    factors
}

pub fn closest_square(area: usize) -> Option<Size> {
    let factors = get_squared_up_factors(area);
    let (width, height) = factors.first()?;
    Some(Size::new(*width, *height))
}
