/// calculates until "until", excluding "until"
pub fn fibonacci(until: usize) -> Vec<usize> {

    if until < 2 { panic!("Invalid parameter. Please use integers above 1.") }

    if until == 2 { return vec![1, 1]; };
    if until == 3 { return vec![1, 1, 2]; };

    let mut array: Vec<usize> = vec![1, 1, 2];

    let mut index: usize = 3;
    loop {
        if array[index - 1] + array[index - 2] >= until {
            break;
        }
        
        array.push(array[index - 1] + array[index - 2]);
        index += 1;
    }
    array
}

/// Find the nth fibonacci number until 1_000_000_000_000_000_000
pub fn nth_fibonacci(nth: usize) -> usize {
    let fibonacci: Vec<usize> = fibonacci(1_000_000_000_000_000_000);

    fibonacci[nth - 1]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fibonacci_test() {
        assert_eq!(fibonacci(2), vec![1, 1]);
        assert_eq!(fibonacci(3), vec![1, 1, 2]);
        assert_eq!(fibonacci(100), vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
    }

    #[test]
    fn nth_fibonacci_test() {
        assert_eq!(nth_fibonacci(1), 1);
        assert_eq!(nth_fibonacci(2), 1);
        assert_eq!(nth_fibonacci(3), 2);
        assert_eq!(nth_fibonacci(4), 3);
        assert_eq!(nth_fibonacci(5), 5);
        assert_eq!(nth_fibonacci(11), 89);
        assert_eq!(nth_fibonacci(12), 144);
        assert_eq!(nth_fibonacci(39), 63245986);
        assert_eq!(nth_fibonacci(40), 102334155);
    }
}
