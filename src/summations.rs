/// Calculates the sum of even numbers in an array.
///
/// Given an array of unsigned integers (`array`), this function calculates and returns the sum of
/// all even numbers in the array.
///
/// # Arguments
///
/// * `array` - A reference to a slice of unsigned integers. The array to calculate the sum from.
///
/// # Examples
///
/// ```
/// use eratosthenes::sum_even;
///
/// let input = vec![1, 2, 3, 4, 5, 6];
/// let sum = sum_even(&input);
/// assert_eq!(sum, 12);
/// ```
pub fn sum_even(array: &Vec<usize>) -> usize {
    array.iter().filter(|&item| item % 2 == 0).sum()
}

/// Calculates the sum of odd numbers in an array.
///
/// Given an array of unsigned integers (`array`), this function
/// calculates and returns the sum of all odd numbers in the array.
///
/// # Arguments
///
/// * `array` - A reference to a slice of unsigned integers. The array to calculate the sum from.
///
/// # Examples
///
/// ```
/// use eratosthenes::sum_odd;
///
/// let input = vec![1, 2, 3, 4, 5, 6];
/// let sum = sum_odd(&input);
/// assert_eq!(sum, 9);
/// ```
pub fn sum_odd(array: &Vec<usize>) -> usize {
    array.iter().filter(|&item| item % 2 == 1).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sum_even_test() {
        assert_eq!(sum_even(&vec![1, 3, 6, 11]), 6)
    }

    #[test]
    fn sum_odd_test() {
        assert_eq!(sum_odd(&vec![1, 3, 6, 11]), 15)
    }
}
