use core::panic;

pub fn triangular_numbers(number_of_triangulars: usize) -> Vec<usize> {
    let mut triangulars: Vec<usize> = Vec::new();

    if number_of_triangulars < 1 {
        panic!("You may request 1 or more triangular numbers.")
    }

    for index in 0..number_of_triangulars {
        triangulars.push(index * (index + 1) / 2);
    }

    triangulars
}

pub fn nth_triangular(nth: usize) -> usize {
    let triangulars_list : Vec<usize> = triangular_numbers(nth);
    triangulars_list[triangulars_list.len() - 1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_numbers_test() {
        assert_eq!(triangular_numbers(1), vec![0]);
        assert_eq!(triangular_numbers(2), vec![0, 1]);
        assert_eq!(triangular_numbers(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45]);
    }

    #[test]
    fn nth_triangular_test() {
        assert_eq!(nth_triangular(1), 0);
        assert_eq!(nth_triangular(2), 1);
        assert_eq!(nth_triangular(3), 3);
        assert_eq!(nth_triangular(4), 6);
    }

}