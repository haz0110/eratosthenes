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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn array_clean_test() {
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
}
