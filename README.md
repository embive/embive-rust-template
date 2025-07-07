# Rust Template for Embive
A simple program that runs inside the Embive interpreter.

## Requirements
- Rust (stable)
- riscv32imac-unknown-none-elf target
    - `$ rustup target add riscv32imac-unknown-none-elf`
- cargo-binutils
    - `$ cargo install cargo-binutils`
- llvm-tools
    - `$ rustup component add llvm-tools`

## How to build
- Compile the project and generate the ELF file:
    - `$ cargo objcopy --release -- app.elf`

## How to run
- Create a new project
    - `$ cargo new embive-project && cd embive-project`
- Add Embive as a dependency
    - `$ cargo add embive`
- Copy the example from Embive's docs/readme.
- Change the code to: `const ELF_FILE: &[u8] = include_bytes!("../app.elf");`
- Copy the generated `app.elf` to your project
- Run it:  
    - `$ cargo run --release`

## Stack
By default, the stack size is set to 2048 bytes (0x800).  
You can change this by modifying the `STACK_SIZE` variable in the [linker script](memory.ld).

## Heap
The heap is set at the end of the memory space allocated by the application (after data and stack).  
As such, the heap size doesn't need to be known at link time, instead being able to grow as large
as the maximum memory available.

## RAM calculation
You can calculate the minimum amount of RAM needed by you application with the following equation:  
- `total_ram = data + bss`

To get the `data` and `bss` sizes, you can run:  
- `$ cargo size --release`
    - The stack size will be reported as part of the bss

The result should be something like this:
```
   text    data     bss     dec     hex filename
    376       4    2052    2432     980 embive-rust-template
```

For this result, our minimum RAM size then would be:  
- `total_ram = 4 + 2052 = 2056 bytes`
