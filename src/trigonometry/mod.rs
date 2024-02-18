use crate::is_valid_triangle;

/// Calculates the length of the third side of a triangle using the Law of Cosines.
///
/// The Law of Cosines states: c^2 = \sqrt{a^2 + b^2 - 2ab \cos(\gamma)}
///
/// # Arguments
///
/// * `a` - Length of side 'a'.
/// * `b` - Length of side 'b'.
/// * `gamma` - Angle opposite to side 'c' in radians.
///
/// # Errors
///
/// Returns an `Err` variant if any of the input values is
/// negative or if they do not form a valid triangle.
///
/// # Examples
///
/// ```
/// use eratosthenes::law_of_cosines;
///
/// // Example with sides of length 3, 4, and angle opposite
/// // to side 'c' being 60 degrees.
/// let result = law_of_cosines(3.0, 4.0, 60.0_f64.to_radians());
/// assert!(result.is_ok());
/// let length_c = result.unwrap();
/// ```
pub fn law_of_cosines(a: f64, b: f64, gamma: f64) -> Result<f64, &'static str> {
    if a < 0.0 || b < 0.0 || gamma < 0.0 {
        return Err("Input values cannot be negative.");
    }
    

    let c_squared = a.powi(2) + b.powi(2) - 2.0 * a * b * gamma.cos();

    if is_valid_triangle(a, b, c_squared.sqrt()) == false {
        return Err("The input values do not form a valid triangle.");
    }

    if c_squared.sqrt().is_sign_negative() {
        return Err("c cannot be negative.");
    }

    Ok(c_squared.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_triangle_test() {
        // Valid triangle with sides of length 3, 4, and angle opposite to side 'c' being 60 degrees.
        let c = law_of_cosines(3.0, 4.0, 60.0_f64.to_radians());
        assert_eq!(crate::reduce_decimals(c.unwrap(), 1), 3.6);
    }

    #[test]
    fn negative_side_a_test() {
        // Negative side 'a' should return an error.
        let result = law_of_cosines(-3.0, 4.0, 30.0_f64.to_radians());
        assert_eq!(result, Err("Input values cannot be negative."));
    }

    #[test]
    fn negative_side_b_test() {
        // Negative side 'b' should return an error.
        let result = law_of_cosines(3.0, -4.0, 45.0_f64.to_radians());
        assert_eq!(result, Err("Input values cannot be negative."));
    }

    #[test]
    fn negative_gamma_test() {
        // Negative angle 'gamma' should return an error.
        let result = law_of_cosines(5.0, 4.0, -30.0_f64.to_radians());
        assert_eq!(result, Err("Input values cannot be negative."));
    }

    #[test]
    fn invalid_triangle_test() {
        // Invalid triangle with sides of length 1, 1, and angle opposite to side 'c' being 180 degrees.
        let result = law_of_cosines(1.0, 1.0, 180.0_f64.to_radians());
        assert_eq!(
            result,
            Err("The input values do not form a valid triangle.")
        );
    }
}
