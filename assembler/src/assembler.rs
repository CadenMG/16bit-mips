use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use ir::Register;
use ir::Directive;
use ir::RType;
use ir::JType;
use ir::IType;
use ir::Pseudo;
use ir::IRInstruction;

const INSTR_SIZE: u16 = 1;
static MIF_HEADER: &'static str = 
"DEPTH = 16384;                -- The size of memory in words
WIDTH = 16;                   -- The size of data in bits
ADDRESS_RADIX = HEX;          -- The radix for address values
DATA_RADIX = BIN;             -- The radix for data values
CONTENT                       -- start of (address : data pairs)
BEGIN\n";



pub fn parse(text: String, output_file: String) -> std::io::Result<()> {
    let mut out_file = File::create(&output_file)?;
    let ir1 = to_ir1(text);
    let parsed = to_mif(to_ir2(ir1.0, ir1.1));
    out_file.write_all(parsed.as_bytes())?;
    Ok(())
}

/*
    First pass of the assembler. Parses into IR and creates a symbol 
    table (map of symbol to address).
*/
fn to_ir1(text: String) -> (Vec<(u16, IRInstruction)>, HashMap<String, u16>) {
    let mut addr_counter: u16 = 0;
    let mut symbol_table: HashMap<String, u16> = HashMap::new();
    let mut instrs: Vec<(u16, IRInstruction)> = Vec::new();
    for line in text.lines() {
        let instr = parse_line(line);
        instrs.push((addr_counter, instr.clone()));
        match instr { // todo: handle pseudo-instr
            IRInstruction::Label(name) => {
                symbol_table.insert(name.to_string(), addr_counter.clone());
                addr_counter += INSTR_SIZE;
            },
            IRInstruction::Directive(dir) => {
                match dir {
                    Directive::Asciiz(s) => addr_counter += s.len() as u16,
                    Directive::Org(new_cnt) => addr_counter = new_cnt,
                    Directive::Space(space) => addr_counter += space,
                    Directive::Byte(_) => addr_counter += INSTR_SIZE,
                    Directive::Word(_) => addr_counter += INSTR_SIZE,
                }
            }
            _ => addr_counter += INSTR_SIZE,
        }
    }
    (instrs, symbol_table)
}

/*
    Second pass of the assembler. Replaces symbols with actual address
    and replaces pseudo-instrs. with real instrs.
*/
fn to_ir2(instrs: Vec<(u16, IRInstruction)>, symbols: HashMap<String, u16>) -> Vec<(u16, IRInstruction)> {
    instrs
        .iter()
        .filter(|(_, instr)| {
            match instr {
                IRInstruction::Directive(dir) => {
                    match dir {
                        Directive::Space(_) => false,
                        Directive::Org(_) => false,
                        _ => true,
                    }
                },
                IRInstruction::Label(_) | IRInstruction::BlankLine => false,
                _ => true
            }
        })
        .fold(vec![], |mut acc, (addr, instr)| {
            match instr {
                IRInstruction::JTypeLabel(j_type, label) => {
                    acc.push((*addr, IRInstruction::JTypeAddr(j_type.clone(), *symbols.get(label).unwrap()+INSTR_SIZE)));
                },
                IRInstruction::Pseudo(pseudo) => {
                    match pseudo {
                        Pseudo::LI(reg, val) => {
                            let lower = (val & 0xFF) as i8;
                            let upper = (val >> 8) as i8;
                            acc.push((*addr, IRInstruction::IType(IType::LLI, reg.clone(), Register::None, lower)));
                            acc.push((*addr + INSTR_SIZE, IRInstruction::IType(IType::LUI, reg.clone(), Register::None, upper)));
                        }
                    }
                },
                _ => {
                    acc.push((*addr, instr.clone()));
                }
            }
            acc
        })
}

/*
    Final pass of the assembler. Adds MIF preamble and translates instructions
    to binary.
*/
fn to_mif(instrs: Vec<(u16, IRInstruction)>) -> String {
    MIF_HEADER.to_owned() +
    &instrs
        .iter()
        .map(|(addr, instr)| instr_to_mif(addr, instr))
        .collect::<Vec<_>>()
        .join("\n") +
    "\nEND;\n"
}

