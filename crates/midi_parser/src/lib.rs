#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
use panic_probe as _;

pub mod parser;
pub mod tables;
pub mod consts;
