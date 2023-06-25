/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library mostly works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

pub mod sequences;
pub mod trigonometry;

pub mod statistics;

pub mod common;
pub use self::common::array_manipulations::{array_clean, array_merge};
pub use self::common::factorizations::{factors, factors_prime};
pub use self::common::mean::{mean_f64, mean_usize};
pub use self::common::palindrome::is_palindrome;
pub use self::common::summations::{sum_even, sum_odd};

