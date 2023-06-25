/// It concerns expressing the sum of the p-th power
/// of the first n integers.
/// 
/// for example, n: 4, p: 2 returns the following vector
/// 
/// 1, 4, 9, 16
pub fn faulhabers(n: &usize, p: &usize) -> Vec<usize> {

    let local_p: usize = *p;

    let local_n: usize = *n;

    let mut result: Vec<usize> = Vec::new();

    for index in 1..=local_n {
        result.push(crate::to_power(&index, &local_p));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn faulhabers_test() {
        assert_eq!(faulhabers(&10, &3), vec![1, 8, 27, 64, 125, 216, 343, 512, 729, 1000]);
    }
}