#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Zero,
    At,
    V0,
    V1,
    A0,
    A1,
    Sp,
    Ra,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Directive {
    Org(u16), // set address counter
    Space(u16), // increment address counter
    Byte(u8), // store byte and increment address counter
    Word(u16), // store word and increment address counter
    Asciiz(String), // store string and increment address counter
}

#[derive(Debug, Clone, PartialEq)]
pub enum RType {
    Add,
    Sub,
    And,
    Or,
    Nor,
    SLL,
    SRL,
    SRA,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    Jmp,
    Jal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IType {
    LW,
    SW,
    BEQ,
    BNE,
    AddI,
    JmpI,
    JalI,
    LLI,
    LUI,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pseudo {
    LI(Register, i16), // Pseudo-instr for loading 16 bit immediates
}

#[derive(Debug, Clone, PartialEq)]
pub enum IRInstruction {
    Directive(Directive),
    Label(String),
    RType(RType, Register, Register, Register),
    JTypeLabel(JType, String),
    JTypeAddr(JType, u16),
    IType(IType, Register, Register, i8),
    Pseudo(Pseudo),
    BlankLine,
}

impl core::fmt::Display for Register {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Register::*;
        match self {
            Zero => write!(f, "$0"),
            At => write!(f, "$at"), 
            V0 => write!(f, "$v0"),
            V1 => write!(f, "$v1"),
            A0 => write!(f, "$a0"),
            A1 => write!(f, "$a1"),
            Sp => write!(f, "$sp"),
            Ra => write!(f, "$ra"),
            None => write!(f, "None"),
        }
    }
}

impl core::fmt::Display for Directive {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Directive::*;
        match self {
            Org(addr) => write!(f, ".org {}", addr),
            Space(space) => write!(f, ".space {}", space),
            Byte(b) => write!(f, ".byte {}", b),
            Word(w) => write!(f, ".word {}", w),
            Asciiz(s) => write!(f, ".asciiz {}", s),
        }
    }
}

impl core::fmt::Display for RType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::RType::*;
        match self {
            Add => write!(f, "add"),
            Sub => write!(f, "sub"),
            And => write!(f, "and"),
            Or => write!(f, "or"),
            Nor => write!(f, "nor"),
            SLL => write!(f, "sll"),
            SRL => write!(f, "srl"),
            SRA => write!(f, "sra"),
        }
    }
}

impl core::fmt::Display for JType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::JType::*;
        match self {
            Jmp => write!(f, "jmp"),
            Jal => write!(f, "jal"),
        }
    }
}

impl core::fmt::Display for IType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::IType::*;
        match self {
            LW => write!(f, "lw"),
            SW => write!(f, "sw"),
            BEQ => write!(f, "beq"),
            BNE => write!(f, "bne"),
            AddI => write!(f, "addi"),
            JmpI => write!(f, "jmpi"),
            JalI => write!(f, "jali"),
            LLI => write!(f, "lli"),
            LUI => write!(f, "lui"),
        }
    }
}

impl core::fmt::Display for Pseudo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::Pseudo::*;
        match self {
            LI(reg, im) => write!(f, "{}, {}", reg, im),
        }
    }
}

impl core::fmt::Display for IRInstruction {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use self::IRInstruction::*;
        match self {
            Directive(dir) => write!(f, "{}", dir),
            Label(s) => write!(f, "{}", s),
            RType(rtype, reg1, reg2, reg3) => 
                write!(f, "{}, {}, {}, {}", rtype, reg1, reg2, reg3),
            JTypeLabel(jtype, s) => write!(f, "{}, {}", jtype, s), 
            JTypeAddr(jtype, addr) => write!(f, "{}, {}", jtype, addr),
            IType(itype, reg1, reg2, im) 
                => write!(f, "{}, {}, {}, {}", itype, reg1, reg2, im),
            Pseudo(pseudo) => write!(f, "{}", pseudo),
            BlankLine => write!(f, "BlankLine"),
        }
    }
}