use std::cmp::max;

pub fn is_prime(number: usize) -> bool {
    match number {
        0 => false,
        1 => false,
        2 => true,
        3 => true,
        number if number % 2 == 0 => false,
        number if number % 3 == 0 => false,
        _ => {
            let sqrt = (number as f32).sqrt() as usize;
            let mut base = 6;
            while base <= sqrt {
                if number % (base - 1) == 0 {
                    return false;
                }
                if number % (base + 1) == 0 {
                    return false;
                }
                base += 6;
            }
            true
        }
    }
}

pub fn biggest_prime_factor(until: usize) -> usize {
    let mut result: usize = 0;
    let mut candidate = (until as f32).sqrt().floor() as usize;

    if candidate % 2 == 0 {
        candidate += 1;
    }

    while candidate > 1 {    
        if is_prime(candidate) && (until % candidate == 0){
            if is_prime(until / candidate) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_test() {
        assert!(is_prime(20) == false);
        assert!(is_prime(19) == true);
        assert!(is_prime(18) == false);
        assert!(is_prime(17) == true);
    }

    #[test]
    fn biggest_prime_factor_test() {
        assert!(biggest_prime_factor(21) == 7);
        assert!(biggest_prime_factor(33) == 11);
    }

}