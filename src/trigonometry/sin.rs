/// Calculates the sine of an angle specified in radians.
///
/// Given an angle in radians (`radians`), this function returns the sine value
/// as a `f64` floating-point number.
///
/// # Arguments
///
/// * `radians` - The angle in radians for which the sine value is calculated.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::trigonometry::sin;
///
/// let radians = 1.0;
/// let sine = sin(radians);
/// println!("Sine of {} radians is: {}", radians, sine);
/// ```
///
/// # Remarks
///
/// The `sin` function internally uses the sine method of the `f64` type, which returns
/// an approximation of the sine value. The returned value is cast to `f64` to ensure
/// consistency in the function's return type.
///
/// It's important to note that due to the finite precision of floating-point arithmetic,
/// the returned sine value may contain a slight error. Therefore, it is recommended to
/// consider the approximate nature of floating-point calculations when using the result
/// for critical operations.
pub fn sin(radians: f64) -> f64 {
    radians.sin()
}

/// Calculates the arcsine of a value in radians.
///
/// Given a value (`radians`), this function returns the arcsine value in radians
/// as a `f64` floating-point number.
///
/// # Arguments
///
/// * `radians` - The value for which the arcsine is calculated.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::trigonometry::asin;
///
/// let value = 0.5;
/// let arc_sine = asin(value);
/// println!("Arcsine of {} is: {}", value, arc_sine);
/// ```
///
/// # Remarks
///
/// The `asin` function internally uses the arcsine method of the `f64` type, which
/// returns the arc arcsine value in radians. The result is returned as a `f64` to maintain
/// consistency in the function's return type.
///
/// It's important to note that the input value must be within the range [-1.0, 1.0]. If the
/// input value falls outside this range, the function may produce NaN (Not-a-Number) as the result.
///
/// The returned arcsine value represents the angle whose sine is equal to the given input value.
/// The result is in radians, which is the standard unit of measurement for trigonometric functions.
pub fn asin(radians: f64) -> f64 {
    radians.asin()
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

    #[test]
    fn asin_test() {
        assert_eq!(asin(1.0), 1.5707963267948966);
        assert_eq!(asin(0.71), 0.7894982093461719);
        assert_eq!(asin(0.11), 0.11022304998774664);
    }
}