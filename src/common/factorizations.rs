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

#[cfg(test)]
mod tests {

    use super::*;

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
}