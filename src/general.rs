/// Checks if the given side lengths form a valid triangle according to the triangle inequality theorem.
/// 
/// The triangle inequality theorem states that the sum of the lengths of any two sides of a triangle 
/// must be greater than the length of the remaining side.
/// 
/// # Arguments
///
/// * `a` - Length of side a.
/// * `b` - Length of side b.
/// * `c` - Length of side c.
///
/// # Returns
///
/// Returns `true` if the side lengths form a valid triangle, and `false` otherwise.
///
/// # Examples
///
/// ```
/// use eratosthenes::is_valid_triangle;
///
/// // Valid triangle with sides of length 3, 4, and 5.
/// assert!(is_valid_triangle(3.0, 4.0, 5.0));
///
/// // Invalid triangle with sides of length 1, 1, and 10.
/// assert!(!is_valid_triangle(1.0, 1.0, 10.0));
/// ```
pub fn is_valid_triangle(a: f64, b: f64, c: f64) -> bool {
    a + b > c && b + c > a && c + a > b
}

pub fn reduce_decimals(value: f64, decimal_places: i64) -> f64 {
    let multiplier = 10u64.pow(decimal_places as u32);
    let rounded_value = (value * multiplier as f64).round() / multiplier as f64;
    rounded_value
}

pub fn template_function() -> Result<Vec<Vec<usize>>, &'static str> {

    let result = vec![vec![0; 10]];

    if result.is_empty() {
        return Err("No result.");
    }
    
    return Ok(result);
}