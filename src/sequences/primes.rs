pub fn nth_prime(nth: usize) -> usize {
    if nth == 1 {
        return 2;
    }

    let mut nth_prime: usize = 1;
    let mut number: usize = 3;
    let mut current_prime: usize = 0;
    loop {
        if is_prime(&number) {
            current_prime = number;
            nth_prime += 1;
        };
        number += 2;

        if nth_prime == nth {
            break;
        }
    }

    current_prime
}

/// Calculates and stores primes.
///
/// Excludes "until".
///
/// Finds primes until 100_000_000 in approximately 15 seconds.
pub fn primes(until: usize) -> Vec<usize> {
    // First 4 primes are handled here.
    if until == 7 || until == 8 || until == 9 || until == 10 {
        return vec![2, 3, 5, 7];
    };
    if until == 6 {
        return vec![2, 3, 5];
    };
    if until == 5 || until == 4 || until == 3 {
        return vec![2, 3];
    };
    if until <= 2 {
        panic!("There are no prime numbers under 2.")
    };

    let mut storage: Vec<usize> = vec![2, 3, 5, 7];

    // starts from 11
    let mut number: usize = 11;
    'outside: loop {
        if number % 3 == 0 || number % 5 == 0 || number % 7 == 0 {
            number += 2;
            continue 'outside;
        }

        let mut is_prime: bool = true;
        let mut divisor: usize = 3;
        'inside: loop {
            if number % divisor == 0 {
                is_prime = false;
                break 'inside;
            }

            if divisor > (f64::sqrt(number as f64) as usize) {
                break 'inside;
            }

            divisor += 2;
        }
        if !is_prime {
            number += 2;
            continue 'outside;
        };

        if number >= until {
            break 'outside;
        };
        storage.push(number);
        number += 2;
    }

    storage
}

pub fn is_prime(number: &usize) -> bool {
    let mut result: bool = true;

    if *number == 1 { return false; }

    for index in 2..number/2+1 {
        if number % index == 0 { result = false };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_test() {
        assert_eq!(
            primes(1_050),
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
        assert_eq!(nth_prime(10), 29);
        assert_eq!(nth_prime(1), 2);
        assert_eq!(nth_prime(2), 3);
        assert_eq!(nth_prime(1000), 7919);
    }

    #[test]
    fn is_prime_test() {
        assert!(!is_prime(&10));
        assert!(is_prime(&13));
        assert!(!is_prime(&1));
    }
}
