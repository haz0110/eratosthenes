/// Calculates the mean value of a vector of unsigned integers.
///
/// Given a vector (`Vec`) of unsigned integers (`array`), this function returns the mean value
/// as a floating-point number (`f64`). The mean is calculated by summing all the elements in the
/// vector and dividing the sum by the number of elements in the vector.
///
/// # Arguments
///
/// * `array` - A reference to a vector (`Vec`) of unsigned integers.
/// The input array for which the mean is calculated.
///
/// # Panics
///
/// This function will panic if the input `array` is empty.
///
/// # Examples
///
/// ```
/// use eratosthenes::mean_usize;
///
/// let input = vec![1, 2, 3, 4, 5];
/// let mean = mean_usize(&input);
/// assert_eq!(mean, 3.0);
/// ```
pub fn mean_usize(array: &Vec<usize>) -> f64 {
    if array.is_empty() {
        panic!("eratosthenes::mean_usize cannot work on an empty array.");
    }

    let sum: usize = array.iter().sum();
    let mean = sum as f64 / array.len() as f64;

    mean
}

/// Calculates the mean value of a vector of floating-point numbers.
///
/// Given a vector (`Vec`) of floating-point numbers (`array`), this function returns the mean
/// value as a floating-point number (`f64`). The mean is calculated by summing all the elements
/// in the vector and dividing the sum by the number of elements in the vector.
///
/// # Arguments
///
/// * `array` - A reference to a vector (`Vec`) of floating-point numbers.
/// The input array for which the mean is calculated.
///
/// # Panics
///
/// This function will panic if the input `array` is empty.
///
/// # Examples
///
/// ```
/// use eratosthenes::mean_f64;
///
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let mean = mean_f64(&input);
/// assert_eq!(mean, 3.0);
/// ```
pub fn mean_f64(array: &Vec<f64>) -> f64 {
    if array.is_empty() {
        panic!("eratosthenes::mean_usize cannot work on an empty array.");
    }

    let sum: f64 = array.iter().sum();
    let mean = sum / array.len() as f64;

    mean
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn mean_usize_test() {
        assert_eq!(mean_usize(&vec![1, 2, 3, 4]), 2.5);
        assert_eq!(mean_usize(&vec![1, 2, 3, 4, 11]), 4.2);
        assert_eq!(mean_usize(&vec![22, 11, 99, 1]), 33.25);
        assert_eq!(mean_usize(&vec![0]), 0.0);
        assert_eq!(mean_usize(&vec![1, 16, 1, 1, 1, 1, 1]), 3.142857142857143);
    }
    
    #[test]
    fn mean_f64_test() {
        assert_eq!(mean_f64(&vec![1.2, 4.1, 0.0]), 1.7666666666666666);
    }
}
