#![no_std]
#![no_main]

mod embive;

use core::ptr::addr_of;

use embive::syscall;

const CONST_DATA: i32 = 20;
static mut GLOBAL_DATA: i32 = 10;

// User's main function
// This is a simple example of a program that runs in the Embive interpreter
// Here, we are calling two system calls, one to get a value from a memory address and another to add two numbers
// The system calls must be implemented in the host application (check Embive example)
fn main() {
    // System Call 2: Get i32 value at address
    // The host will receive the GLOBAL_DATA address, read it from memory and return its value
    let result = syscall(2, addr_of!(GLOBAL_DATA) as i32, 0, 0, 0, 0, 0, 0);

    // Do something here

    if let Ok(value) = result {
        // System Call 1: Add two numbers (a0 + a1)
        let _result = syscall(1, value, CONST_DATA, 0, 0, 0, 0, 0);
    }
}
