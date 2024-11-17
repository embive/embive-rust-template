use core::{arch::asm, mem::zeroed, panic::PanicInfo, ptr::{addr_of_mut, read, write_volatile}};

// Panics will simply exit the interpreter (ebreak)
// Here we could also make a system call to send the panic info to the host
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!("ebreak");
    }
    loop {}
}

/// System Call. Check [syscall(2)](https://man7.org/linux/man-pages/man2/syscall.2.html). Must be implemented by the host.
pub fn syscall(nr: i32, a0: i32, a1: i32, a2: i32, a3: i32, a4: i32, a5: i32) -> (i32, i32) {
    let ret1: i32;
    let ret2: i32;
    unsafe {
        asm!(
            "ecall",
            in("a7") nr,
            inlateout("a0") a0 => ret1,
            inlateout("a1") a1 => ret2,
            in("a2") a2,
            in("a3") a3,
            in("a4") a4,
            in("a5") a5,
        );
    }
    (ret1, ret2)
}

// Code execution starts here. Embive initializes the stack pointer and jumps to this address.
// This code is responsible for initializing the .bss and .data sections and calling the user's main function.
// From: https://interrupt.memfault.com/blog/zero-to-main-rust-1
#[link_section = ".entry"]
#[no_mangle]
fn entry() -> ! {
    extern "C" {
        // These symbols come from `linker.ld`
        static mut _sbss: u32; // Start of .bss section
        static mut _ebss: u32; // End of .bss section
        static mut _sdata: u32; // Start of .data section
        static mut _edata: u32; // End of .data section
        static _srodata: u32; // Start of .rodata section
    }

    // Initialize (Zero) BSS
    unsafe {
        let mut sbss: *mut u32 = addr_of_mut!(_sbss);
        let ebss: *mut u32 = addr_of_mut!(_ebss);

        while sbss < ebss {
            write_volatile(sbss, zeroed());
            sbss = sbss.offset(1);
        }
    }

    // Initialize Data
    unsafe {
        let mut sdata: *mut u32 = addr_of_mut!(_sdata);
        let edata: *mut u32 = addr_of_mut!(_edata);
        let mut rodata: *const u32 = &_srodata;

        while sdata < edata {
            write_volatile(sdata, read(rodata));
            sdata = sdata.offset(1);
            rodata = rodata.offset(1);
        }
    }

    // Call user's main function
    crate::main();

    // Exit the interpreter
    unsafe {
        asm!("ebreak");
    }
    loop {}
}