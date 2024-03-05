#[cfg(test)]
mod tests {

    use eratosthenes::{ERASequence, ERASequenceTrait};

    #[test]
    fn test_fibonacci() {
        let calculation1 = ERASequence::fibonacci(10);
        assert_eq!(calculation1.result.unwrap(), vec![1, 1, 2, 3, 5, 8]);

        let calculation2 = ERASequence::fibonacci(1);
        assert_eq!(
            calculation2.result,
            Err("Err: the input parameter must be greater than 2.".to_string())
        );

        let calculation3 = ERASequence::fibonacci(100_000_000);
        assert_eq!(
            calculation3.result.unwrap(),
            vec![
                1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1_597, 2_584, 4_181,
                6_765, 10_946, 17_711, 28_657, 46_368, 75_025, 121_393, 196_418, 317_811, 514_229,
                832_040, 1_346_269, 2_178_309, 3_524_578, 5_702_887, 9_227_465, 14_930_352,
                24_157_817, 39_088_169, 63_245_986
            ]
        );
    }
}
