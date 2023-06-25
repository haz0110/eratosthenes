/*!
This module includes some mathmetical sequences and related calculations.
*/

pub mod arithmetic;
pub use self::arithmetic::arithmetic;

pub mod fibonacci;
pub use self::fibonacci::{fibonacci, nth_fibonacci};

pub mod primes;
pub use self::primes::{is_prime, nth_prime, primes};

pub mod squares;
pub use self::squares::square_numbers;

pub mod triangular_numbers;
pub use self::triangular_numbers::{nth_triangular, triangular_numbers};