pub fn instr_to_mif(addr: &u16, instr: &IRInstruction) -> String {
    match instr {
        IRInstruction::Directive(dir) => {
                match dir {
                    Directive::Asciiz(s) => {
                        let bytes: String = 
                            s
                            .as_bytes()
                            .into_iter()
                            .map(|b| format!("{:X}", b))
                            .fold(String::new(), |a, b| a + " " + &b);
                        return format!("{:X} : {}; -- {}", addr, bytes, instr);
                    },
                   _ => format!("{:X} : {:X}; -- {}", addr, instr_to_int(instr), instr), 
                }
        },
        _ => format!("{:X} : {:X}; -- {}", addr, instr_to_int(instr), instr),
    }
    
}

fn instr_to_int(instr: &IRInstruction) -> u16 {
    match instr {
        IRInstruction::Directive(dir) => {
            return match dir {
                Directive::Byte(b) => *b as u16,
                Directive::Word(w) => *w as u16,
                _ => 0,
            }
        },
        IRInstruction::RType(rtype, reg1, reg2, reg3) => {
            return (reg_to_int(reg1) << 9) 
            + (reg_to_int(reg2) << 6) 
            + (reg_to_int(reg3) << 3)
            + rtype_to_int(rtype)
        },
        IRInstruction::JTypeAddr(jtype, addr) => {
            return (jtype_to_int(jtype) << 12) + addr
        },
        IRInstruction::IType(itype, reg1, reg2, immediate) => {
            return (itype_to_int(itype) << 12)
            + (reg_to_int(reg1) << 9)
            + (reg_to_int(reg2) << 6)
            + ((*immediate as u8) as u16)
        },
        _ => panic!("Unexpected instruction type"),
    } 
}

fn rtype_to_int(rtype: &RType) -> u16 {
    match rtype {
        RType::Add => 0,
        RType::Sub => 1,
        RType::And => 2,
        RType::Or => 3,
        RType::Nor => 4,
        RType::SLL => 5,
        RType::SRL => 6,
        RType::SRA => 7,
    }
}

fn jtype_to_int(jtype: &JType) -> u16 {
    match jtype {
        JType::Jmp => 1,
        JType::Jal => 2,
    }
}

fn itype_to_int(itype: &IType) -> u16 {
    match itype {
        IType::LW => 3,
        IType::SW => 4,
        IType::BEQ => 5,
        IType::BNE => 6,
        IType::AddI => 7,
        IType::JmpI => 8,
        IType::JalI => 9,
        IType::LLI => 10,
        IType::LUI => 11,
    }
}

fn reg_to_int(reg: &Register) -> u16 {
    match reg {
        Register::Zero => 0,
        Register::At => 1,
        Register::V0 => 2,
        Register::V1 => 3,
        Register::A0 => 4,
        Register::A1 => 5,
        Register::Sp => 6,
        Register::Ra => 7,
        Register::None => 0,
    }
}

/*
    Parses the given line into an IRInstruction.
    A line may be blank
    A line may be just a comment
    A line may be just a label
    A line may contain a single instruction
    A line may contain a pseudo instruction
    A line may contain an assembler directive
*/
pub fn parse_line(raw_line: &str) -> IRInstruction {
    // remove text after '#' - as that's a comment
    let line = raw_line.trim();
    let comment_idx = line.find("#");
    let line_wo_comment = match comment_idx {
        None => line,
        Some(idx) => &line[..idx],
    };
    // remove extra whitespace
    let parts: Vec<&str> = line_wo_comment
        .split_whitespace()
        .filter(|&s| !s.chars().all(|c| char::is_ascii_whitespace(&c)))
        .collect();

    // blank/empty lines
    if line_wo_comment.len() == 0 || parts.len() == 0 {
        return IRInstruction::BlankLine
    }

    // check for label
    let colon_idx = parts.get(0).unwrap().find(":");
    if colon_idx.is_some() {
        let label = parts.get(0).unwrap()[..colon_idx.unwrap()].to_string();
        return IRInstruction::Label(label);
    }

    // check for assembler directive
    let dot_idx = parts.get(0).unwrap().find(".");
    if dot_idx.is_some() {
        let directive = &parts.get(0).unwrap()[dot_idx.unwrap()+1..];
        return IRInstruction::Directive(parse_directive(&directive, &parts[1..].to_vec()));
    } 

    // parse a single instruction
    parse_instruction(parts.get(0).unwrap(), &parts[1..].to_vec())

}

