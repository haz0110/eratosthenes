use palindrome::*;

pub fn is_palindrome_number(number: usize) -> bool {
    let number_as_string: String = format!("{:?}", number);
    is_palindrome(number_as_string)
}

pub fn palindromes(digit: usize) -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(1000);

    match digit {
        2 => {
            for number in (10 * 10)..(99 * 99) {
                if is_palindrome(number) {
                    'inLoop: for first_multiplier in 10..=99 {
                        for second_multiplier in 10..=99 {
                            if second_multiplier * first_multiplier == number {
                                storage.push(number);
                                break 'inLoop;
                            }
                        }
                    }
                }
            }
        }
        3 => {
            for number in (100 * 100)..(999 * 999) {
                if is_palindrome(number) {
                    'inLoop: for first_multiplier in 100..=999 {
                        for second_multiplier in 100..=999 {
                            if second_multiplier * first_multiplier == number {
                                storage.push(number);
                                break 'inLoop;
                            }
                        }
                    }
                }
            }
        }
        _ => return vec![0],
    }
    storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes_test() {
        assert_eq!(palindromes(2)[0], 121);
        assert_eq!(palindromes(2)[8], 444);
        assert_eq!(palindromes(3)[0], 10_201);
        assert_eq!(palindromes(3)[1], 11_211);
    }
}
