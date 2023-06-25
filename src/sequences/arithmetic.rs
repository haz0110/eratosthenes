/// Generates an arithmetic sequence of numbers.
///
/// Given the first term (`a`), the common difference (`d`), and the number of terms (`n`),
/// this function returns a vector (`Vec`) containing the arithmetic sequence.
/// The arithmetic sequence is generated by adding the common difference to the previous term.
///
/// # Arguments
///
/// * `a` - The first term of the arithmetic sequence.
/// * `d` - The common difference between consecutive terms.
/// * `n` - The number of terms to generate in the arithmetic sequence.
///
/// # Examples
/// ```
/// use eratosthenes::sequences::arithmetic;
///
/// let a = 2;
/// let d = 3;
/// let n = 5;
///
/// let sequence = arithmetic(a, d, n);
/// assert_eq!(sequence, vec![2, 5, 8, 11, 14]);
/// ```
pub fn arithmetic(a: usize, d: usize, n: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(n);

    for index in 0..n {
        result.push(a + index * d);
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn arithmetic_test() {
        assert_eq!(arithmetic(2, 3, 4), [2, 5, 8, 11]);
    }
}