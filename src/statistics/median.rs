pub fn median(array: &mut [f64]) -> f64 {
    if array.is_empty() { panic!("Cannot compute the median of an empty vector") };

    array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = array.len() / 2;

    if array.len() % 2 == 0 {
        (array[mid - 1] + array[mid]) / 2.0
    } else {
        array[mid]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn median_test() {
        assert_eq!(median(&mut vec![1.0, 2.0, 2.0, 3.0, 1.0]), 2.0);
    }
}