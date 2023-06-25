/*!
Common mathematical functions.
# Example Usage 1
```
use eratosthenes::common;

let array: Vec<usize> = vec![1, 2, 3];

// prints "6"
println!("{}", common::sum(&array));
```
*/

use core::panic;

/// Sorts the vector in ascending order, removes duplicate numbers, returns it as a new array.
/// 
/// # Example
/// 
/// ```
/// use eratosthenes::common::clean_array;
/// 
/// let array: Vec<usize> = vec![1, 3, 2, 0];
/// 
/// let cleaned_array = clean_array(&array);
/// ```
pub fn clean_array(array: &Vec<usize>) -> Vec<usize> {
    if array.is_empty() {
        panic!("Cannot work on empty array.")
    };

    let mut storage: Vec<usize> = Vec::new();
    storage.extend_from_slice(array);
    storage.sort();
    storage.dedup();
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
/// 
/// Factors include 1 and the number itself.
pub fn factors(number: &usize) -> Vec<usize> {

    let local_number:usize = *number;

    if local_number == 1 { return vec![1] };

    let mut storage: Vec<usize> = Vec::new();
    storage.push(1);
    for divisor in 2..number / 2 + 1 {
        if number % divisor == 0 {
            storage.push(divisor)
        }
    }
    storage.push(*number);

    storage
}

pub fn prime_factors(number: &usize) -> Vec<usize> {

    let local_number = *number;

    if local_number < 2 { panic!("Enter a number above 1.") };

    let factors = factors(&local_number);

    let mut vector: Vec<usize> = Vec::new();

    for prime in factors.iter() {
        if crate::sequences::primes::is_prime(prime) {
            vector.push(*prime);
        }
    }

    if vector.is_empty() {
        panic!("Couldn't populate the vector. Make sure your input is valid.")
    }

    vector
}

/// sums all the numbers in an array.
pub fn sum(array: &[usize]) -> usize {

    let local_array = array;

    let mut sum: usize = 0;

    for item in local_array.iter() {
        sum += item;
    }
    sum
}

/// sums the even numbers in an array.
pub fn sum_even(array: &[usize]) -> usize {

    let local_array = array;

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

    let local_array = array;

    let mut sum = 0;
    for (index, item) in local_array.iter().enumerate() {
        if item % 2 == 1 {
            sum += array[index];
        }
    }
    sum
}

/// returns number^power
pub fn to_power(number: &usize, power: &usize) -> usize {

    let local_number = *number;
    let local_power = *power;

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
            clean_array(&vec![1, 2, 3, 5, 8, 13]),
            vec![1, 2, 3, 5, 8, 13]
        );
    }

    #[test]
    fn merge_2_arrays_test() {
        assert_eq!(
            merge_2_arrays(&mut [1, 3, 5, 8], &mut [2, 3, 5, 7]),
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
        assert_eq!(prime_factors(&28), vec![2, 7]);
        assert_eq!(prime_factors(&2), vec![2]);
        assert_eq!(prime_factors(&30), vec![2, 3, 5]);
    }

    #[test]
    #[should_panic]
    fn clean_array_fail() {
        assert_eq!(clean_array(&vec![]), vec![0]);
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
    fn to_power_test() {
        assert_eq!(to_power(&2, &3), 8);
        assert_eq!(to_power(&10, &2), 100);
        assert_eq!(to_power(&4, &8), 65536);
    }
}
