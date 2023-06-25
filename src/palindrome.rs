/// Checks if a given number is a palindrome.
///
/// Given an unsigned integer (`number`), this function checks if it is a palindrome, meaning it
/// reads the same forwards and backwards. It returns `true` if the number is a palindrome and
/// `false` otherwise.
///
/// # Arguments
///
/// * `number` - An unsigned integer. The number to check for palindromicity.
///
/// # Examples
///
/// ```
/// use eratosthenes::is_palindrome;
///
/// let input = 12321;
/// let is_palindrome = is_palindrome(input);
/// assert_eq!(is_palindrome, true);
/// ```
pub fn is_palindrome(number: usize) -> bool {
    let number_as_string: String = number.to_string();
    let reversed: String = number_as_string.chars().rev().collect();

    number_as_string == reversed
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