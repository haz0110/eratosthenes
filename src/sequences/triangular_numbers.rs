/// /// Generates a vector of triangular numbers up to a given count.
///
/// # Arguments
///
/// * `number_of_triangulars` - A usize value that indicates the number of triangular numbers to generate.
///
/// # Panics
///
/// Panics if the `number_of_triangulars` is less than 1.
///
/// # Example
///
/// ```
/// use eratosthenes::sequences::triangular_numbers;
/// let t_nums: Vec<usize> = triangular_numbers(5);
/// assert_eq!(t_nums, vec![0, 1, 3, 6, 10]);
/// ```
pub fn triangular_numbers(number_of_triangulars: usize) -> Vec<usize> {
    if number_of_triangulars < 1 {
        panic!("You may request 1 or more triangular numbers.");
    }

    (0..number_of_triangulars)
        .map(|n| n * (n + 1) / 2)
        .collect()
}

/// Returns the nth triangular number.
///
/// # Arguments
///
/// * `nth` - A usize value that indicates the nth triangular number.
///
/// # Example
///
/// ```
/// use eratosthenes::sequences::nth_triangular;
/// let fifth_triangular = nth_triangular(5);
/// assert_eq!(fifth_triangular, 10);
/// ```
pub fn nth_triangular(nth: usize) -> usize {
    nth * (nth - 1) / 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn triangular_numbers_test() {
        assert_eq!(triangular_numbers(1), vec![0]);
        assert_eq!(triangular_numbers(2), vec![0, 1]);
        assert_eq!(
            triangular_numbers(10),
            vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45]
        );
    }

    #[test]
    fn nth_triangular_test() {
        assert_eq!(nth_triangular(1), 0);
        assert_eq!(nth_triangular(2), 1);
        assert_eq!(nth_triangular(3), 3);
        assert_eq!(nth_triangular(4), 6);
    }
}