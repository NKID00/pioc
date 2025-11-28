use pioc_core::OpCode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Include(String),
    Org(u16),
    RawWord(u16),
    Define(String, String),
    Label(String),
    Instruction(OpCode),
    End,
    RawAssembly(String), // unparsed
}
