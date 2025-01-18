#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub mod handler;
mod never;

pub use never::Never;