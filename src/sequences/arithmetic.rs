/// Returns sequence of numbers in which each successive term is a
/// sum of its preceding term and a fixed number.
/// 
/// Formula: a + ( n_i * d) starting from n = 0
/// 
/// # Example Usage 1
/// ```
/// use eratosthenes::sequences::arithmetic::*;
/// let start = 10;
/// let constant_multiplier = 3;
/// let n = 10;
/// 
/// // prints [10, 13, 16, 19, 22, 25, 28, 31, 34, 37]
/// println!("{:?}", arithmetic(&start, &constant_multiplier, &n));
/// ```
pub fn arithmetic(a: &usize, d: &usize, n: &usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let local_a = *a;
    let local_d = *d;
    let local_n: usize = *n;

    for index in 0..local_n {
        result.push( local_a + index * local_d );
    }

    result
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn arithmetic_test() {
        assert_eq!(arithmetic(&2, &3, &4), [2, 5, 8, 11]);
    }
}