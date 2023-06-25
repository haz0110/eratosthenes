use eratosthenes;

#[test]
fn test_clean_array() {
    assert_eq!(
        eratosthenes::array_clean(&vec![1, 2, 3, 5, 8, 13]),
        vec![1, 2, 3, 5, 8, 13]
    );
}