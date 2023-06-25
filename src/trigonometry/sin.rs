pub fn sin(radians: f64) -> f64 {
    radians.sin() as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sin_test() {
        assert_eq!(sin(10.0), -0.5440211108893698);
        assert_eq!(sin(22.8), -0.723494756044245);
        assert_eq!(sin(0.0), 0.0);
    }
}