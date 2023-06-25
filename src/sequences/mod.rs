/*!
 This module includes some mathmetical sequences and related calculations.
 */

pub mod arithmetic;
pub mod fibonacci;
pub mod primes;
pub mod squares;
pub mod triangular_numbers;

pub use self::arithmetic::arithmetic;

pub use self::fibonacci::fibonacci;
pub use self::fibonacci::nth_fibonacci;

pub use self::primes::is_prime;
pub use self::primes::nth_prime;
pub use self::primes::primes;

pub use self::squares::square_numbers;

pub use self::triangular_numbers::nth_triangular;
pub use self::triangular_numbers::triangular_numbers;
