/*!
Mathematics library written in pure Rust.

This library is inspired while solving projecteuler.net problems. You may find the functions of this library useful for those problems.
*/

pub struct ERAMath<T> {
    pub result: Result<T, String>,
    pub duration: std::time::Duration,
}

impl<T> ERAMath<T> {
    pub fn new(result: Result<T, String>, duration: std::time::Duration) -> Self {
        ERAMath { result, duration }
    }
}

pub struct ERABool {
    pub result: Result<bool, String>,
    pub duration: std::time::Duration,
}

impl ERABool {
    pub fn new(result: Result<bool, String>, duration: std::time::Duration) -> Self {
        ERABool { result, duration }
    }
}

impl PartialEq for ERABool {
    fn eq(&self, other: &Self) -> bool {
        self.result == other.result && self.duration == other.duration
    }
}

impl Eq for ERABool {}

pub mod general;
pub use general::*;

pub mod trigonometry;
pub use trigonometry::*;

pub mod prime;
pub use prime::*;