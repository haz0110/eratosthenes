/// Returns a vector of square numbers of consecutive integers
/// starting from 0.
/// 
/// # Example Usage 1
/// 
/// ```
/// use eratosthenes::sequences::square_numbers::*;
/// 
/// let number_of_squares = 10;
/// 
/// // prints [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
/// println!("{:?}", square_numbers(&10));
/// ```
pub fn square_numbers(number_of_squares: &usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let n: usize = number_of_squares.clone();

    for index in 0..n {
        result.push(index * index);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_numbers_test() {
        assert_eq!(square_numbers(&10), vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }
}