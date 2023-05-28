use std::cmp::max;
use primes::*;

pub fn biggest_prime_factor(until: usize) -> usize {
    let mut result: usize = 0;
    let mut candidate = (until as f32).sqrt().floor() as usize;

    if candidate % 2 == 0 {
        candidate += 1;
    }

    while candidate > 1 {    
        if primes::is_prime(candidate as u64) && (until % candidate == 0){
            if primes::is_prime((until / candidate) as u64) {
                result = max(candidate, until / candidate);
                break;
            }
            result = candidate;
            break;
        }
        candidate -= 2;
    }

    result
}

pub fn nth_prime_alt(n: usize) -> usize {
    let cap: usize = 1_000_000;
    let mut storage: Vec<usize> = Vec::new();
    for number in 2..cap {
        if primes::is_prime(number as u64){
            storage.push(number);
        }
        if storage.len() == n {
            break;
        }
    }
    storage[storage.len() - 1]
}

pub fn nth_prime(n: usize) -> usize {
    let mut pset = primes::Sieve::new();

    for (index, number) in pset.iter().enumerate().take(n) {
        if index == n - 1 {
            return number as usize;
        }
        if index >= n {
            return 0;
        }
    }
    // for safety
    0
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biggest_prime_factor_test() {
        assert_eq!(biggest_prime_factor(21), 7);
        assert_eq!(biggest_prime_factor(33), 11);
    }

    #[test]
    fn nth_prime_test() {
        assert_eq!(nth_prime(3), 5);
        assert_eq!(nth_prime(10001), 104743);
    }

}