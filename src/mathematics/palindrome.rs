pub fn palindromes_2_digits() -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(1000);
    for number in (10*10)..(99*99) {

        let number_as_string: String = format!("{:?}", number);
        let reversed_number_as_string = number_as_string.chars().rev().collect::<String>();
        let reversed_number: usize = reversed_number_as_string.parse().unwrap();

        if number == reversed_number {

            'inLoop: for first_multiplier in 10..=99 {
                for second_multiplier in 10..=99 {
                    if second_multiplier * first_multiplier == number {
                        println!("{} * {} = {}", first_multiplier, second_multiplier, number);
                        storage.push(number);
                        break 'inLoop;
                    }
                }
            }
        }
        
    }

    storage
}

pub fn palindromes_3_digits() -> Vec<usize> {
    let mut storage: Vec<usize> = Vec::with_capacity(1000);
    for number in (100*100)..(999*999) {

        let number_as_string: String = format!("{:?}", number);
        let reversed_number_as_string = number_as_string.chars().rev().collect::<String>();
        let reversed_number: usize = reversed_number_as_string.parse().unwrap();

        if number == reversed_number {
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

    storage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes_2_digits_test() {
        assert!(palindromes_2_digits()[0] == 121);
        assert!(palindromes_2_digits()[8] == 444);
    }

    #[test]
    fn palindromes_3_digits_test() {
        assert_eq!(palindromes_3_digits()[0], 10_201);
        assert_eq!(palindromes_3_digits()[1], 11_211);
    }

}