/// Calculates the cosine of an angle specified in radians.
///
/// Given an angle in radians (`radians`), this function returns the cosine value
/// as a `f64` floating-point number.
///
/// # Arguments
///
/// * `radians` - The angle in radians for which the cosine value is calculated.
///
/// # Examples
///
/// ```
/// use eratosthenes::trigonometry::cos;
///
/// let radians = 1.0;
/// let cosine = cos(radians);
/// println!("Cosine of {} radians is: {}", radians, cosine);
/// ```
///
/// # Remarks
///
/// The `cos` function internally uses the cosine method of the `f64` type, which returns
/// an approximation of the cosine value. The returned value is cast to `f64` to ensure
/// consistency in the function's return type.
///
/// It's important to note that due to the finite precision of floating-point arithmetic,
/// the returned cosine value may contain a slight error. Therefore, it is recommended to
/// consider the approximate nature of floating-point calculations when using the result
/// for critical operations.
pub fn cos(radians: f64) -> f64 {
    radians.cos()
}

pub fn acos(radians: f64) -> f64 {
    radians.acos()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cos_test() {
        assert_eq!(cos(10.0), -0.8390715290764524);
        assert_eq!(cos(22.8), -0.6903298762015725);
        assert_eq!(cos(0.0), 1.0);
    }

    #[test]
    fn acos_test() {
        assert_eq!(acos(0.1), 1.4706289056333368);
        assert_eq!(acos(0.3), 1.266103672779499);
        assert_eq!(acos(-0.2), 1.7721542475852274);
    }
}