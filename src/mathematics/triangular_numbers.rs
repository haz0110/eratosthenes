pub fn is_triangular(number: usize) -> bool {
    match number {
        0 => false,
        1 => true,
        _ => {
            let mut count: usize = 0;
            let mut is_triangular: bool = false;
            for index in 1..=number {
                count += index;
                if count > number {
                    break;
                }
                if count == number {
                    is_triangular = true;
                    break;
                } 
            }
            is_triangular
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_triangular_test() {
        assert_eq!(is_triangular(3), true);
        assert_eq!(is_triangular(6), true);
        assert_eq!(is_triangular(10), true);
        assert_eq!(is_triangular(15), true);
        assert_eq!(is_triangular(21), true);
        assert_eq!(is_triangular(1), true);
        assert_eq!(is_triangular(2), false);
    }
}