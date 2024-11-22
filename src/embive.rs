use core::{
    arch::{asm, global_asm},
    mem::zeroed,
    panic::PanicInfo,
    ptr::{addr_of_mut, read, write_volatile},
};

// Panics will simply exit the interpreter (ebreak)
// Here we could also make a system call to send the panic info to the host
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { asm!("ebreak", options(nostack, noreturn)) }
}

/// System Call. Must be implemented by the host.
pub fn syscall(
    nr: i32,
    a0: i32,
    a1: i32,
    a2: i32,
    a3: i32,
    a4: i32,
    a5: i32,
    a6: i32,
) -> Result<i32, i32> {
    let error: i32;
    let value: i32;

    unsafe {
        asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") a0 => error,
            inlateout("a1") a1 => value,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
            in("a6") a6,
            options(nostack),
        );
    }

    if error == 0 {
        Ok(value)
    } else {
        Err(error)
    }
}

// Binary entry point
// Initializes the global, stack, and frame pointers; and then calls the _code_entry function
global_asm! {
    ".section .text.init.entry, \"ax\"",
    ".global _entry",
    "_entry:",
    // Initialize global pointer
    ".option push",
    ".option norelax",
    "la gp, __global_pointer$",
    ".option pop",
    // Initialize stack and frame pointers
    "la t1, __stack_start",
    "andi sp, t1, -16",
    "add s0, sp, zero",
    // Call _code_entry
    "jal ra, _code_entry"
}

/// This code is responsible for initializing the .bss and .data sections, and calling the user's main function.
/// Based on: https://interrupt.memfault.com/blog/zero-to-main-rust-1
#[no_mangle]
unsafe fn _code_entry() -> ! {
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
    asm!("ebreak", options(nostack, noreturn))
}
