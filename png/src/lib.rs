#![forbid(unsafe_code)]

#[macro_use]
extern crate bitflags;

pub mod chunk;
mod common;
mod encoder;
mod filter;
mod srgb;
mod traits;
mod utils;

pub use crate::common::*;
pub use crate::encoder::{Encoder, EncodingError, StreamWriter, Writer};
pub use crate::filter::{AdaptiveFilterType, FilterType};
