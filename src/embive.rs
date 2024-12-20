#![allow(dead_code)]

use core::{
    arch::{asm, global_asm},
    mem::zeroed,
    num::NonZeroI32,
    ptr::{addr_of_mut, read, write_volatile},
};

/// Number of syscall arguments
pub const SYSCALL_ARGS: usize = 7;

/// System Call. Must be implemented by the host.
///
/// Parameters:
/// - nr: System call number
/// - args: Array of arguments
///
/// Returns:
/// - Ok(value): The system call was successful.
/// - Err(error): The system call failed.
pub fn syscall(nr: i32, args: &[i32; SYSCALL_ARGS]) -> Result<i32, NonZeroI32> {
    let error: i32;
    let value: i32;

    unsafe {
        asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") args[0] => error,
            inlateout("a1") args[1] => value,
            in("a2") args[2],
            in("a3") args[3],
            in("a4") args[4],
            in("a5") args[5],
            in("a6") args[6],
            options(nostack),
        );
    }

    match NonZeroI32::new(error) {
        Some(error) => Err(error),
        None => Ok(value),
    }
}

/// Wait For Interrupt
///
/// Ask the host to put the interpreter to sleep until an interruption occurs
/// May return without any interruption.
#[inline(always)]
pub fn wfi() {
    unsafe {
        asm!("wfi", options(nostack));
    }
}

/// Exit the interpreter
#[inline(always)]
pub fn ebreak() -> ! {
    unsafe {
        asm!("ebreak", options(nostack, noreturn));
    }
}

/// Enable Interrupts
///
/// Set the `mstatus.MIE` bit to 1
#[inline(always)]
pub fn enable_interrupts() {
    unsafe {
        asm!("csrsi mstatus, 8", options(nostack));
    }
}

/// Disable Interrupts
///
/// Set the `mstatus.MIE` bit to 0
#[inline(always)]
pub fn disable_interrupts() {
    unsafe {
        asm!("csrci mstatus, 8", options(nostack));
    }
}

// Binary entry point
// Initializes the global, stack, and frame pointers; and then calls the _code_entry function
global_asm! {
    ".section .text.init.entry, \"ax\"",
    ".global _entry",
    "_entry:",
    ".option push",
    ".option norelax",
    // Initialize global pointer
    "la gp, __global_pointer$",
    // Set interrupt trap
    "la t0, _interrupt_trap",
    "csrw mtvec, t0",
    // Enable embive interrupt (mie bit 16)
    "la t0, 65536",
    "csrw mie, t0",
    // Initialize stack and frame pointers
    "la t1, __stack_start",
    "andi sp, t1, -16",
    "add s0, sp, zero",
    ".option pop",
    // Call _code_entry
    "jal ra, _code_entry",
}

// Interrupt trap
global_asm! {
    ".option push",
    ".balign 0x4",
    ".option norelax",
    ".option norvc",
    "_interrupt_trap:",
    // Save registers
    "addi sp, sp, -16*4",
    "sw ra, 0*4(sp)",
    "sw t0, 1*4(sp)",
    "sw t1, 2*4(sp)",
    "sw t2, 3*4(sp)",
    "sw t3, 4*4(sp)",
    "sw t4, 5*4(sp)",
    "sw t5, 6*4(sp)",
    "sw t6, 7*4(sp)",
    "sw a0, 8*4(sp)",
    "sw a1, 9*4(sp)",
    "sw a2, 10*4(sp)",
    "sw a3, 11*4(sp)",
    "sw a4, 12*4(sp)",
    "sw a5, 13*4(sp)",
    "sw a6, 14*4(sp)",
    "sw a7, 15*4(sp)",
    // Call interrupt handler
    "jal ra, interrupt_handler",
    // Restore registers
    "lw ra, 0*4(sp)",
    "lw t0, 1*4(sp)",
    "lw t1, 2*4(sp)",
    "lw t2, 3*4(sp)",
    "lw t3, 4*4(sp)",
    "lw t4, 5*4(sp)",
    "lw t5, 6*4(sp)",
    "lw t6, 7*4(sp)",
    "lw a0, 8*4(sp)",
    "lw a1, 9*4(sp)",
    "lw a2, 10*4(sp)",
    "lw a3, 11*4(sp)",
    "lw a4, 12*4(sp)",
    "lw a5, 13*4(sp)",
    "lw a6, 14*4(sp)",
    "lw a7, 15*4(sp)",
    "addi sp, sp, 16*4",
    // Return from trap
    "mret",
    ".option pop",
}

/// This code is responsible for initializing the .bss and .data sections, and calling the user's main function.
/// Based on: https://interrupt.memfault.com/blog/zero-to-main-rust-1
#[no_mangle]
unsafe extern "C" fn _code_entry() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut __bss_target_start: u32; // Start of .bss target
        static mut __bss_target_end: u32; // End of .bss target
        static mut __data_target_start: u32; // Start of .data target
        static mut __data_target_end: u32; // End of .data target
        static __data_source_start: u32; // Start of .data source
    }

    // Initialize (Zero) BSS
    let mut sbss: *mut u32 = addr_of_mut!(__bss_target_start);
    let ebss: *mut u32 = addr_of_mut!(__bss_target_end);

    while sbss < ebss {
        write_volatile(sbss, zeroed());
        sbss = sbss.offset(1);
    }

    // Initialize Data
    let mut sdata: *mut u32 = addr_of_mut!(__data_target_start);
    let edata: *mut u32 = addr_of_mut!(__data_target_end);
    let mut sdatas: *const u32 = &__data_source_start;

    while sdata < edata {
        write_volatile(sdata, read(sdatas));
        sdata = sdata.offset(1);
        sdatas = sdatas.offset(1);
    }

    // Call user's main function
    crate::main();

    // Exit the interpreter
    ebreak()
}
