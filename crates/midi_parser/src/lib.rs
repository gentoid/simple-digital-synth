#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
use panic_probe as _;

pub mod parser;
pub mod tables;
pub mod consts;
