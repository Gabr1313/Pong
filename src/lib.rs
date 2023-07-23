pub mod utils;
pub use crate::utils::*;
pub mod constants;
pub use crate::constants::*;
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
