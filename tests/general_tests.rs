#[allow(clippy::approx_constant)]
#[cfg(test)]
mod tests {

    use eratosthenes::{ERAGeneral, ERAGeneralTrait};

    #[test]
    fn test_is_valid_triangle() {
        let calculation = ERAGeneral::is_valid_triangle(3.0, 4.0, 5.0);
        assert!(calculation.result.unwrap());

        let calculation2 = ERAGeneral::is_valid_triangle(1.0, 1.0, 10.0);
        assert!(calculation2.result.is_err());
        assert_eq!(
            calculation2.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );

        let calculation3 = ERAGeneral::is_valid_triangle(4.0, 8.0, 4.0);
        assert!(calculation3.result.is_err());
        assert_eq!(
            calculation3.result.err().unwrap(),
            "Invalid triangle sides: they do not satisfy the triangle inequality theorem."
        );

        let calculation4 = ERAGeneral::is_valid_triangle(0.0, 0.0, 0.0);
        assert!(calculation4.result.is_err());
        assert_eq!(
            calculation4.result.err().unwrap(),
            "a, b, c must be positive."
        );

        let calculation5 = ERAGeneral::is_valid_triangle(-2.0, -4.0, -6.0);
        assert!(calculation5.result.is_err());
        assert_eq!(
            calculation5.result.err().unwrap(),
            "a, b, c must be positive."
        );
    }

    #[test]
    fn test_number_of_decimal_digits() {
        let calculation = ERAGeneral::number_of_decimal_digits(0.000321);
        assert!(calculation.result.is_ok());
        assert_eq!(calculation.result.unwrap(), 6);

        let calculation2 = ERAGeneral::number_of_decimal_digits(0.41290000412);
        assert!(calculation2.result.is_ok());
        assert_eq!(calculation2.result.unwrap(), 11);

        let calculation3 = ERAGeneral::number_of_decimal_digits(5.3);
        assert!(calculation3.result.is_ok());
        assert_eq!(calculation3.result.unwrap(), 1);

        let calculation4 = ERAGeneral::number_of_decimal_digits(0.0);
        assert!(calculation4.result.is_ok());
        assert_eq!(calculation4.result.unwrap(), 0);

        let calculation5 = ERAGeneral::number_of_decimal_digits(12.0);
        assert!(calculation5.result.is_ok());
        assert_eq!(calculation5.result.unwrap(), 0);

        let calculation6 = ERAGeneral::number_of_decimal_digits(3.14159);
        assert!(calculation6.result.is_ok());
        assert_eq!(calculation6.result.unwrap(), 5);

        let calculation7 = ERAGeneral::number_of_decimal_digits(1.1);
        assert!(calculation7.result.is_ok());
        assert_eq!(calculation7.result.unwrap(), 1);
    }

    #[test]
    fn test_reduce_decimal_digits() {
        let calculation = ERAGeneral::reduce_decimal_digits(3.14159, 2);
        assert_eq!(calculation.result.unwrap(), 3.14);

        let calculation2 = ERAGeneral::reduce_decimal_digits(0.000000321, 2);
        assert_eq!(calculation2.result.unwrap(), 0.0);

        let calculation3 = ERAGeneral::reduce_decimal_digits(0.000321, 7);
        assert_eq!(
            calculation3.result.err().unwrap(),
            "Err: cannot increase decimal places."
        );

        let calculation4 = ERAGeneral::reduce_decimal_digits(5.6789, 3);
        assert_eq!(calculation4.result.unwrap(), 5.679);

        let calculation2 = ERAGeneral::reduce_decimal_digits(0.000000321, 7);
        assert_eq!(calculation2.result.unwrap(), 0.0000003);
    }

    #[test]
    fn test_sum() {
        let calculation = ERAGeneral::sum(vec![10, 11, 34]);
        assert_eq!(calculation.result.unwrap(), 55);
    }

    #[test]
    fn arithmetic_sequence() {
        let calculation = ERAGeneral::arithmetic_sequence(3, 10, 2);
        assert_eq!(calculation.result.unwrap(), vec![3, 5, 7, 9]);
    }

    #[test]
    fn multiples_of() {
        let calculation = ERAGeneral::multiples_of(3, 10, 3);
        assert_eq!(calculation.result.unwrap(), vec![3, 6, 9]);

        let calculation2 = ERAGeneral::multiples_of(3, 10, 2);
        assert_eq!(calculation2.result.unwrap(), vec![4, 6, 8, 10]);
    }
}
