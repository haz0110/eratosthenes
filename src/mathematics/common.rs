#![warn(dead_code)]
#![allow(unused)]

use std::cmp::max;

pub fn clean_array(array: &mut Vec<usize>) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(array.len());
    storage.extend_from_slice(array);
    storage.sort();
    storage.dedup();
    storage.shrink_to_fit();

    storage
}

pub fn merge_two_arrays_arrange_and_clean(
    array1: &mut Vec<usize>,
    array2: &mut Vec<usize>,
) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(array1.len() + array2.len());
    storage.extend_from_slice(array1);
    storage.extend_from_slice(array2);
    storage.sort();
    storage.dedup();
    storage.shrink_to_fit();
    storage
}

pub fn number_of_factors_one_integer(factor: u32, until: u32) -> u32 {
    until / factor
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_arrays_test() {
        let mut array_x: Vec<usize> = vec![3, 6, 9, 12, 15, 18];
        let mut array_y: Vec<usize> = vec![5, 10, 15];
        let result: Vec<usize> = merge_two_arrays_arrange_and_clean(&mut array_x, &mut array_y);
        assert_eq!(result, vec![3, 5, 6, 9, 10, 12, 15, 18]);
    }

    #[test]
    fn euler1_number_of_factors_test() {
        assert_eq!(number_of_factors_one_integer(3, 20), 6);
    }

    #[test]
    fn clean_array_test() {
        let mut array: Vec<usize> = vec![1, 1, 2, 8, 5, 5, 13, 3];
        let correct_array: Vec<usize> = vec![1, 2, 3, 5, 8, 13];

        assert_eq!(clean_array(&mut array), correct_array);
    }

    #[test]
    fn sum_of_even_array_items_test() {
        assert_eq!(sum_of_even_array_items(&mut [1, 3, 6, 11]), 6)
    }

    #[test]
    fn merge_two_arrays_arrange_and_clean_test() {
        assert_eq!(
            merge_two_arrays_arrange_and_clean(&mut vec![1, 3, 5, 8], &mut vec![2, 3, 5, 7]),
            vec![1, 2, 3, 5, 7, 8]
        )
    }
}
