/// Geometric Sequence: a_n = a * r^(n-1)
/// 
/// n_start > 1
/// 
/// r > 0
/// 
/// Starts with a_(n_start)
/// 
/// Ends with a_(n_end)
/// 
/// n_start < n_end
pub fn geometric(a: usize, r: usize, n_start: usize, n_end: usize) -> Vec<usize> {

    if n_start < 1 { panic!("n_start must be higher than 1.") };
    if r < 1 { panic!("r must be higher than 1") };
    if n_start >= n_end { panic!("n_start must be lesser than n_end") };


    let mut vector: Vec<usize> = Vec::new();

    for n in n_start..=n_end {
        vector.push(a * crate::common::to_power(&r, &(n - 1)));
    }

    if vector.is_empty() { panic!("Couldn't populate the vector.") };

    vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometric_sequence_test() {
        assert_eq!(geometric(3, 2, 1, 4), vec![3, 6, 12, 24]);
        assert_eq!(geometric(4, 12, 1, 8), vec![4, 48, 576, 6912, 82944, 995328, 11943936, 143327232]);
        assert_eq!(geometric(3, 5, 2, 3), vec![15, 75]);
    }
}