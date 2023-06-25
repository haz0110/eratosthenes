/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library mostly works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

pub mod sequences;

pub mod array_manipulations;
pub use self::array_manipulations::{array_clean, array_merge};

pub mod factorizations;
pub use self::factorizations::{factors, factors_prime};

pub mod mean;
pub use self::mean::{mean_f64, mean_usize};

pub mod palindrome;
pub use self::palindrome::is_palindrome;

pub mod summations;
pub use self::summations::{sum_even, sum_odd};

