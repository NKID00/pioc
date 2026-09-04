//! Assemble PIOC programs for WCH microcontrollers at compile-time.
//!
//! # Examples
//!
//! Include an compile-time assembled PIOC program as an array of [u16].
//!
//! ```rust
//! use pioc::pioc;
//!
//! // this example program toggles IO1 at 1/6 clock speed with 50% duty cycle
//! const ROM: [u8; 12] = pioc! {"
//!         BS SFR_PORT_DIR, SB_PORT_DIR1   ; set IO1 to output
//! LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
//!         NOP
//!         NOP
//!         BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
//!         JMP LOOP                        ; jump takes 2 cycles
//! "};
//! ```
//!
//! Include an compile-time assembled PIOC program from an assembly file as an array of [u16].
//!
//! ```rust
//! use pioc::pioc_include;
//!
//! const ROM: [u8; 12] = pioc_include!("ROM.ASM");
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub use pioc_asm::*;

pub use pioc_core::*;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub use pioc_macros as __inner_macros;

/// Include an compile-time assembled PIOC program as an array of [u16].
///
/// ## Example
///
/// ```rust
/// use pioc::pioc;
///
/// // this example program toggles IO1 at 1/6 clock speed with 50% duty cycle
/// const ROM: [u8; 12] = pioc! {"
///         BS SFR_PORT_DIR, SB_PORT_DIR1   ; set IO1 to output
/// LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
///         NOP
///         NOP
///         BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
///         JMP LOOP                        ; jump takes 2 cycles
/// "};
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! pioc {
    { $asm:literal } => {
        $crate::__inner_macros::pioc_inner!($asm)
    };
}

/// Include an compile-time assembled PIOC program from an assembly file as an array of [u16].
///
/// ## Example
///
/// ```rust
/// use pioc::pioc_include;
///
/// const ROM: [u8; 12] = pioc_include!("ROM.ASM");
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! pioc_include {
    ($path:literal) => {
        $crate::__inner_macros::pioc_include_inner!($path)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pioc() {
        let prog = super::pioc! {"
        BS SFR_PORT_DIR, SB_PORT_DIR1   ; set IO1 to output
LOOP:   BS SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to high
        NOP
        NOP
        BC SFR_PORT_IO, SB_PORT_OUT1    ; set IO1 to low
        JMP LOOP                        ; jump takes 2 cycles
        "};
        assert_eq!(
            prog,
            [0x0a, 0x49, 0x0b, 0x49, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x41, 0x01, 0x60]
        );
    }

    #[test]
    fn test_pioc_include() {
        let prog = super::pioc_include!("tests/test.asm");
        assert_eq!(
            prog,
            [0x0a, 0x49, 0x0b, 0x49, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x41, 0x01, 0x60]
        );
    }
}
