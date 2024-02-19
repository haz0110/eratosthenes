use crate::{ERAMath, ERABool};

pub trait ERAGeneralTrait {
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool;
    fn number_of_decimal_digits(number: f64) -> ERAMath<usize>;
    fn reduce_decimal_digits(value: f64, decimal_places: usize) -> ERAMath<f64>;
    fn arithmetic_sequence(start: usize, end: usize, ratio: usize) -> ERAMath<Vec<usize>>;
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
            let err_message = "Invalid triangle sides: they do not satisfy the triangle inequality theorem.";
            let duration = start_time.elapsed();
            ERABool::new(Err(err_message.to_string()), duration)    
        }
    }

    fn number_of_decimal_digits(number: f64) -> ERAMath<usize> {
            
        let start_time = std::time::Instant::now();

        #[cfg(debug_assertions)]
        {
            println!("number.fract() is: {}, number is: {}", number.fract(), number);
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

    fn reduce_decimal_digits(value: f64, decimal_places: usize) -> ERAMath<f64> {

        let start_time = std::time::Instant::now();

        let original_decimal_places = crate::ERAGeneral::number_of_decimal_digits(value).result.unwrap();

        if original_decimal_places < decimal_places {
            let err_message = "Err: cannot increase decimal places.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        if original_decimal_places == decimal_places {
            let err_message = "Err: cannot reduce decimal places because the number already has the specified number of decimal digits.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        let rounded_value = format!("{:.*}", decimal_places, value).parse::<f64>().unwrap_or(value);

        let duration = start_time.elapsed();

        ERAMath::new(Ok(rounded_value), duration)
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
}