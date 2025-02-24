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
- Swap the line `const ELF_FILE: &[u8] ...` to `const ELF_FILE: &[u8] = include_bytes!("../app.elf");`
- Copy the generated `app.elf` to your project
- Run it:  
    - `$ cargo run --release`

## Stack Size
By default, the stack size is set to 1024 bytes (0x400).  
You can change this by modifying the `STACK_SIZE` variable in the [linker script](memory.ld).  
The stack size should always be a multiple of 16 bytes.

## Heap Size
By default, the heap size is set to 1024 bytes (0x400).  
You can change this by modifying the `HEAP_SIZE` variable in the [linker script](memory.ld).  
The heap end address will always be aligned to 16 bytes.  
**Check `embive::get_heap` for more information about how to use the heap**  
**If not using the heap, you may set `HEAP_SIZE` to zero.**

## RAM calculation
You can calculate the minimum amount of RAM needed by you application with the following equation:  
- `total_ram = data + bss`

To get the `data` and `bss` sizes, you can run:  
- `$ cargo size --release`
    - The stack size will be reported as part of the bss

The result should be something like this:
```
   text    data     bss     dec     hex filename
    340       4    2060    2404     964 embive-rust-template
```

For this result, our minimum RAM size then would be:  
- `total_ram = 4 + 2060 = 2064 bytes`
