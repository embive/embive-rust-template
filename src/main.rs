#![no_std]
#![no_main]

mod embive;

use core::{panic::PanicInfo, ptr::addr_of, sync::atomic::{AtomicI32, Ordering}};

use embive::{ebreak, enable_interrupts, syscall, wfi};

static CONST_DATA: i32 = 20;
static GLOBAL_DATA: AtomicI32 = AtomicI32::new(0);

/// Panics will simply exit the interpreter (ebreak)
/// Here we could also make a system call to send the panic info to the host
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    ebreak()
}

/// Interrupt handler
/// This function is called when an interruption occurs
#[no_mangle]
fn interrupt_handler(value: i32) {
    // Set GLOBAL_DATA to the received value
    GLOBAL_DATA.store(value, Ordering::SeqCst);
}

// User's main function
// This is a simple example of a program that runs in the Embive interpreter
// Here, we are calling two system calls, one to get a value from a memory address and another to add two numbers
// The system calls must be implemented in the host application (check Embive example)
fn main() {
    // Enable interrupts
    enable_interrupts();

    // System Call 2: Get i32 value at address
    // The host will receive the CONST_DATA address, read it from memory and return its value
    let result = syscall(2, &[addr_of!(CONST_DATA) as i32, 0, 0, 0, 0, 0, 0]);

    // Wait for an interrupt
    wfi();

    if let Ok(value) = result {
        // System Call 1: Add two numbers (a0 + a1)
        let _result = syscall(1, &[value, GLOBAL_DATA.load(Ordering::SeqCst), 0, 0, 0, 0, 0]);
    }
}
