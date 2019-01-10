//! # Making Docs
//!
//! `makingdocs` is a crate for testing out docs and comments
//! Hello completely useless crate!

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, makingdocs::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod art;
