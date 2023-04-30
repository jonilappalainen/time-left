
pub fn round(target: f64, precision: u32) -> f64 {
    let r = u32::pow(10, precision) as f64;
    return (target * r).round() / r;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_more_significant_numbers_then_rounds_up() {
        assert_eq!(1.116, round(1.1156, 3));
    }

    #[test]
    fn given_less_significant_numbers_then_returns_correct_value() {
        assert_eq!(1.1, round(1.1, 3));
    }

    #[test]
    fn given_more_significant_numbers_then_rounds_down() {
        assert_eq!(0.123, round(0.1234, 3));
    }
}
