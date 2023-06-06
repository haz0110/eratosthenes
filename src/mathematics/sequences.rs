use core::panic;
use crate::mathematics::common::to_power;

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

/// Calculates and stores primes. Excludes "until".
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

        let mut is_prime = true;
        let mut divisor = 3;
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
        if !is_prime { number += 2; continue 'outside; };

        if number >= until { break 'outside; };
        storage.push(number);
        number += 2;
    }

    storage
}


pub fn nth_prime(nth: usize) -> usize {
    primes(nth*12)[nth - 1]
}

/// calculates until "until", excluding "until"
pub fn fibonacci(until: usize) -> Vec<usize> {

    if until < 2 { panic!("Invalid parameter. Please use integers above 1.") }

    if until == 2 { return vec![1, 1]; };
    if until == 3 { return vec![1, 1, 2]; };

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

/// Find the nth fibonacci number until 89th
pub fn nth_fibonacci(nth: usize) -> usize {
    let fibonacci = fibonacci(18_000_000_000_000_000_000);

    fibonacci[nth - 1]
}

/// Geometric Sequence: a_n = a * r^(n-1)
/// 
/// n_start > 1
/// 
/// r > 0
/// 
/// Starts with a_(n_start)
/// 
/// Ends with a_(n_end)
/// 
/// n_start < n_end
pub fn geometric_sequence(a: usize, r: usize, n_start: usize, n_end: usize) -> Vec<usize> {

    if n_start < 1 { panic!("n_start must be higher than 1.") };
    if r < 1 { panic!("r must be higher than 1") };
    if n_start >= n_end { panic!("n_start must be lesser than n_end") };


    let mut vector: Vec<usize> = Vec::new();

    for n in n_start..=n_end {
        vector.push(a * to_power(r, n - 1));
    }

    if vector.is_empty() { panic!("Couldn't populate the vector.") };

    vector
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
    }

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
        assert_eq!(nth_fibonacci(89), 1779979416004714189);
    }

    #[test]
    fn geometric_sequence_test() {
        assert_eq!(geometric_sequence(3, 2, 1, 4), vec![3, 6, 12, 24]);
        assert_eq!(geometric_sequence(4, 12, 1, 8), vec![4, 48, 576, 6912, 82944, 995328, 11943936, 143327232]);
        assert_eq!(geometric_sequence(3, 5, 2, 3), vec![15, 75]);
    }
}
