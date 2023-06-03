pub fn special_pythagorean_triplet() -> usize {
    let mut vector: Vec<usize> = Vec::new();

    for number_a in 1..1000 {
        for number_b in 1..1000 {
            for number_c in 1..1000 {
                if number_a < number_b
                    && number_b < number_c
                    && (number_a as usize).pow(2) + (number_b as usize).pow(2)
                        == (number_c as usize).pow(2)
                    && number_a + number_b + number_c == 1000
                {
                    vector = vec![number_a, number_b, number_c];
                }
            }
        }
    }
    vector[0] * vector[1] * vector[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem9_test() {
        assert_eq!(special_pythagorean_triplet(), 31875000);
    }
}
