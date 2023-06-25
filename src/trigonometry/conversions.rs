pub fn rad_to_deg(radian: f64) -> f64 {
    radian.to_degrees()
}

pub fn deg_to_rad(degree: f64) -> f64 {
    degree.to_radians()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rad_to_deg_test() {
        assert_eq!(rad_to_deg(10.0), 572.9577951308232);
    }

    #[test]
    fn deg_to_rad_test() {
        assert_eq!(deg_to_rad(10.0), 0.17453292519943295);
    }

}