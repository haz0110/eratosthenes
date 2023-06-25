pub mod triangular_numbers;
pub mod faulhabers;

/// Returns sequence of numbers in which each successive term is a
/// sum of its preceding term and a fixed number.
/// 
/// Formula: a + ( n_i * d) starting from n = 0
/// 
/// # Example Usage 1
/// ```
/// use eratosthenes::sequences::arithmetic;
/// let start = 10;
/// let constant_multiplier = 3;
/// let n = 10;
/// 
/// // prints [10, 13, 16, 19, 22, 25, 28, 31, 34, 37]
/// println!("{:?}", arithmetic(&start, &constant_multiplier, &n));
/// ```
pub fn arithmetic(a: &usize, d: &usize, n: &usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let local_a = *a;
    let local_d = *d;
    let local_n: usize = *n;

    for index in 0..local_n {
        result.push( local_a + index * local_d );
    }

    result
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

/// Find the nth fibonacci number until 1_000_000_000_000_000_000
pub fn nth_fibonacci(nth: usize) -> usize {
    let fibonacci: Vec<usize> = fibonacci(1_000_000_000_000_000_000);

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
pub fn geometric(a: &usize, r: &usize, n_start: &usize, n_end: &usize) -> Vec<usize> {

    let local_a: usize = a.clone();
    let local_r: usize = r.clone();
    let local_n_start: usize = n_start.clone();
    let local_n_end: usize = n_end.clone();

    if local_n_start < 1 { panic!("n_start must be higher than 1.") };
    if local_r < 1 { panic!("r must be higher than 1") };
    if local_n_start >= local_n_end { panic!("n_start must be lesser than n_end") };


    let mut vector: Vec<usize> = Vec::new();

    for n in local_n_start..=local_n_end {
        vector.push(local_a * crate::to_power(&r, &(n - 1)));
    }

    if vector.is_empty() { panic!("Couldn't populate the vector.") };

    vector
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

pub fn is_prime(number: &usize) -> bool {
    let mut result: bool = true;

    if *number == 1 { return false; }

    for index in 2..number/2+1 {
        if number % index == 0 { result = false };
    }

    result
}

/// Returns a vector of square numbers of consecutive integers
/// starting from 0.
/// 
/// # Example Usage 1
/// 
/// ```
/// use eratosthenes::sequences::square_numbers;
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
    fn arithmetic_test() {
        assert_eq!(arithmetic(&2, &3, &4), [2, 5, 8, 11]);
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
    }

    #[test]
    fn geometric_sequence_test() {
        assert_eq!(geometric(&3, &2, &1, &4), vec![3, 6, 12, 24]);
        assert_eq!(geometric(&4, &12, &1, &8), vec![4, 48, 576, 6912, 82944, 995328, 11943936, 143327232]);
        assert_eq!(geometric(&3, &5, &2, &3), vec![15, 75]);
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
        assert_eq!(nth_prime(1000), 7919);
    }

    #[test]
    fn is_prime_test() {
        assert!(!is_prime(&10));
        assert!(is_prime(&13));
        assert!(!is_prime(&1));
    }

    #[test]
    fn square_numbers_test() {
        assert_eq!(square_numbers(&10), vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
    }

    
}