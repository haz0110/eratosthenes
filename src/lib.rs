/*!
Mathematics library written in Rust.

This library is inspired while solving projecteuler.net problems.
*/

pub struct ERAMath<T> {
    pub result: Result<T, String>,
    pub duration: std::time::Duration,
    pub analysis: String,
}

impl<T> ERAMath<T> {
    pub fn new(result: Result<T, String>, duration: std::time::Duration, analysis: String) -> Self {
        ERAMath { result, duration, analysis }
    }
}

pub struct ERABool {
    pub result: Result<bool, String>,
    pub duration: std::time::Duration,
    pub analysis: String,
}

impl ERABool {
    pub fn new(result: Result<bool, String>, duration: std::time::Duration, analysis: String) -> Self {
        ERABool { result, duration, analysis }
    }
}

pub struct ERAString {
    pub result: Result<String, String>,
    pub duration: std::time::Duration,
    pub analysis: String,
}

impl ERAString {
    pub fn new(result: Result<String, String>, duration: std::time::Duration, analysis: String) -> Self {
        ERAString { result, duration, analysis }
    }
}

pub mod general;
pub use general::*;

pub mod trigonometry;
pub use trigonometry::*;

pub mod prime;
pub use prime::*;