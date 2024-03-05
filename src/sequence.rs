use crate::ERAMath;

pub trait ERASequenceTrait {
    fn fibonacci(until: usize) -> ERAMath<Vec<usize>>;
}

pub struct ERASequence;

impl ERASequenceTrait for ERASequence {
    fn fibonacci(until: usize) -> ERAMath<Vec<usize>> {
        let start_time = std::time::Instant::now();

        if until < 2 {
            let err_message = "Err: the input parameter must be greater than 2.".to_string();
            let duration = start_time.elapsed();
            return ERAMath::new(Err(err_message), duration);
        }

        if until == 2 {
            let result = vec![1, 1, 2];
            let duration = start_time.elapsed();
            return ERAMath::new(Ok(result), duration);
        }

        if until == 3 {
            let result = vec![1, 1, 2, 3];
            let duration = start_time.elapsed();
            return ERAMath::new(Ok(result), duration);
        }

        let mut result: Vec<usize> = vec![1, 1, 2];

        let mut index: usize = 3;
        loop {
            if result[index - 1] + result[index - 2] >= until {
                break;
            }

            result.push(result[index - 1] + result[index - 2]);
            index += 1;
        }

        let duration = start_time.elapsed();
        ERAMath::new(Ok(result), duration)
    }
}
