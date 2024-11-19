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

## RAM calculation
You can calculate the minimum amount of RAM needed by you application with the following equation:  
`total_ram = data + bss + stack`

To get the `data` and `bss` sizes, you can run:  
`$ cargo size --release`

The result should be something like this:
```
   text    data     bss     dec     hex filename
    172       4       0     176      b0 embive-rust-template
```

For this result, if we chose a stack size of 512 bytes, our minimum RAM size then would be:  
`total_ram = 4 + 0 + 512 = 516 bytes`