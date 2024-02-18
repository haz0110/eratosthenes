
/// Generates prime numbers up to a specified limit using the Sieve of Eratosthenes algorithm.
///
/// Given the limit (`until`), this function returns a vector (`Vec`) containing all prime
/// numbers less than or equal to the specified limit.
///
/// # Arguments
///
/// * `until` - The limit for generating prime numbers. The sequence will contain all prime
///             numbers less than or equal to this limit.
///
/// # Error
///
/// This function will throw error if the `until` parameter is less than 2, as there are no prime
/// numbers below 2.
///
/// # Examples
///
/// ```
/// use eratosthenes::primes;
///
/// let until = 20;
/// let prime_sequence = primes(until).unwrap();
/// assert_eq!(prime_sequence, vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn primes(until: usize) -> Result<Vec<usize>, &'static str> {
    if until < 2 {
        return Err("There are no prime numbers under 2.");
    };

    let mut sieve: Vec<bool> = vec![true; until as usize + 1];
    sieve[0] = false;
    sieve[1] = false;

    let mut number: usize = 2;
    while number * number <= until as usize {
        if sieve[number] {
            let mut multiple: usize = number * number;
            while multiple <= until as usize {
                sieve[multiple] = false;
                multiple += number;
            }
        }
        number += 1;
    }

    let result: Vec<usize> = sieve
        .iter()
        .enumerate()
        .filter(|(_, &is_prime)| is_prime)
        .map(|(number, _)| number)
        .collect();

    Ok(result)
}

/// Finds the nth prime number.
///
/// Given an unsigned integer (`nth`), this function finds and returns the nth prime number.
///
/// # Arguments
///
/// * `nth` - An unsigned integer. The position of the prime number to find.
///
/// # Examples
///
/// ```
/// use eratosthenes::nth_prime;
///
/// let nth: usize = 10;
/// let prime = nth_prime(nth).unwrap();
/// assert_eq!(prime, 29);
/// ```
pub fn nth_prime(nth: usize) -> Result<usize, &'static str> {
    
    // exception cases
    if nth == 0 {
        return Err("There is no 0th prime number.");
    }
    if nth == 1 {
        return Ok(2);
    }
    if nth == 2 {
        return Ok(3);
    }
    
    let primes = primes(((nth as f64 * (nth as f64).ln()) * 2.0) as usize)?;

    if let Some(&result) = primes.get(nth - 1) {
        Ok(result)
    } else {
        Err("Error occurred while finding the nth prime.")
    }
}

/// Checks if a number is prime.
///
/// Given an unsigned integer (`number`), this function checks if it is a prime number and returns
/// a boolean indicating the result.
///
/// # Arguments
///
/// * `number` - An unsigned integer. The number to check for primality.
///
/// # Examples
///
/// ```
/// use eratosthenes::is_prime;
///
/// let number = 37;
/// let is_prime_number = is_prime(number).unwrap();
/// assert_eq!(is_prime_number, true);
/// ```
pub fn is_prime(number: usize) -> Result<bool, &'static str> {
    if number < 2 {
        return Ok(false);
    }

    if number == 2 || number == 3 {
        return Ok(true);
    }

    if number % 2 == 0 || number % 3 == 0 {
        return Ok(false);
    }

    let sqrt = (number as f64).sqrt() as usize;
    let mut index = 5;

    while index <= sqrt {
        if number % index == 0 || number % (index + 2) == 0 {
            return Ok(false);
        }
        index += 6;
    }

    Ok(true)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn primes_test() {
        assert_eq!(
            primes(1_050).unwrap(),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563,
                569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
                659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
                769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877,
                881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991,
                997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049
            ]
        );
    }

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime(1).unwrap(), 2);
        assert_eq!(nth_prime(2).unwrap(), 3);
        assert_eq!(nth_prime(10).unwrap(), 29);
        assert_eq!(nth_prime(1000).unwrap(), 7_919);
        // assert_eq!(nth_prime(31_240_412).unwrap(), 598_294_381);
    }

    #[test]
    fn is_prime_test() {
        assert!(!is_prime(1).unwrap());
        assert!(is_prime(13).unwrap());
        assert!(is_prime(2).unwrap());
        assert!(is_prime(7_741).unwrap());
        assert!(is_prime(612_271_815_315_483_857).unwrap());

    }
}