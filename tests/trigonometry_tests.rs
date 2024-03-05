#[cfg(test)]
mod tests {

    use eratosthenes::{ERAGeneral, ERAGeneralTrait, ERATrigonometry, ERATrigonometryTrait};

    #[test]
    fn test_law_of_cosines() {
        let calculation1 = ERATrigonometry::law_of_cosines(10.0, 10.0, 20.0)
            .result
            .unwrap();
        let rounded1 = ERAGeneral::reduce_decimal_digits(calculation1, 1)
            .result
            .unwrap();
        assert_eq!(rounded1, 10.8);
    }
}