fn parse_instruction(instruction: &str, args: &Vec<&str>) -> IRInstruction {
    match instruction {
        "add" | "sub" | "and" | "or" | "nor" | "sll" | "srl" | "sra" => {
            let r1 = args.get(0).unwrap();
            let r2 = args.get(1).unwrap();
            let r3 = args.get(2).unwrap();
            IRInstruction::RType(
                parse_rtype(instruction),
                parse_register(&r1[..r1.len()-1]),
                parse_register(&r2[..r2.len()-1]),
                parse_register(r3)
            )
        },
        "jmp" | "jal" => {
            let addr_or_label = args.get(0).unwrap().trim_start_matches("0x");
            let jtype = parse_jtype(instruction);
            if args.get(0).unwrap().len() != addr_or_label.len() {
                return IRInstruction::JTypeAddr(jtype, u16::from_str_radix(addr_or_label, 16)
                    .expect("Error parsing address in j-type instruction"));
            } else if addr_or_label.chars().all(char::is_numeric) {
                return IRInstruction::JTypeAddr(jtype, u16::from_str_radix(addr_or_label, 10)
                    .expect("Error parsing address in j-type instruction"));
            }
            IRInstruction::JTypeLabel(jtype, addr_or_label.to_string())
        },
        "lw" | "sw" => {
            // ex instruction: lw $v0, 2($a0)
            let im_and_reg: Vec<&str> = args.get(1).unwrap().split("(").collect();
            let im = parse_num(im_and_reg.get(0).unwrap()) as i8;
            let reg = &im_and_reg.get(1).unwrap()[..im_and_reg.get(1).unwrap().len()-1];
            IRInstruction::IType(
                parse_itype(instruction),
                parse_register(&args.get(0).unwrap()[..args.get(0).unwrap().len()-1]),
                parse_register(reg),
                im
            )
        },
        "beq" | "bne" | "addi" => {
            // ex instruction: bne $v0, $a0, 2
            IRInstruction::IType(
                parse_itype(instruction),
                parse_register(&args.get(0).unwrap()[..args.get(0).unwrap().len()-1]),
                parse_register(&args.get(1).unwrap()[..args.get(1).unwrap().len()-1]),
                parse_num(args.get(2).unwrap()) as i8
            )
        },
        "jmpi" | "jali" => {
            // ex instruction: jmpi 2($v0)
            let im_and_reg: Vec<&str> = args.get(0).unwrap().split("(").collect();
            let im = parse_num(im_and_reg.get(0).unwrap()) as i8;
            let reg = &im_and_reg.get(1).unwrap()[..im_and_reg.get(1).unwrap().len()-1];
            IRInstruction::IType(
                parse_itype(instruction),
                parse_register(reg),
                Register::None,
                im
            )
        },
        "lli" | "lui" => {
            // ex instruction: lli $v0, 2
            IRInstruction::IType(
                parse_itype(instruction),
                parse_register(&args.get(0).unwrap()[..args.get(0).unwrap().len()-1]),
                Register::None,
                parse_num(args.get(1).unwrap()) as i8

            )
        },
        "li" => {
            // ex instruction: li $v0, 0x1000
            IRInstruction::Pseudo(
                parse_pseudo(instruction, args)
            )
        }
        _ => panic!("Unrecognized instruction type {}", instruction),
    }
}

fn parse_register(register: &str) -> Register {
    match register {
        "$0" => Register::Zero,
        "$at" => Register::At,
        "$v0" => Register::V0,
        "$v1" => Register::V1,
        "$a0" => Register::A0,
        "$a1" => Register::A1,
        "$sp" => Register::Sp,
        "$ra" => Register::Ra,
        _ => panic!("Unrecognized register type {}", register),
    }
}

fn parse_directive(directive: &str, args: &Vec<&str>) -> Directive {
    match directive {
            "space" => {
                let num_bytes = parse_num_unsigned(args.get(0).unwrap());
                Directive::Space(num_bytes)
            }
            "word" => {
                let word = parse_num_unsigned(args.get(0).unwrap());
                Directive::Word(word)
            }
            "byte" => {
                let byte = parse_num_unsigned(args.get(0).unwrap()) as u8;
                Directive::Byte(byte)
            }
            "asciiz" => {
                let s: String = args.join(" ");
                if s.chars().nth(0).unwrap() != '"' || s.chars().nth(s.len()-1).unwrap() != '"' {
                    panic!("String not surrounded in quotes within asciiz directive");
                }
                Directive::Asciiz(s[1..s.len()-1].to_string())
            }
            "org" => {
                let new_counter = parse_num_unsigned(args.get(0).unwrap());
                Directive::Org(new_counter)
            },
            _ => panic!("Unexpected directive keyword {}", directive),
        }
}

