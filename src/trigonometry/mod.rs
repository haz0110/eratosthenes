//! # Trigonometric Operations
//!
//! This module provides a set of trigonometric functions that allow you to perform common
//! calculations involving angles and their corresponding trigonometric values. The functions
//! in this module are based on the standard trigonometric functions provided by Rust's
//! standard library.
//!
//! ## Supported Functions
//!
//! - `sin`: Calculates the sine of an angle in radians.
//! - `asin`: Calculates the arc sine of a value in radians.
//! - `cos`: Calculates the cosine of an angle in radians.
//! - `acos`: Calculates the arc cosine of a value in radians.
//! - `tan`: Calculates the tangent of an angle in radians.
//! - `atan`: Calculates the arc tangent of a value in radians.
//!
//! ## Usage
//!
//! To use the trigonometric functions in this module, simply import the `trigonometry` module
//! into your Rust code and call the desired function. Here's an example:
//!
//! ```rust
//! use eratosthenes::trigonometry;
//!
//! fn main() {
//!     let radian = 0.8;
//!
//!     let sine = trigonometry::sin(radian);
//!     let arc_sine = trigonometry::asin(radian);
//!
//!     println!("Sine of {}: {}", radian, sine);
//!     println!("Arc sine of {}: {}", sine, arc_sine);
//! }
//! ```
//!
//! **Note:** This package leverages the trigonometric functions provided by Rust's standard
//! library to offer convenient access to common trigonometric calculations. If you require
//! advanced or specialized trigonometric operations, consider exploring external libraries
//! that offer additional functionality tailored to your specific needs.

pub mod sin;
pub use self::sin::{sin, asin};

pub mod cos;
pub use self::cos::{cos, acos};

pub mod tan;
pub use self::tan::{tan, atan};

pub mod conversions;
pub use self::conversions::{deg_to_rad, rad_to_deg};