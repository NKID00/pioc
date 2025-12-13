use std::collections::BTreeMap;

use derive_more::{Deref, DerefMut};

use crate::{AssembleError, AssembleResult, Expr, Ident, Stmt};

#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct SymTab(BTreeMap<String, i32>);

impl SymTab {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl Default for SymTab {
    fn default() -> Self {
        Self(
            [
                ("SFR_INDIR_PORT", 0x00),
                ("SFR_INDIR_PORT2", 0x01),
                ("SFR_PRG_COUNT", 0x02),
                ("SFR_STATUS_REG", 0x03),
                ("SFR_INDIR_ADDR", 0x04),
                ("SFR_TMR0_COUNT", 0x05),
                ("SFR_TIMER_CTRL", 0x06),
                ("SFR_TMR0_INIT", 0x07),
                ("SFR_BIT_CYCLE", 0x08),
                ("SFR_INDIR_ADDR2", 0x09),
                ("SFR_PORT_DIR", 0x0A),
                ("SFR_PORT_IO", 0x0B),
                ("SFR_BIT_CONFIG", 0x0C),
                ("SFR_SYS_CFG", 0x1C),
                ("SFR_CTRL_RD", 0x1D),
                ("SFR_CTRL_WR", 0x1E),
                ("SFR_DATA_EXCH", 0x1F),
                ("SFR_DATA_REG0", 0x20),
                ("SFR_DATA_REG1", 0x21),
                ("SFR_DATA_REG2", 0x22),
                ("SFR_DATA_REG3", 0x23),
                ("SFR_DATA_REG4", 0x24),
                ("SFR_DATA_REG5", 0x25),
                ("SFR_DATA_REG6", 0x26),
                ("SFR_DATA_REG7", 0x27),
                ("SFR_DATA_REG8", 0x28),
                ("SFR_DATA_REG9", 0x29),
                ("SFR_DATA_REG10", 0x2A),
                ("SFR_DATA_REG11", 0x2B),
                ("SFR_DATA_REG12", 0x2C),
                ("SFR_DATA_REG13", 0x2D),
                ("SFR_DATA_REG14", 0x2E),
                ("SFR_DATA_REG15", 0x2F),
                ("SFR_DATA_REG16", 0x30),
                ("SFR_DATA_REG17", 0x31),
                ("SFR_DATA_REG18", 0x32),
                ("SFR_DATA_REG19", 0x33),
                ("SFR_DATA_REG20", 0x34),
                ("SFR_DATA_REG21", 0x35),
                ("SFR_DATA_REG22", 0x36),
                ("SFR_DATA_REG23", 0x37),
                ("SFR_DATA_REG24", 0x38),
                ("SFR_DATA_REG25", 0x39),
                ("SFR_DATA_REG26", 0x3A),
                ("SFR_DATA_REG27", 0x3B),
                ("SFR_DATA_REG28", 0x3C),
                ("SFR_DATA_REG29", 0x3D),
                ("SFR_DATA_REG30", 0x3E),
                ("SFR_DATA_REG31", 0x3F),
                ("SB_EN_TOUT_RST", 5),
                ("SB_STACK_USED", 4),
                ("SB_GP_BIT_Y", 3),
                ("SB_FLAG_Z", 2),
                ("SB_GP_BIT_X", 1),
                ("SB_FLAG_C", 0),
                ("SB_EN_LEVEL1", 7),
                ("SB_EN_LEVEL0", 6),
                ("SB_TMR0_ENABLE", 5),
                ("SB_TMR0_OUT_EN", 4),
                ("SB_TMR0_MODE", 3),
                ("SB_TMR0_FREQ2", 2),
                ("SB_TMR0_FREQ1", 1),
                ("SB_TMR0_FREQ0", 0),
                ("SB_BIT_TX_O0", 7),
                ("SB_BIT_CYCLE_6", 6),
                ("SB_BIT_CYCLE_5", 5),
                ("SB_BIT_CYCLE_4", 4),
                ("SB_BIT_CYCLE_3", 3),
                ("SB_BIT_CYCLE_2", 2),
                ("SB_BIT_CYCLE_1", 1),
                ("SB_BIT_CYCLE_0", 0),
                ("SB_PORT_MOD3", 7),
                ("SB_PORT_MOD2", 6),
                ("SB_PORT_MOD1", 5),
                ("SB_PORT_MOD0", 4),
                ("SB_PORT_PU1", 3),
                ("SB_PORT_PU0", 2),
                ("SB_PORT_DIR1", 1),
                ("SB_PORT_DIR0", 0),
                ("SB_PORT_IN_XOR", 7),
                ("SB_BIT_RX_I0", 6),
                ("SB_PORT_IN1", 5),
                ("SB_PORT_IN0", 4),
                ("SB_PORT_XOR1", 3),
                ("SB_PORT_XOR0", 2),
                ("SB_PORT_OUT1", 1),
                ("SB_PORT_OUT0", 0),
                ("SB_BIT_TX_EN", 7),
                ("SB_BIT_CODE_MOD", 6),
                ("SB_PORT_IN_EDGE", 5),
                ("SB_BIT_CYC_TAIL", 4),
                ("SB_BIT_CYC_CNT6", 3),
                ("SB_BIT_CYC_CNT5", 2),
                ("SB_BIT_CYC_CNT4", 1),
                ("SB_BIT_CYC_CNT3", 0),
                ("SB_INT_REQ", 7),
                ("SB_DATA_SW_MR", 6),
                ("SB_DATA_MW_SR", 5),
                ("SB_MST_CFG_B4", 4),
                ("SB_MST_IO_EN1", 3),
                ("SB_MST_IO_EN0", 2),
                ("SB_MST_RESET", 1),
                ("SB_MST_CLK_GATE", 0),
                ("BI_C_XOR_IN0", 0),
                ("BIO_FLAG_C", 0),
                ("BI_BIT_RX_I0", 1),
                ("BI_PORT_IN0", 2),
                ("BI_PORT_IN1", 3),
                ("BO_BIT_TX_O0", 1),
                ("BO_PORT_OUT0", 2),
                ("BO_PORT_OUT1", 3),
                ("WB_DATA_SW_MR_0", 0),
                ("WB_BIT_CYC_TAIL_1", 1),
                ("WB_PORT_I0_FALL", 2),
                ("WB_PORT_I0_RISE", 3),
                ("WB_DATA_MW_SR_1", 4),
                ("WB_PORT_XOR1_1", 5),
                ("WB_PORT_XOR0_0", 6),
                ("WB_PORT_XOR0_1", 7),
            ]
            .into_iter()
            .map(|(s, v)| (s.to_owned(), v))
            .collect(),
        )
    }
}

pub(crate) fn resolve_symbol(mut sym: SymTab, prog: &[Stmt]) -> AssembleResult<SymTab> {
    use Expr::*;
    use Stmt::*;

    let mut unresolved = BTreeMap::new();
    let mut origin = Expr::Num(0);
    let mut offset = 0;
    for stmt in prog {
        match stmt {
            Define(ident, expr) => {
                unresolved.insert(ident.0.clone(), expr.clone());
            }
            Origin(expr) => {
                origin = expr.clone();
                offset = 0;
            }
            Inst(Some(Ident(label)), _, _) => {
                match &origin {
                    Label(ident) => unresolved.insert(label.clone(), Add(ident.clone(), offset)),
                    Num(v) => unresolved.insert(label.clone(), Num(v + offset)),
                    Add(_, _) => unreachable!(),
                };
                offset += 2;
            }
            Inst(_, _, _) => offset += 2,
            Include(_) => unreachable!(),
        }
    }

    loop {
        let mut resolved = Vec::new();
        for (name, expr) in unresolved.iter() {
            match expr {
                Label(Ident(s)) => {
                    if let Some(v) = sym.get(s).cloned() {
                        sym.insert(name.clone(), v);
                        resolved.push(name.clone());
                    }
                }
                Num(v) => {
                    sym.insert(name.clone(), *v);
                    resolved.push(name.clone());
                }
                Add(Ident(s), b) => {
                    if let Some(a) = sym.get(s).cloned() {
                        sym.insert(name.clone(), a + *b);
                        resolved.push(name.clone());
                    }
                }
            }
        }
        if resolved.is_empty() {
            break;
        }
        for name in resolved {
            unresolved.remove(&name);
        }
    }

    if !unresolved.is_empty() {
        for (name, _) in unresolved {
            // 'A' and 'F' are valid operand for some instructions
            if name.to_lowercase() == "a" || name.to_lowercase() == "f" {
                continue;
            }
            return Err(AssembleError::SymbolResolveError(name));
        }
    }

    Ok(sym)
}

#[test]
fn test_resolve_symbol() {
    use crate::Expr::*;
    use crate::Mnemonic::*;
    use crate::Operand::*;
    use crate::Stmt::*;
    assert_eq!(
        resolve_symbol(
            SymTab::new(),
            &[
                Define("var0".into(), Num(42)),
                Define("var1".into(), Label("L1".into())),
                Inst(Some("L0".into()), NOP, Op0),
                Origin(Label("var0".into())),
                Inst(None, NOP, Op0),
                Inst(None, NOP, Op0),
                Inst(Some("L1".into()), NOP, Op0),
            ]
        )
        .unwrap(),
        SymTab(
            [("var0", 42), ("var1", 46), ("L0", 0), ("L1", 46),]
                .into_iter()
                .map(|(s, v)| (s.to_owned(), v))
                .collect()
        )
    );
}
