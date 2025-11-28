use pioc_core::OpCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Define(String, String),
    Origin(u16),
    Include(String),
    /// Optional label, and opcode
    Inst(Option<String>, OpCode),
}
