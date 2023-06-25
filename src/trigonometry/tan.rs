/// Calculates the tangent of a value in radians.
///
/// Given a value (`radians`), this function returns the tangent value in radians
/// as a `f64` floating-point number.
///
/// # Arguments
///
/// * `radians` - The value for which the tangent is calculated.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::trigonometry::tan;
///
/// let value = 0.8;
/// let tangent = tan(value);
/// println!("Tangent of {} is: {}", value, tangent);
/// ```
///
/// # Remarks
///
/// The `tan` function internally uses the tangent method of the `f64` type, which
/// returns the tangent value in radians. The result is returned as a `f64` to maintain
/// consistency in the function's return type.
///
/// It's important to note that the tangent function may produce NaN (Not-a-Number) or
/// infinity as the result if the input value causes division by zero or exceeds the range
/// of representable values in the `f64` type.
///
/// The returned tangent value represents the ratio of the sine to the cosine of the given
/// input value. The result is in radians, which is the standard unit of measurement for
/// trigonometric functions.
pub fn tan(radians: f64) -> f64 {
    radians.tan()
}

/// Calculates the arc tangent of a value in radians.
///
/// Given a value (`radians`), this function returns the arc tangent value in radians
/// as a `f64` floating-point number.
///
/// # Arguments
///
/// * `radians` - The value for which the arc tangent is calculated.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::trigonometry::atan;
///
/// let value = 1.0;
/// let arc_tangent = atan(value);
/// println!("Arc tangent of {} is: {}", value, arc_tangent);
/// ```
///
/// # Remarks
///
/// The `atan` function internally uses the arc tangent method of the `f64` type, which
/// returns the arc tangent value in radians. The result is returned as a `f64` to maintain
/// consistency in the function's return type.
///
/// The returned arc tangent value represents the angle whose tangent is equal to the given
/// input value. The result is in radians, which is the standard unit of measurement for
/// trigonometric functions. The range of the arc tangent value is [-π/2, π/2].
pub fn atan(radians: f64) -> f64 {
    radians.atan()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tan_test() {
        assert_eq!(tan(10.0), 0.6483608274590867);
        assert_eq!(tan(22.8), 1.0480420752251909);
        assert_eq!(tan(0.0), 0.0);
    }

    #[test]
    fn atan_test() {
        assert_eq!(atan(1.0), 0.7853981633974483);
        assert_eq!(atan(0.71), 0.6174058917515727);
        assert_eq!(atan(0.11), 0.10955952677394434);
    }
}