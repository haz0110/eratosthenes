pub fn sum_of_squares(until: usize) -> usize {
    let mut sum: usize = 0;

    for number in 1..=until {
        sum += number * number;
    }

    sum
}

pub fn square_of_sum_of_numbers(until: usize) -> usize {
    let mut sum: usize = 0;
    for number in 1..=until {
        sum += number;
    }
    sum * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_of_sum_of_numbers_test() {
        assert!(square_of_sum_of_numbers(3) == 36);
        assert!(square_of_sum_of_numbers(10) == 3025);
    }

    #[test]
    fn sum_of_squares_test() {
        assert!(sum_of_squares(10) == 385);
    }
}