#![warn(dead_code)]
#![allow(unused)]

use core::panic;
use std::cmp::max;

use crate::mathematics::sequences::is_prime;

pub fn clean_array(array: &mut Vec<usize>) -> Vec<usize> {
    if array.is_empty() {
        panic!("Cannot work on empty array.")
    };

    let mut storage: Vec<usize> = Vec::with_capacity(array.len());
    storage.extend_from_slice(array);
    storage.sort();
    storage.dedup();
    storage.shrink_to_fit();

    storage
}

/// Accept two array borrows and returns a optimized,
/// sorted, duplicates removed, exact capacity array.
pub fn merge_2_arrays(
    array1: &mut [usize],
    array2: &mut [usize],
) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::new();
    storage.extend_from_slice(array1);
    storage.extend_from_slice(array2);
    storage
}

/// Returns an array with the factors of "number".
pub fn factors(number: usize, include_one: bool, include_self: bool) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::new();
    if include_one {
        storage.push(1)
    };

    for divisor in 2..number / 2 + 1 {
        if number % divisor == 0 {
            storage.push(divisor)
        }
    }

    if include_self {
        storage.push(number)
    };

    storage
}

pub fn prime_factors(number: usize) -> Vec<usize> {

    if number < 2 { panic!("Enter a number above 1.") };

    let mut factors = factors(number, false, true);

    let mut vector: Vec<usize> = Vec::new();

    for prime in factors.iter() {
        if is_prime(prime) {
            vector.push(*prime);
        }
    }

    if vector.is_empty() {
        panic!("Couldn't populate the vector. Make sure your input is valid.")
    }

    vector
}

pub fn sum_of_array_items(array: Vec<usize>) -> usize {
    let mut sum: usize = 0;

    for item in array.iter() {
        sum += item;
    }

    sum
}

pub fn sum_of_even_array_items(array: &mut [usize]) -> usize {
    let mut sum = 0;
    for (index, item) in array.to_owned().iter().enumerate() {
        if item % 2 == 0 {
            sum += array[index];
        }
    }
    sum
}

pub fn to_power(number: usize, power: usize) -> usize {

    if number < 2 {panic!("Number must be greater than 1.")};

    if power == 0 {
        return 1;
    }

    let multiplier = number;
    let mut result: usize = 1;

    for index in 1..=power {
        result *= multiplier;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_test() {
        assert_eq!(factors(28, true, true), vec![1, 2, 4, 7, 14, 28]);
        assert_eq!(factors(28, false, true), vec![2, 4, 7, 14, 28]);
        assert_eq!(factors(28, true, false), vec![1, 2, 4, 7, 14]);
        assert_eq!(factors(28, false, false), vec![2, 4, 7, 14]);
    }

    #[test]
    fn prime_factors_test() {
        assert_eq!(prime_factors(28), vec![2, 7]);
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(30), vec![2, 3, 5]);
    }

    #[test]
    fn clean_array_test() {
        assert_eq!(
            clean_array(&mut vec![1, 2, 3, 5, 8, 13]),
            vec![1, 2, 3, 5, 8, 13]
        );
    }

    #[test]
    #[should_panic]
    fn clean_array_fail() {
        assert_eq!(clean_array(&mut vec![]), vec![0]);
    }

    #[test]
    fn sum_of_even_array_items_test() {
        assert_eq!(sum_of_even_array_items(&mut [1, 3, 6, 11]), 6)
    }

    #[test]
    fn merge_2_arrays_test() {
        assert_eq!(
            merge_2_arrays(&mut [1, 3, 5, 8], &mut [2, 3, 5, 7]),
            vec![1, 3, 5, 8, 2, 3, 5, 7]
        )
    }

    #[test]
    fn to_power_test() {
        assert_eq!(to_power(2, 3), 8);
        assert_eq!(to_power(10, 2), 100);
        assert_eq!(to_power(4, 8), 65536);
    }
}
