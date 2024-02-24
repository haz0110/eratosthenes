use crate::{ERAMath, ERAGeneral, ERAGeneralTrait};

pub trait ERATrigonometryTrait {
    fn law_of_cosines(a: f64, b: f64, gamma: f64) -> ERAMath<Vec<f64>>;
}

pub struct ERATrigonometry;

impl ERATrigonometryTrait for ERATrigonometry {

    fn law_of_cosines(a: f64, b: f64, gamma: f64) -> ERAMath<Vec<f64>> {

        let start_time = std::time::Instant::now();
        let analysis = "placeholder".to_string();
        
        if a < 0.0 || b < 0.0 || gamma < 0.0 {
            let duration = start_time.elapsed();
            let err_message = "Input values cannot be negative.".to_string();
            return ERAMath::new(Err(err_message), duration, analysis);
        }

        let c_squared = a.powi(2) + b.powi(2) - 2.0 * a * b * gamma.cos();

        let validation_result = ERAGeneral::is_valid_triangle(a, b, c_squared.sqrt());

        match validation_result.result {
            Ok(true) => {
                let duration = start_time.elapsed();
                ERAMath::new(Ok(vec![c_squared.sqrt()]), duration, analysis)
            }
            _ => {
                let duration = start_time.elapsed();
                let err_message = "The input values do not form a valid triangle.".to_string();
                ERAMath::new(Err(err_message), duration, analysis)
            }
        }
    }
}