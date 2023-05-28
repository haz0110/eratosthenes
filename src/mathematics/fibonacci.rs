use crate::mathematics::common::clean_array;

pub fn fibonacci_series(until: usize) -> Vec<usize> {
    let mut array: Vec<usize> = vec![1; until + 1];
    array[1] = 2;

    // first two terms and skipped, starts from the third term.
    for index in 2..until {
        if array[index - 1] >= until {
            array[index - 1] = 1;
            break;
        }

        array[index] = array[index - 1] + array[index - 2];
    }

    let resulting_array: Vec<usize> = clean_array(&mut array);

    resulting_array
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fibonacci_series_test() {
        assert_eq!(fibonacci_series(30), vec![1, 2, 3, 5, 8, 13, 21]);
    }
}
