# Rust Template for Embive
A simple program that runs inside the Embive interpreter.

## Requirements
- Rust (stable)
- riscv32im-unknown-none-elf target
    - `$ rustup target add riscv32im-unknown-none-elf`
- cargo-binutils
    - `$ cargo install cargo-binutils`
- llvm-tools
    - `$ rustup component add llvm-tools`

## How to build
- Compile the project
    - `$ cargo build --release`
- Convert the output ELF to a flat binary
    - `$ cargo objcopy --release -- -O binary app.bin`

## How to run
- Create a new project
    - `$ cargo new embive-project && cd embive-project`
- Add Embive as a dependency
    - `$ cargo add embive`
- Copy the example from Embive's docs/readme.
- Swap the line `let code = ...` to `let code = include_bytes!("../app.bin");`
    - You can also remove the line `ram[..4].copy_from_slice...` (not needed / unused).
- Copy the generated `app.bin` to your project
- Run it:  
    - `$ cargo run --release`

## Stack Size
By default, the stack size is set to 512 bytes (0x200).  
You can change this by modifying the `STACK_SIZE` variable on the [linker script](memory.ld).  
The stack size should always be a multiple of 16 bytes.

## RAM calculation
You can calculate the minimum amount of RAM needed by you application with the following equation:  
- `total_ram = data + bss`

To get the `data` and `bss` sizes, you can run:  
- `$ cargo size --release`
    - The stack size will be reported as part of the bss

The result should be something like this:
```
   text    data     bss     dec     hex filename
    224       4     512     740     2e4 embive-rust-template
```

For this result, our minimum RAM size then would be:  
- `total_ram = 4 + 512 = 516 bytes`
