
pub fn round(target: f64, precision: u32) -> f64 {
    let r = u32::pow(10, precision) as f64;
    return (target * r).round() / r;
}
