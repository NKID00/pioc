# pioc

Assembler and disassembler for the PIOC (Programmable Protocol I/O Microcontroller) peripheral featured in CH32X035/3, CH32V205, CH32H417/6/5 and CH643.

The PIOC peripheral is documented in [CHRISC8B.pdf](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Manual) (Basic instructions, Assembly syntax) and [PIOC.pdf](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Manual) (Behavior of special instructions, Registers) from evaluation kit of CH32X035. It seems to be an embedded [CH537](https://www.wch-ic.com/products/CH537.html) microcontroller. Specifically it has:

- 16-bit fixed-length "RISC8B" ISA, 66 instructions, Mostly single-cycle
- Same clock freq as main core (max 48MHz on CH32X035/3)
- 2K words code ROM (reused from 4KB SRAM), 49 byte registers (33 general purpose)
- 2 I/O ports with single-cycle operations
- Manchester coding and Timer/Counter for PWM coding

WCH provides an official assembler [WASM53B.EXE](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Tool).

# Usage

Add `pioc` dependency in your `Cargo.toml`:

```toml
[dependencies]
pioc = { git = "https://github.com/NKID00/pioc.git", branch = "master" }
```

Include an compile-time assembled PIOC program as an array of u16.

```rust
use pioc::pioc;

const ROM: [u16; 2] = pioc! {"
    NOP
    NOP
"};
```

Include an compile-time assembled PIOC program from an assembly file as an array of u16.

```rust
use pioc::pioc_include;

const ROM: [u16; 2] = pioc_include!("ROM.ASM");
```

## Project Status

- [x] OpCodes and AST
- [x] Assemler
- [x] Disassembler
- [x] Pretty Printer for Program
- [x] proc-macro for inline PIOC program
