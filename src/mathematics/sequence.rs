use core::panic;

/// For more information about this algorithm see
/// https://oeis.org/A000217
pub fn triangular_numbers(number_of_triangulars: usize) -> Vec<usize> {
    let mut triangulars: Vec<usize> = Vec::new();

    if number_of_triangulars < 1 {
        panic!("You may request 1 or more triangular numbers.")
    }

    for index in 0..number_of_triangulars {
        triangulars.push(index * (index + 1) / 2);
    }

    triangulars
}

pub fn nth_triangular(nth: usize) -> usize {
    let triangulars_list: Vec<usize> = triangular_numbers(nth);
    triangulars_list[triangulars_list.len() - 1]
}
/// Calculates and stores primes. Excludes "until"
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

    for index in 11..until {
        let mut is_prime = true;
        for divisor in 2..=(f64::sqrt(index as f64) as usize) {
            if index % divisor == 0 {
                is_prime = false;
            }
        }
        if !is_prime {
            continue;
        }
        storage.push(index);
    }
    storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_numbers_test() {
        assert_eq!(triangular_numbers(1), vec![0]);
        assert_eq!(triangular_numbers(2), vec![0, 1]);
        assert_eq!(
            triangular_numbers(10),
            vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45]
        );
    }

    #[test]
    fn nth_triangular_test() {
        assert_eq!(nth_triangular(1), 0);
        assert_eq!(nth_triangular(2), 1);
        assert_eq!(nth_triangular(3), 3);
        assert_eq!(nth_triangular(4), 6);
    }

    #[test]
    fn primes_test() {
        assert_eq!(
            primes(10_000_000),
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
}
