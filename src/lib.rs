/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

use core::panic;

pub mod sequences;

/// Sorts the vector in ascending order, removes duplicate numbers, returns it as a new array.
/// 
/// # Example Usage 1
/// ```
/// use eratosthenes::array_clean;
/// 
/// let array: Vec<usize> = vec![1, 3, 2, 0];
/// 
/// let cleaned_array = array_clean(&array);
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

/// Accept two array references and returns a new array
/// with two arrays combined.
pub fn array_merge(
    array1: &mut [usize],
    array2: &mut [usize],
) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::new();
    storage.extend_from_slice(array1);
    storage.extend_from_slice(array2);
    storage
}

/// Returns an array with the factors of "number".
/// 
/// Factors include 1 and the number itself.
pub fn factors(number: &usize) -> Vec<usize> {

    let local_number:usize = number.clone();

    if local_number == 1 { return vec![1] };

    let mut storage: Vec<usize> = Vec::new();
    storage.push(1);
    for divisor in 2..local_number / 2 + 1 {
        if number % divisor == 0 {
            storage.push(divisor)
        }
    }
    storage.push(local_number);

    storage
}

pub fn factors_prime(number: &usize) -> Vec<usize> {

    let local_number = number.clone();

    if local_number < 2 { panic!("Enter a number above 1.") };

    let factors = factors(&local_number);

    let mut result: Vec<usize> = Vec::new();

    for prime in factors.iter() {
        if crate::sequences::is_prime(prime) {
            result.push(prime.clone());
        }
    }

    if result.is_empty() {
        panic!("Couldn't populate the vector. Make sure your input is valid.")
    }

    result
}

pub fn is_palindrome(number: &usize) -> bool {

    let local_number = number.clone();

    let mut is_palindrome: bool = false;

    let number_as_string: String = format!("{:?}", local_number);
    let reversed: String = number_as_string.chars().rev().collect::<String>();

    if number_as_string == reversed {
        is_palindrome = true;
    };

    is_palindrome
}

/// sums all the numbers in an array.
pub fn sum(array: &[usize]) -> usize {

    let local_array = array.clone();

    let mut sum: usize = 0;

    for item in local_array.iter() {
        sum += item;
    }
    sum
}

/// sums the even numbers in an array.
pub fn sum_even(array: &[usize]) -> usize {

    let local_array = array.clone();

    let mut sum = 0;
    for (index, item) in local_array.iter().enumerate() {
        if item % 2 == 0 {
            sum += array[index];
        }
    }
    sum
}

/// sums the odd numbers in an array.
pub fn sum_odd(array: &[usize]) -> usize {

    let local_array= array.clone();

    let mut sum: usize = 0;
    for (index, item) in local_array.iter().enumerate() {
        if item % 2 == 1 {
            sum += array[index];
        }
    }
    sum
}

pub fn square(number: &usize) -> usize {
    let local_number: usize = number.clone();
    local_number * local_number
}

/// returns number^power
pub fn to_power(number: &usize, power: &usize) -> usize {

    let local_number = number.clone();
    let local_power = power.clone();

    if local_number == 0 { panic!("What is the power of 0? You tell me.") };
    if local_number == 1 { return 1; };

    if local_power == 0 {
        return 1;
    }

    let multiplier: usize = local_number;
    let mut result: usize = 1;

    let mut local_number: usize = 1;
    loop {
        result *= multiplier;
        if local_number == local_power {
            break;
        }
        local_number += 1;
    }

    result
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
    }

    #[test]
    fn merge_2_arrays_test() {
        assert_eq!(
            array_merge(&mut [1, 3, 5, 8], &mut [2, 3, 5, 7]),
            vec![1, 3, 5, 8, 2, 3, 5, 7]
        )
    }

    #[test]
    fn factors_test() {
        assert_eq!(factors(&28), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(factors(&1), vec![1]);
        assert_eq!(factors(&2), vec![1, 2]);
    }

    #[test]
    fn prime_factors_test() {
        assert_eq!(factors_prime(&28), vec![2, 7]);
        assert_eq!(factors_prime(&2), vec![2]);
        assert_eq!(factors_prime(&30), vec![2, 3, 5]);
    }

    #[test]
    #[should_panic]
    fn clean_array_fail() {
        assert_eq!(array_clean(&vec![]), vec![0]);
    }

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome(&1001));
        assert!(is_palindrome(&20002));
        assert!(!is_palindrome(&2049523));
    }

    #[test]
    fn sum_test() {
        assert_eq!(sum(&[1, 3, 6, 11]), 21)
    }

    #[test]
    fn sum_even_test() {
        assert_eq!(sum_even(&[1, 3, 6, 11]), 6)
    }

    #[test]
    fn sum_odd_test() {
        assert_eq!(sum_odd(&[1, 3, 6, 11]), 15)
    }

    #[test]
    fn square_test() {
        assert_eq!(square(&10), 100);
        assert_eq!(square(&0), 0);
        assert_eq!(square(&3), 9);
    }

    #[test]
    fn to_power_test() {
        assert_eq!(to_power(&2, &3), 8);
        assert_eq!(to_power(&10, &2), 100);
        assert_eq!(to_power(&4, &8), 65536);
    }
}
