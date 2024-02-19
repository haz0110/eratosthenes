use crate::{ERAMath, ERABool};

pub trait ERAGeneralTrait {
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool;
    fn number_of_decimal_digits(number: f64) -> ERAMath<usize>;
    fn reduce_decimal_digits(value: f64, decimal_places: usize) -> ERAMath<f64>;
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

        let mut digits = 0;
        let mut number = number;

        while number.fract() != 0.0 {
            number *= 10.0;
            digits += 1;
        }

        let duration = start_time.elapsed();

        ERAMath::new(Ok(digits), duration)
    }

    fn reduce_decimal_digits(value: f64, decimal_places: usize) -> ERAMath<f64> {

        let start_time = std::time::Instant::now();

        if crate::ERAGeneral::number_of_decimal_digits(value).result.unwrap() < decimal_places {
            let err_message = "Err: cannot increase decimal places, multiply with 10 instead.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        if crate::ERAGeneral::number_of_decimal_digits(value).result.unwrap() == decimal_places {
            let err_message = "Err: cannot reduce decimal places because the number already has the specified number of decimal digits.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        let multiplier = 10u64.pow(decimal_places as u32);
        let rounded_value = (value * multiplier as f64).round() / multiplier as f64;

        let duration = start_time.elapsed();

        ERAMath::new(Ok(rounded_value), duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_triangle_valid() {
        let calculation = ERAGeneral::is_valid_triangle(3.0, 4.0, 5.0);
        assert!(calculation.result.unwrap());
    }

    #[test]
    fn test_is_valid_triangle_invalid1() {
        let calculation = ERAGeneral::is_valid_triangle(1.0, 1.0, 10.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid2() {
        let calculation = ERAGeneral::is_valid_triangle(4.0, 8.0, 4.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid3() {
        let calculation = ERAGeneral::is_valid_triangle(0.0, 0.0, 0.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "a, b, c must be positive."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid4() {
        let calculation = ERAGeneral::is_valid_triangle(-2.0, -4.0, -6.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "a, b, c must be positive."
        );
    }

    #[test]
    fn test_reduce_decimal_digits_rounding1() {
        let calculation = ERAGeneral::reduce_decimal_digits(3.14159, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 3.14);
    }

    #[test]
    fn test_reduce_decimal_digits_rounding2() {
        let calculation = ERAGeneral::reduce_decimal_digits(0.000000321, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 0.0);
    }

    #[test]
    fn test_reduce_decimal_digits_rounding3() {
        let calculation = ERAGeneral::reduce_decimal_digits(0.000321, 7);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Err: cannot increase decimal places, multiply with 10 instead."
        );
    }

    #[test]
    fn test_reduce_decimal_digits_no_rounding() {
        let calculation = ERAGeneral::reduce_decimal_digits(5.6789, 3);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 5.679);
    }

    #[test]
    fn test_number_of_decimal_digits1() {
        let calculation = ERAGeneral::number_of_decimal_digits(5.6789);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 4);
    }
}