fn parse_rtype(instruction: &str) -> RType {
    match instruction {
        "add" => RType::Add,
        "sub" => RType::Sub,
        "and" => RType::And,
        "or" => RType::Or,
        "nor" => RType::Nor,
        "sll" => RType::SLL,
        "srl" => RType::SRL,
        "sra" => RType::SRA,
        _ => panic!("Unrecognized r-type instruction {}", instruction),
    }
}

fn parse_jtype(instruction: &str) -> JType {
    match instruction {
        "jmp" => JType::Jmp,
        "jal" => JType::Jal,
        _ => panic!("Unrecognized j-type instruction {}", instruction),
    }
}

fn parse_itype(instruction: &str) -> IType {
    match instruction {
        "lw" => IType::LW,
        "sw" => IType::SW,
        "beq" => IType::BEQ,
        "bne" => IType::BNE,
        "addi" => IType::AddI,
        "jmpi" => IType::JmpI,
        "jali" => IType::JalI,
        "lli" => IType::LLI,
        "lui" => IType::LUI,
        _ => panic!("Unrecognized i-type instruction {}", instruction),
    }
}

fn parse_pseudo(instruction: &str, args: &Vec<&str>) -> Pseudo {
    match instruction {
        "li" => Pseudo::LI(
            parse_register(&args.get(0).unwrap()[..args.get(0).unwrap().len()-1]),
            parse_num(&args.get(1).unwrap())
        ),
        _ => panic!("Unrecognized pseudo instruction {}", instruction),
    }
}

fn parse_num_unsigned(expr: &str) -> u16 {
    if expr.starts_with("0x") {
        u16::from_str_radix(expr.trim_start_matches("0x"), 16)
            .expect(&format!("Error parsing number: {}", expr))
    } else {
        u16::from_str_radix(expr, 10)
            .expect(&format!("Error parsing number: {}", expr))
    }
}

fn parse_num(expr: &str) -> i16 {
    if expr.starts_with("0x") {
        i16::from_str_radix(expr.trim_start_matches("0x"), 16)
            .expect(&format!("Error parsing number: {}", expr))
    } else {
        i16::from_str_radix(expr, 10)
            .expect(&format!("Error parsing number: {}", expr))
    }
}

#[cfg(test)]
mod tests {
    use crate::{assembler::parse_line, ir::*};

    #[test]
    fn parse_empty_line() {
        assert_eq!(parse_line(""), IRInstruction::BlankLine);
        assert_eq!(parse_line(" "), IRInstruction::BlankLine);
        assert_eq!(parse_line("  "), IRInstruction::BlankLine);
        assert_eq!(parse_line("\t"), IRInstruction::BlankLine);
        assert_eq!(parse_line("\t "), IRInstruction::BlankLine);
    }

    #[test]
    fn parse_empty_line_comment() {
        assert_eq!(parse_line("#"), IRInstruction::BlankLine);
        assert_eq!(parse_line(" #"), IRInstruction::BlankLine);
        assert_eq!(parse_line("# abcd"), IRInstruction::BlankLine);
        assert_eq!(parse_line(" # abcd"), IRInstruction::BlankLine);
        assert_eq!(parse_line(" # abcd "), IRInstruction::BlankLine);
    }

    #[test]
    fn parse_label() {
        assert_eq!(parse_line("abc:"), IRInstruction::Label(String::from("abc")));
        assert_eq!(parse_line(" abc:"), IRInstruction::Label(String::from("abc")));
        assert_eq!(parse_line(" abc: "), IRInstruction::Label(String::from("abc")));
        assert_eq!(parse_line("\tabc:"), IRInstruction::Label(String::from("abc")));
        assert_eq!(parse_line("abc: # comment"), IRInstruction::Label(String::from("abc")));
    }

    #[test]
    fn parse_directive() {
        assert_eq!(parse_line(".space 1"), IRInstruction::Directive(Directive::Space(1)));
        assert_eq!(parse_line(".word 1"), IRInstruction::Directive(Directive::Word(1)));
        assert_eq!(parse_line(".byte 1"), IRInstruction::Directive(Directive::Byte(1)));
        assert_eq!(parse_line(".asciiz \"abc\""), IRInstruction::Directive(Directive::Asciiz(String::from("abc"))));
        assert_eq!(parse_line(".asciiz \"abc\""), IRInstruction::Directive(Directive::Asciiz(String::from("abc"))));
    }

