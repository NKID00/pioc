# pioc

Assembler and disassembler for the PIOC (Programmable Protocol I/O Microcontroller) peripheral featured in CH32X035/3, CH32V205, CH32H417/6/5 and CH643.

The PIOC peripheral is documented in [CHRISC8B.pdf](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Manual) (Basic instructions, Assembly syntax) and [PIOC.pdf](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Manual) (Behavior of special instructions, Registers) from evaluation kit of CH32X035. It seems to be an embedded version of [CH537](https://www.wch-ic.com/products/CH537.html) microcontroller. Specifically it has:

- 16-bit fixed-length "RISC8B" ISA, 66 instructions, Mostly single-cycle
- Same clock freq as main core (max 48MHz on CH32X035/3)
- 2K words code ROM (reused from 4KB SRAM), 49 byte registers (33 general purpose)
- 2 I/O ports with single-cycle operations
- Manchester coding and Timer/Counter for PWM coding

WCH provides an official assembler [WASM53B.EXE](https://github.com/openwch/ch32x035/tree/main/EVT/EXAM/PIOC/Tool_Manual/Tool). Cross-checks against the official assembler have been done successfully on all assembly files from the evaluation kit. But there are still a few incompatibilities, see section [Differences from Official Assembler](#differences-from-official-assembler).

## Usage

Add `pioc` dependency in your `Cargo.toml`:

```toml
[dependencies]
pioc = { git = "https://github.com/NKID00/pioc.git", branch = "master" }
```

To embed a PIOC program, use `pioc!` macro:

```rust
use pioc::pioc;

// this example program toggles IO1 at 1/6 clock speed with 50% duty cycle
const ROM: [u8; 12] = pioc! {"
        BS SFR_PORT_DIR, SB_PORT_DIR1   ; set IO1 to output
LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
        NOP
        NOP
        BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
        JMP LOOP                        ; jump takes 2 cycles
"};
```

Or include an assembly file with `pioc_include!` macro:

```rust
use pioc::pioc_include;

const ROM: [u8; 12] = pioc_include!("ROM.ASM");
```

Export symbols to retrieve label address:

```rust
let (rom, func_a_addr, start_addr) = pioc! {"
            ORG 0x000
FUNC_A:     NOP

            ORG 0x100   ; subsequent instructions start at address 0x100
START:      NOP

", "FUNC_A", "START"};  // export these symbols
```

To build PIOC programs at runtime, it is possible to assemble on the fly by calling `pioc::assemble` (`std` required). But generally you don't want it to happen on an embedded device. Instead, prepare program snippets assembled at compile-time, then concatenate and modify the binary form programmatically. Or build programs with `pioc::Inst` enum representing instructions, then convert them into bytes by calling `pioc::Inst::to_bytes`.

## Command Line Interface

Install with cargo:

```
cargo install --git https://github.com/NKID00/pioc.git --branch master --locked pioc-cli
```

```
Usage: pioc [OPTIONS] <COMMAND>

Commands:
  as       Assemble a PIOC assembly file
  dis      Disassemble a PIOC binary file
  as-one   Assemble a single line of PIOC assembly
  dis-one  Disassemble a single PIOC instruction
  help     Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Turn debugging information on
  -h, --help        Print help
  -V, --version     Print version
```


## Differences from Official Assembler

- EOF indicator `END` is no longer necessary.
- Builtin constants from `PIOC_INC.ASM` are available by default and doesn't require manual inclusion.
- Expressions are parsed and evaluated conforming to regular C operator associativity and precedence, and may have parenthesis (human-readable expressions are always welcome!). This is quite different from the official assembler where every operator is associated from right to left, and parenthesis are not allowed.
