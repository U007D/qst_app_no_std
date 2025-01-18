#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use lib::{error::Result, handler};

// `#[no_mangle]` is unsafe but is required to be able to call application entry point
#[allow(unsafe_code)]
#[cfg_attr(not(test), no_mangle)]
#[allow(clippy::missing_const_for_fn)]
extern "C" fn main() -> ! {
    let Err(err) = inner_main();
    panic!("{err}");
}

fn inner_main() -> Result<Never> {
    unimplemented!()
}
