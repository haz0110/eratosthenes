#![warn(dead_code)]
#![allow(unused)]

pub fn smallest_multiple(until: usize, cap: usize) -> usize {
    let mut result: usize = 0;

    for number in 2..=cap {
        let mut counter: usize = 1;
        'inner: for until_index in 2..=until {
            if number % until_index != 0 {
                break 'inner;
            }
            counter += 1;
        }
        if counter == until {
            result = number;
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_multiple_test() {
        assert_eq!(smallest_multiple(10, 2530), 2520);
        assert_eq!(smallest_multiple(20, 232_792_561), 232_792_560);
    }
}
