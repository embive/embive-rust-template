#![no_std]
#![no_main]

mod embive;

use core::ptr::addr_of;

use embive::syscall;

const CONST_DATA: i32 = 20;

// This will be placed at the start of RAM
#[link_section = ".data"]
static mut GLOBAL_DATA: i32 = 10;

// User's main function
fn main() {
    // System Call 2: Get i32 value at address (Must be implemented in your application, check Embive example)
    let (val, _error) = syscall(2, addr_of!(GLOBAL_DATA) as i32, 0, 0, 0, 0, 0);

    // System Call 1: Add two numbers (a0 + a1) (Must be implemented in your application, check Embive example)
    let (_res, _error) = syscall(1, val, CONST_DATA, 0, 0, 0, 0);
}
