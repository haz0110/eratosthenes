/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library mostly works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

use core::panic;

pub mod sequences;

/// Cleans an array of unsigned integers by removing duplicate values and sorting it.
///
/// Given an array of unsigned integers (`array`), this function returns a new vector (`Vec<usize>`)
/// containing the unique values from the input array, sorted in ascending order. It panics if
/// the input array is empty.
///
/// # Arguments
///
/// * `array` - A reference to a vector (`Vec<usize>`) of unsigned integers. The array to be cleaned.
///
/// # Panics
///
/// This function will panic if the input `array` is empty.
///
/// # Examples
///
/// ```
/// use eratosthenes::array_clean;
///
/// let input = vec![5, 2, 7, 2, 4, 5];
/// let cleaned_array = array_clean(&input);
/// assert_eq!(cleaned_array, vec![2, 4, 5, 7]);
/// ```
pub fn array_clean(array: &Vec<usize>) -> Vec<usize> {
    if array.is_empty() {
        panic!("eratosthenes::array_clean cannot work on an empty array.");
    };

    let mut storage: Vec<usize> = Vec::new();
    storage.extend_from_slice(array);
    storage.sort();
    storage.dedup();
    storage
}

/// Merges two mutable arrays of unsigned integers into a single vector.
///
/// Given two mutable arrays of unsigned integers (`array1` and `array2`), this function returns
/// a new vector (`Vec`) containing the elements from both input arrays. The function does not
/// modify the original arrays.
///
/// # Arguments
///
/// * `array1` - A mutable slice (`&mut [usize]`) of unsigned integers. The first array to be merged.
/// * `array2` - A mutable slice (`&mut [usize]`) of unsigned integers. The second array to be merged.
///
/// # Examples
///
/// ```
/// use eratosthenes::array_merge;
///
/// let mut input1: Vec<usize> = vec![1, 2, 3];
/// let mut input2: Vec<usize> = vec![4, 5, 6];
/// let merged_array = array_merge(&input1, &input2);
/// assert_eq!(merged_array, vec![1, 2, 3, 4, 5, 6]);
/// ```
pub fn array_merge(array1: &Vec<usize>, array2: &Vec<usize>) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::new();
    storage.extend_from_slice(array1);
    storage.extend_from_slice(array2);
    storage
}

/// Calculates the factors of a given number.
///
/// Given an unsigned integer (`number`), this function returns a vector (`Vec`) containing all
/// the factors of the input number. The factors are the positive integers that evenly divide the
/// given number. The vector includes 1 and the number itself.
///
/// # Arguments
///
/// * `number` - An unsigned integer. The number for which factors are calculated.
///
/// # Examples
///
/// ```
/// use eratosthenes::factors;
///
/// let input = 12;
/// let factor_sequence = factors(input);
/// assert_eq!(factor_sequence, vec![1, 2, 3, 4, 6, 12]);
/// ```
pub fn factors(number: usize) -> Vec<usize> {
    if number == 1 {
        return vec![1];
    };

    let mut storage: Vec<usize> = Vec::new();
    storage.push(1);
    for divisor in 2..number / 2 + 1 {
        if number % divisor == 0 {
            storage.push(divisor)
        }
    }
    storage.push(number);

    storage
}

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

/// Calculates the prime factors of a given number.
///
/// Given an unsigned integer (`number`), this function returns a vector (`Vec`) containing all
/// the prime factors of the input number. Prime factors are the prime numbers that divide the
/// given number without leaving a remainder. The vector includes the prime factors in ascending order.
///
/// # Arguments
///
/// * `number` - An unsigned integer. The number for which prime factors are calculated.
///
/// # Panics
///
/// This function will panic if the input `number` is less than 2.
/// This function will panic if no prime factors are found for the input `number`.
///
/// # Examples
///
/// ```
/// use eratosthenes::factors_prime;
///
/// let input = 12;
/// let prime_factor_sequence = factors_prime(input);
/// assert_eq!(prime_factor_sequence, vec![2, 3]);
/// ```
pub fn factors_prime(number: usize) -> Vec<usize> {
    if number < 2 {
        panic!("eratosthenes::factors_prime cannot work with numbers less than 2.")
    };

    let factors: Vec<usize> = factors(number);

    let result: Vec<usize> = factors
        .into_iter()
        .filter(|factor| crate::sequences::is_prime(*factor))
        .collect();

    if result.is_empty() {
        panic!("Couldn't populate the vector. Make sure your input is valid.")
    }

    result
}

/// Checks if a given number is a palindrome.
///
/// Given an unsigned integer (`number`), this function checks if it is a palindrome, meaning it
/// reads the same forwards and backwards. It returns `true` if the number is a palindrome and
/// `false` otherwise.
///
/// # Arguments
///
/// * `number` - An unsigned integer. The number to check for palindromicity.
///
/// # Examples
///
/// ```
/// use eratosthenes::is_palindrome;
///
/// let input = 12321;
/// let is_palindrome = is_palindrome(input);
/// assert_eq!(is_palindrome, true);
/// ```
pub fn is_palindrome(number: usize) -> bool {
    let number_as_string: String = number.to_string();
    let reversed: String = number_as_string.chars().rev().collect();

    number_as_string == reversed
}

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
    fn clean_array_test() {
        assert_eq!(
            array_clean(&vec![1, 2, 3, 5, 8, 13]),
            vec![1, 2, 3, 5, 8, 13]
        );
        assert_eq!(array_clean(&vec![0, 0, 0, 0]), vec![0]);
        assert_eq!(
            array_clean(&vec![9, 11, 20, 30, 1, 23]),
            vec![1, 9, 11, 20, 23, 30]
        );
    }

    #[test]
    #[should_panic]
    fn array_clean_fail() {
        assert_eq!(array_clean(&vec![]), vec![0]);
    }

    #[test]
    fn array_merge_test() {
        assert_eq!(
            array_merge(&vec![1, 3, 5, 8], &vec![2, 3, 5, 7]),
            vec![1, 3, 5, 8, 2, 3, 5, 7]
        )
    }

    #[test]
    fn factors_test() {
        assert_eq!(factors(28), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(factors(1), vec![1]);
        assert_eq!(factors(2), vec![1, 2]);
    }

    #[test]
    fn factors_prime_test() {
        assert_eq!(factors_prime(28), vec![2, 7]);
        assert_eq!(factors_prime(2), vec![2]);
        assert_eq!(factors_prime(30), vec![2, 3, 5]);
        assert_eq!(factors_prime(99651999), vec![3, 33217333]);
    }

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

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome(1001));
        assert!(is_palindrome(20002));
        assert!(!is_palindrome(2049523));
    }

    #[test]
    fn sum_even_test() {
        assert_eq!(sum_even(&vec![1, 3, 6, 11]), 6)
    }

    #[test]
    fn sum_odd_test() {
        assert_eq!(sum_odd(&vec![1, 3, 6, 11]), 15)
    }
}
