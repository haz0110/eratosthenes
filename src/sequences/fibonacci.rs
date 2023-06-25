/// Generates a Fibonacci sequence up to a specified limit.
///
/// Given the limit (`until`), this function returns a vector (`Vec<usize>`) containing the Fibonacci
/// sequence up to, but excluding, the specified limit. The Fibonacci sequence starts with the
/// numbers 1 and 1, and each subsequent number is the sum of the two preceding numbers.
///
/// # Arguments
///
/// * `until` - The limit for generating the Fibonacci sequence. The sequence will contain all
///            Fibonacci numbers less than this limit.
///
/// # Panics
///
/// This function will panic if the `until` parameter is less than 2, as the Fibonacci sequence
/// is typically defined to start with the numbers 1 and 1.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::sequences::fibonacci;
///
/// let until = 20;
/// let sequence = fibonacci(until);
/// assert_eq!(sequence, vec![1, 1, 2, 3, 5, 8, 13]);
/// ```
pub fn fibonacci(until: usize) -> Vec<usize> {
    if until < 2 {
        panic!("Invalid parameter. Please use integers above 1.")
    }

    if until == 2 {
        return vec![1, 1];
    };
    if until == 3 {
        return vec![1, 1, 2];
    };

    let mut array: Vec<usize> = vec![1, 1, 2];

    let mut index: usize = 3;
    loop {
        if array[index - 1] + array[index - 2] >= until {
            break;
        }

        array.push(array[index - 1] + array[index - 2]);
        index += 1;
    }
    array
}

/// Returns the nth Fibonacci number.
///
/// Given the position (`nth`), this function calculates and returns the nth Fibonacci number.
/// It generates a Fibonacci sequence up to a large limit (1_000_000_000_000_000_000) and retrieves
/// the desired Fibonacci number from the sequence.
///
/// # Arguments
///
/// * `nth` - The position of the Fibonacci number to retrieve. The position starts from 1.
///
/// # Examples
///
/// ```rust
/// use eratosthenes::sequences::nth_fibonacci;
///
/// let nth = 10;
/// let fibonacci_number = nth_fibonacci(nth);
/// assert_eq!(fibonacci_number, 55);
/// ```
pub fn nth_fibonacci(nth: usize) -> usize {
    let fibonacci: Vec<usize> = fibonacci(1_000_000_000_000_000_000);
    fibonacci[nth - 1]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci(2), vec![1, 1]);
        assert_eq!(fibonacci(3), vec![1, 1, 2]);
        assert_eq!(fibonacci(100), vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    #[test]
    fn nth_fibonacci_test() {
        assert_eq!(nth_fibonacci(1), 1);
        assert_eq!(nth_fibonacci(2), 1);
        assert_eq!(nth_fibonacci(3), 2);
        assert_eq!(nth_fibonacci(4), 3);
        assert_eq!(nth_fibonacci(5), 5);
        assert_eq!(nth_fibonacci(11), 89);
        assert_eq!(nth_fibonacci(12), 144);
        assert_eq!(nth_fibonacci(39), 63245986);
        assert_eq!(nth_fibonacci(40), 102334155);
    }
}
