/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.

Note that this library mostly works with `usize` integers, tested on 64 bit systems, and most of the functions return whether a `usize` or `Vec<usize>`
*/

pub mod constants;
pub use constants::PI;

pub mod general;
pub use general::{is_valid_triangle, reduce_decimals};

pub mod trigonometry;
pub use trigonometry::law_of_cosines;

pub mod primes;
pub use primes::{primes, nth_prime, is_prime};