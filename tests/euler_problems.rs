use eratosthenes;

#[test]
fn euler_problem1() {
    let limit: usize = 500;

    let mut multiples_of_3: Vec<usize> = Vec::new();
    let mut current_3_multiple: usize = 3;
    loop {
        if current_3_multiple >= limit {
            break;
        };

        multiples_of_3.push(current_3_multiple);

        current_3_multiple += 3;
    }

    let mut multiples_of_5: Vec<usize> = Vec::new();
    let mut current_5_multiple: usize = 5;
    loop {
        if current_5_multiple >= limit {
            break;
        };

        multiples_of_5.push(current_5_multiple);

        current_5_multiple += 5;
    }

    let mut array: Vec<usize> = eratosthenes::array_merge(&mut multiples_of_3, &mut multiples_of_5);

    array = eratosthenes::array_clean(&array);

    let result: usize = array.iter().sum();

    assert_eq!(result, 57918);
}

#[test]
fn euler_problem2() {
    let fibonacci: Vec<usize> = eratosthenes::sequences::fibonacci(2_000_000);
    let result = eratosthenes::sum_even(&fibonacci);

    assert_eq!(result, 1089154);
}
