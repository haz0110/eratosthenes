/*!
Mathematics library written in Rust.

This library is inspired while solving projecteuler.net problems.
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

pub struct ERAString {
    pub result: Result<String, String>,
    pub duration: std::time::Duration,
}

impl ERAString {
    pub fn new(result: Result<String, String>, duration: std::time::Duration) -> Self {
        ERAString { result, duration }
    }
}

pub mod general;
pub use general::*;

pub mod sequence;
pub use sequence::*;

pub mod trigonometry;
pub use trigonometry::*;

pub mod prime;
pub use prime::*;
