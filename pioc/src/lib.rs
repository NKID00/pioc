pub use pioc_asm::*;
pub use pioc_core::*;

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
}
