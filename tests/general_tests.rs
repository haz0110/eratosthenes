#[cfg(test)]
mod tests {

    use eratosthenes::{ERAGeneral, ERAGeneralTrait};

    #[test]
    fn test_is_valid_triangle_valid() {
        let calculation = ERAGeneral::is_valid_triangle(3.0, 4.0, 5.0);
        assert!(calculation.result.unwrap());
    }

    #[test]
    fn test_is_valid_triangle_invalid1() {
        let calculation = ERAGeneral::is_valid_triangle(1.0, 1.0, 10.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid2() {
        let calculation = ERAGeneral::is_valid_triangle(4.0, 8.0, 4.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid3() {
        let calculation = ERAGeneral::is_valid_triangle(0.0, 0.0, 0.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "a, b, c must be positive."
        );
    }

    #[test]
    fn test_is_valid_triangle_invalid4() {
        let calculation = ERAGeneral::is_valid_triangle(-2.0, -4.0, -6.0);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "a, b, c must be positive."
        );
    }

    #[test]
    fn test_number_of_decimal_digits1() {
        let calculation = ERAGeneral::number_of_decimal_digits(0.000321);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 6);
    }

    #[test]
    fn test_number_of_decimal_digits2() {
        let calculation = ERAGeneral::number_of_decimal_digits(0.41290000412);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 11);
    }

    #[test]
    fn test_number_of_decimal_digits3() {
        let calculation = ERAGeneral::number_of_decimal_digits(5.3);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 1);
    }

    #[test]
    fn test_number_of_decimal_digits4() {
        let calculation = ERAGeneral::number_of_decimal_digits(0.0);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 0);
    }

    #[test]
    fn test_number_of_decimal_digits5() {
        let calculation = ERAGeneral::number_of_decimal_digits(12.0);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 0);
    }

    #[test]
    fn test_number_of_decimal_digits6() {
        let calculation = ERAGeneral::number_of_decimal_digits(3.14159);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 5);
    }

    #[test]
    fn test_number_of_decimal_digits7() {
        let calculation = ERAGeneral::number_of_decimal_digits(1.1);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 1);
    }

    #[test]
    fn test_reduce_decimal_digits_rounding1() {
        let calculation = ERAGeneral::reduce_decimal_digits(3.14159, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 3.14);
    }

    #[test]
    fn test_reduce_decimal_digits_rounding2() {
        let calculation = ERAGeneral::reduce_decimal_digits(0.000000321, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 0.0);
    }

    #[test]
    fn test_reduce_decimal_digits_rounding3() {
        let calculation = ERAGeneral::reduce_decimal_digits(0.000321, 7);
        assert!(calculation.result.is_err());
        assert_eq!(
            calculation.result.err().unwrap(),
            "Err: cannot increase decimal places."
        );
    }

    #[test]
    fn test_reduce_decimal_digits_no_rounding() {
        let calculation = ERAGeneral::reduce_decimal_digits(5.6789, 3);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 5.679);
    }

    #[test]
    fn arithmetic_sequence1() {
        let calculation = ERAGeneral::arithmetic_sequence(3, 10, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), vec![3, 5, 7, 9]);
    }

    #[test]
    fn multiples_of1() {
        let calculation = ERAGeneral::multiples_of(3, 10, 2);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), vec![3, 5, 7, 9]);
    }
}
