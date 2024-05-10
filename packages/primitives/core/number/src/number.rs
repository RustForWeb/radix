pub fn clamp(value: f64, [min, max]: [f64; 2]) -> f64 {
    value.max(min).min(max)
}
