// no rust standard library 
#![no_std]
// disable rust level entry points
#![no_main]
use core::panic::PanicInfo;

// dont mangle the name of this function 
//  mangleing is cryptographic has assigned to each function by rust
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is entry point, liker looks for function
    // named _start by default
    loop {}
}

// function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
