pub fn is_palindrome(number: usize) -> bool {

    let mut is_palindrome: bool = false;

    let number_as_string: String = format!("{:?}", number);
    let reversed: String = number_as_string.chars().rev().collect::<String>();

    if number_as_string == reversed {
        is_palindrome = true;
    };

    is_palindrome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert!(is_palindrome(1001));
        assert!(is_palindrome(20002));
        assert!(!is_palindrome(2049523));
    }
}
