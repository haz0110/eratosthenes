pub fn mean(array: &[f64]) -> f64 {

    if array.len() == 0 { panic!("Array cannot be empty.") };

    let sum: f64 = array.iter().sum();
    sum / array.len() as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mean_test() {
        assert_eq!(mean(&vec![1.0, 2.0]), 1.5);
    }
}