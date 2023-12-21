#![no_std]
#![no_main]

mod handler;

// `#[no_mangle]` is unsafe but is required to be able to call application entry point
#[allow(unsafe_code)]
#[no_mangle]
#[allow(clippy::missing_const_for_fn)]
extern "C" fn main() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
