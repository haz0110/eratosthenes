/// Generates a vector of square numbers for a given number of squares.
///
/// Given an unsigned integer `number_of_squares`, this function generates and returns a vector
/// containing the square numbers of consecutive integers starting from 0 up to `number_of_squares - 1`.
///
/// # Arguments
///
/// * `number_of_squares` - An unsigned integer representing the number of squares to generate.
///
/// # Examples
///
/// ```
/// use eratosthenes::sequences::square_numbers;
///
/// let number_of_squares = 10;
/// let squares = square_numbers(number_of_squares);
///
/// assert_eq!(squares, vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
/// ```
pub fn square_numbers(number_of_squares: usize) -> Vec<usize> {
    (0..number_of_squares).map(|index| index * index).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn square_numbers_test() {
        assert_eq!(square_numbers(10), vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }
}
