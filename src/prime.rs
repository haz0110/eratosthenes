use crate::{ERABool, ERAMath};

pub trait ERAPrimeTrait {
    fn primes(until: usize) -> ERAMath<Vec<usize>>;
    fn nth_prime(nth: usize) -> ERAMath<usize>;
    fn is_prime(number: usize) -> ERABool;
}
pub struct ERAPrime;

impl ERAPrimeTrait for ERAPrime {
    fn primes(until: usize) -> ERAMath<Vec<usize>> {
        let start_time = std::time::Instant::now();
        let analysis = "placeholder".to_string();

        if until < 2 {
            let err_message = "Err: there are no prime numbers under 2.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration, analysis);
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

        let calculation: Vec<usize> = sieve
            .iter()
            .enumerate()
            .filter(|(_, &is_prime)| is_prime)
            .map(|(number, _)| number)
            .collect();

        let duration = start_time.elapsed();

        ERAMath::new(Ok(calculation), duration, analysis)
    }
    fn nth_prime(nth: usize) -> ERAMath<usize> {
        let start_time = std::time::Instant::now();
        let analysis = "placeholder".to_string();

        // exception cases
        if nth == 0 {
            let err_message = "Err: There is no 0th prime number.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration, analysis);
        }
        if nth == 1 {
            let duration = start_time.elapsed();
            return ERAMath::new(Ok(2), duration, analysis);
        }
        if nth == 2 {
            let duration = start_time.elapsed();
            return ERAMath::new(Ok(3), duration, analysis);
        }

        let primes = crate::ERAPrime::primes(((nth as f64 * (nth as f64).ln()) * 2.0) as usize);

        let calculation = primes.result.unwrap()[nth - 1];
        let duration = start_time.elapsed();

        ERAMath::new(Ok(calculation), duration, analysis)
    }

    fn is_prime(number: usize) -> ERABool {
        let start_time = std::time::Instant::now();
        let analysis = "placeholder".to_string();

        if number == 2 {
            let duration = start_time.elapsed();
            return ERABool::new(Ok(true), duration, analysis);
        }

        if number < 2 || number % 2 == 0 || number % 3 == 0 {
            let duration = start_time.elapsed();
            return ERABool::new(Ok(false), duration, analysis);
        }

        let sqrt = (number as f64).sqrt() as usize;
        let mut index = 5;

        while index <= sqrt {
            if number % index == 0 || number % (index + 2) == 0 {
                let duration = start_time.elapsed();
                return ERABool::new(Ok(false), duration, analysis);
            }
            index += 6;
        }

        let duration = start_time.elapsed();
        return ERABool::new(Ok(true), duration, analysis);
    }
}