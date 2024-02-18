use crate::{ERAMath, ERABool};

pub trait ERAGeneralTrait {
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool;
    fn reduce_decimals(value: f64, decimal_places: i64) -> ERAMath<f64>;
}
pub struct ERAGeneral;

impl ERAGeneralTrait for ERAGeneral {
    
    fn is_valid_triangle(a: f64, b: f64, c: f64) -> ERABool {

        let start_time = std::time::Instant::now();

        // Triangle inequality check
        if a + b > c && a + c > b && b + c > a {
            let result = true;
            let duration = start_time.elapsed();
            ERABool::new(Ok(result), duration)
        } else {
            let error_message = "Invalid triangle sides: they do not satisfy the triangle inequality theorem.";
            let duration = start_time.elapsed();
            ERABool::new(Err(error_message.to_string()), duration)    
        }
    }

    fn reduce_decimals(value: f64, decimal_places: i64) -> ERAMath<f64> {

        let start_time = std::time::Instant::now();

        let multiplier = 10u64.pow(decimal_places as u32);
        let rounded_value = (value * multiplier as f64).round() / multiplier as f64;

        let duration = start_time.elapsed();

        ERAMath::new(Ok(rounded_value), duration)

    }
}