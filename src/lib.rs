/*!

Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library works with `usize` integers, tested on 64 bit systems, and all functions returns whether a `usize` or `Vec<usize>`.

Operations for floating numbers and negative numbers are not planned for now.

# Example Usage 1
```
use eratosthenes::sequences::primes::is_prime;

let number: usize = 10;

// prints "Is 10 a prime? -> false"
println!("Is {} a prime? -> {}", number, is_prime(&number));
```
*/

pub mod common;
pub mod palindrome;
pub mod sequences;
pub mod square_operations;

pub fn clean_array(array: &Vec<usize>) -> Vec<usize> {
    common::clean_array(&array)
}