    #[test]
    fn parse_rtype() {
        assert_eq!(parse_line("add $v0, $a0, $a1"),
            IRInstruction::RType(RType::Add, Register::V0, Register::A0, Register::A1));
        assert_eq!(parse_line("sub $v0, $a0, $a1"),
            IRInstruction::RType(RType::Sub, Register::V0, Register::A0, Register::A1));
        assert_eq!(parse_line("or $v0, $a0, $a1"),
            IRInstruction::RType(RType::Or, Register::V0, Register::A0, Register::A1));
    }

    #[test]
    fn parse_jtype_with_address() {
        assert_eq!(parse_line("jmp 1"), IRInstruction::JTypeAddr(JType::Jmp, 1));
        assert_eq!(parse_line("jal 1"), IRInstruction::JTypeAddr(JType::Jal, 1));
        assert_eq!(parse_line("jmp 0x1"), IRInstruction::JTypeAddr(JType::Jmp, 1));
        assert_eq!(parse_line("jal 0x1"), IRInstruction::JTypeAddr(JType::Jal, 1));
    }

    #[test]
    fn parse_jtype_with_label() {
        assert_eq!(parse_line("jmp abc"), IRInstruction::JTypeLabel(JType::Jmp, String::from("abc")));
        assert_eq!(parse_line("jal abc"), IRInstruction::JTypeLabel(JType::Jal, String::from("abc")));
    }

    #[test]
    fn parse_itype_two_reg_and_offset() {
        assert_eq!(
            parse_line("lw $v0, 2($a0)"), 
            IRInstruction::IType(IType::LW, Register::V0, Register::A0, 2));
        assert_eq!(
            parse_line("sw $v0, 2($a0)"), 
            IRInstruction::IType(IType::SW, Register::V0, Register::A0, 2));
    }

    #[test]
    fn parse_itype_two_reg_and_immediate() {
        assert_eq!(
            parse_line("bne $v0, $a0, 2"), 
            IRInstruction::IType(IType::BNE, Register::V0, Register::A0, 2));
        assert_eq!(
            parse_line("beq $v0, $a0, 2"), 
            IRInstruction::IType(IType::BEQ, Register::V0, Register::A0, 2));
        assert_eq!(
            parse_line("addi $v0, $a0, 2"), 
            IRInstruction::IType(IType::AddI, Register::V0, Register::A0, 2));
        assert_eq!(
            parse_line("addi $v0, $a0, -2"), 
            IRInstruction::IType(IType::AddI, Register::V0, Register::A0, -2));
    }

    #[test]
    fn parse_itype_jump() {
        assert_eq!(
            parse_line("jmpi 2($v0)"), 
            IRInstruction::IType(IType::JmpI, Register::V0, Register::None, 2));
        assert_eq!(
            parse_line("jmpi 0xA($v0)"), 
            IRInstruction::IType(IType::JmpI, Register::V0, Register::None, 10));
        assert_eq!(
            parse_line("jali 2($v0)"), 
            IRInstruction::IType(IType::JalI, Register::V0, Register::None, 2));
        assert_eq!(
            parse_line("jali 0xA($v0)"), 
            IRInstruction::IType(IType::JalI, Register::V0, Register::None, 10));
    }

    #[test]
    fn parse_itype_one_reg_and_immediate() {
        assert_eq!(
            parse_line("lli $v0, 2"), 
            IRInstruction::IType(IType::LLI, Register::V0, Register::None, 2));
        assert_eq!(
            parse_line("lli $v0, 0xA"), 
            IRInstruction::IType(IType::LLI, Register::V0, Register::None, 10));
        assert_eq!(
            parse_line("lui $v0, 2"), 
            IRInstruction::IType(IType::LUI, Register::V0, Register::None, 2));
        assert_eq!(
            parse_line("lui $v0, 0xA"), 
            IRInstruction::IType(IType::LUI, Register::V0, Register::None, 10));
    }

    #[test]
    fn parse_pseudo_instr() {
        assert_eq!(
            parse_line("li $v0, 0x100"), 
            IRInstruction::Pseudo(Pseudo::LI(Register::V0, 256)));
        assert_eq!(
            parse_line("li $v0, -256"), 
            IRInstruction::Pseudo(Pseudo::LI(Register::V0, -256)));
    }
}