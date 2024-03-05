use std::fmt::Debug;

use crate::{ERABool, ERAMath};

pub trait ERAGeneralTrait {
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool;
    fn number_of_decimal_digits(number: f64) -> ERAMath<usize>;
    fn reduce_decimal_digits(value: f64, desired_decimal_digits: usize) -> ERAMath<f64>;
    fn arithmetic_sequence(start: usize, end: usize, ratio: usize) -> ERAMath<Vec<usize>>;
    fn multiples_of(start: usize, end: usize, multiplier: usize) -> ERAMath<Vec<usize>>;
    fn sum<T: std::ops::Add<Output = T> + Default + Clone + Debug>(array: Vec<T>) -> ERAMath<T>;
    fn is_even(number: usize) -> ERABool;
    fn is_odd(number: usize) -> ERABool;
}
pub struct ERAGeneral;

impl ERAGeneralTrait for ERAGeneral {
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool {
        let start_time = std::time::Instant::now();

        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            let err_message = "a, b, c must be positive.".to_string();
            let duration = start_time.elapsed();
            return ERABool::new(Err(err_message), duration);
        }

        // Triangle inequality check
        if a + b > c && a + c > b && b + c > a {
            let result = true;
            let duration = start_time.elapsed();
            ERABool::new(Ok(result), duration)
        } else {
            let err_message =
                "Invalid triangle sides: they do not satisfy the triangle inequality theorem.";
            let duration = start_time.elapsed();
            ERABool::new(Err(err_message.to_string()), duration)
        }
    }

    fn is_even(number: usize) -> ERABool {
        let start_time = std::time::Instant::now();
        let result = number % 2 == 0;

        let duration = start_time.elapsed();
        ERABool::new(Ok(result), duration)
    }

    fn is_odd(number: usize) -> ERABool {
        let start_time = std::time::Instant::now();
        let result = number % 2 == 1;

        let duration = start_time.elapsed();
        ERABool::new(Ok(result), duration)
    }

    fn number_of_decimal_digits(number: f64) -> ERAMath<usize> {
        let start_time = std::time::Instant::now();

        #[cfg(debug_assertions)]
        {
            println!(
                "number.fract() is: {}, number is: {}",
                number.fract(),
                number
            );
        }

        if let Some(decimal_point_index) = number.to_string().chars().position(|c| c == '.') {
            let decimal_digits = number.to_string().len() - (decimal_point_index + 1);
            let duration = start_time.elapsed();
            println!("returns inside if let statement.");
            return ERAMath::new(Ok(decimal_digits), duration);
        }

        #[cfg(debug_assertions)]
        {
            println!("function returns at the end");
        }

        let duration: std::time::Duration = start_time.elapsed();
        ERAMath::new(Ok(0), duration)
    }

    fn reduce_decimal_digits(number: f64, desired_decimal_digits: usize) -> ERAMath<f64> {
        let start_time = std::time::Instant::now();

        let number_of_decimals = crate::ERAGeneral::number_of_decimal_digits(number)
            .result
            .unwrap();

        if number_of_decimals < desired_decimal_digits {
            let err_message = "Err: cannot increase decimal places.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        if number_of_decimals == desired_decimal_digits {
            let err_message = "Err: cannot reduce decimal places because the number already has the specified number of decimal digits.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        let multiplier = 10_f64.powi(desired_decimal_digits as i32);
        let mut adj_number = (number * multiplier).round();
        adj_number /= multiplier;
        let result = adj_number;

        let duration = start_time.elapsed();

        ERAMath::new(Ok(result), duration)
    }

    fn sum<T: std::ops::Add<Output = T> + Default + Clone + Debug>(array: Vec<T>) -> ERAMath<T> {
        let start_time = std::time::Instant::now();

        let sum_result = array
            .clone()
            .into_iter()
            .fold(T::default(), |acc, x| acc + x);

        let duration = start_time.elapsed();
        ERAMath::new(Ok(sum_result), duration)
    }

    fn arithmetic_sequence(start: usize, end: usize, ratio: usize) -> ERAMath<Vec<usize>> {
        let start_time = std::time::Instant::now();

        if start > end {
            let err_message = "Err: start cannot be greater than end.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        let mut calculation: Vec<usize> = Vec::new();

        if ratio != 0 {
            let mut current = start;
            while current < end {
                calculation.push(current);
                current += ratio;
            }
        }

        let duration = start_time.elapsed();

        ERAMath::new(Ok(calculation), duration)
    }

    fn multiples_of(start: usize, end: usize, multiplier: usize) -> ERAMath<Vec<usize>> {
        let start_time = std::time::Instant::now();

        if start > end {
            let err_message = "Err: start cannot be greater than end.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }
        let mut index_start = start;
        while index_start % multiplier != 0 {
            index_start += 1;
        }

        let mut calculation: Vec<usize> = Vec::new();

        if multiplier != 0 {
            let mut current = index_start;
            while current <= end {
                calculation.push(current);
                current += multiplier;
            }
        }

        let duration = start_time.elapsed();

        ERAMath::new(Ok(calculation), duration)
    }
}
