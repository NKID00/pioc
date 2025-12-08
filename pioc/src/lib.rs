pub use pioc_asm::*;
pub use pioc_core::*;

/// Include an compile-time assembled PIOC program as an array of u16.
///
/// ## Example
///
/// ```rust
/// use pioc::pioc;
///
/// const ROM: [u16; 2] = pioc! {"
///     NOP
///     NOP
/// "};
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! pioc {
    { $asm:literal } => {
        pioc_macros::pioc_inner!($asm)
    };
}

/// Include an compile-time assembled PIOC program from an assembly file as an array of u16.
///
/// ## Example
///
/// ```ignore
/// use pioc::pioc_include;
///
/// const ROM: [u16; 2] = pioc_include!("ROM.ASM");
/// ```
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! pioc_include {
    ($path:literal) => {
        pioc_macros::pioc_include_inner!($path)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pioc() {
        let prog = super::pioc! {"
            NOP
            NOP
        "};
        assert_eq!(prog, [0, 0]);
    }

    #[test]
    fn test_pioc_include() {
        let prog = super::pioc_include!("test.asm");
        assert_eq!(prog, [0, 0]);
    }
}
