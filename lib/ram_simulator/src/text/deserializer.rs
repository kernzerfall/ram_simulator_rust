use std::io::{BufReader, BufRead};
use std::path::PathBuf;
use std::fs::File;

use regex::{Regex, Captures};
use lazy_static::lazy_static;
use crate::RegisterMachine;

use crate::comparison::Comparison;
use crate::instruction::*;

lazy_static!{
    static ref COND_JMP_PARSER: Regex = Regex::new(r"IF\s+[cC]\(0\)\s*(<|>|=|>=|<=)\s*([0-9]+)\s*THEN\s*GOTO\s*([0-9]+)")
        .unwrap();
}

macro_rules! gen_single_arg_instr {
    ($isv:expr, $instr:ident, $argtype:ident, $tkiter:expr, $line:expr) => {
        {
            match $tkiter.next() {
                    Some(s) => {
                        match s.parse::<$argtype>() {
                            Ok(arg) => $isv.push_instruction($instr::new(arg)),
                            Err(err) => return Err(
                                format!("Line {}: {} needs an argument of type {} [{}]", 
                                    $line, 
                                    stringify!($instr), 
                                    stringify!($argtype),
                                    err.to_string(),
                                )
                            ),
                        }
                    },
                    None =>{
                        return Err(format!("Line {}: {} needs an argument", $line, stringify!($instr)));
                    },
                }
        }
    };
}

pub fn parse_buf<R>(br: BufReader<R>) -> Result<RegisterMachine, String> where R: std::io::Read {
    let mut isv = InstructionVec::new();
    let mut initial_state = Vec::new();

    let mut init_cmd = false;
    let mut end_cmd = false;

    for (i, line) in br.lines().enumerate() {

        let current_line = line.unwrap();
        let mut tokens = current_line.split_whitespace();
        
        let next_token = tokens.next();
        if next_token.is_none() {
            return Err(format!("Expected an instruction on line {}", i));
        }

        let instruction = next_token.unwrap().trim();

        match instruction.trim().to_uppercase().as_str() {
            "INIT" => if i == 0 {
                    init_cmd = true;
                    for tkn in tokens {
                        match tkn.trim().parse::<u128>() {
                            Ok(val) => initial_state.push(val),
                            Err(u) => return Err(
                                format!("INIT args must be numbers [{}]", u.to_string())
                            )
                        }
                    }
                } else {
                    return Err("INIT called inside program".to_string())
                },

            "LOAD"      => gen_single_arg_instr!(isv, Load,     usize,  tokens, i),
            "INDLOAD"   => gen_single_arg_instr!(isv, IndLoad,  usize,  tokens, i),
            "CLOAD"     => gen_single_arg_instr!(isv, CLoad,    u128,   tokens, i),

            "STORE"     => gen_single_arg_instr!(isv, Store,    usize,  tokens, i),
            "INDSTORE"  => gen_single_arg_instr!(isv, IndStore, usize,  tokens, i),

            "ADD"       => gen_single_arg_instr!(isv, Add,      usize,  tokens, i),
            "INDADD"    => gen_single_arg_instr!(isv, IndAdd,   usize,  tokens, i),
            "CADD"      => gen_single_arg_instr!(isv, CAdd,     u128,   tokens, i),

            "SUB"       => gen_single_arg_instr!(isv, Sub,      usize,  tokens, i),
            "INDSUB"    => gen_single_arg_instr!(isv, IndSub,   usize,  tokens, i),
            "CSUB"      => gen_single_arg_instr!(isv, CSub,     u128,   tokens, i),

            "MULT"      => gen_single_arg_instr!(isv, Mult,     usize,  tokens, i),
            "INDMULT"   => gen_single_arg_instr!(isv, IndMult,  usize,  tokens, i),
            "CMULT"     => gen_single_arg_instr!(isv, CMult,    u128,   tokens, i),

            "DIV"       => gen_single_arg_instr!(isv, Div,      usize,  tokens, i),
            "INDDIV"    => gen_single_arg_instr!(isv, IndDiv,   usize,  tokens, i),
            "CDIV"      => gen_single_arg_instr!(isv, CDiv,     u128,   tokens, i),

            "GOTO" |
            "JMP" => gen_single_arg_instr!(isv, Jmp, usize, tokens, i),

            "END" => {
                end_cmd = true;
                isv.push_instruction(
                    End::new()
                )
            },

            "IF" => {
                let tokens: Captures;
                match COND_JMP_PARSER.captures(&current_line) { 
                    Some(val) => tokens = val,
                    None => return Err(
                        format!("Line {}: IF statement invalid", i)
                    )
                };

                let comp = Comparison::str_to_comp(&tokens[1]);
                let value: u128;
                match tokens[2].parse::<u128>() {
                    Ok(val) => value = val,
                    Err(u) => return Err(
                        format!("Line {}: IF condition must have an integer value [{}]", i, u.to_string())
                    )
                };

                let addr: usize;
                match tokens[3].parse::<usize>() {
                    Ok(val) => addr = val,
                    Err(u) => return Err(
                        format!("Line {}: IF condition must have an integer goto address [{}]", i, u.to_string())
                    )
                };

                isv.push_instruction(
                    CondJmp::new(comp, value, addr)
                )
            }

            _ => return Err(format!("Line {}: Unknown instruction {}", i, instruction)),
        };
    }

    if !init_cmd {
        return Err("The program must start with INIT, even if it's empty".to_string())
    }
    if !end_cmd {
        return Err("The program must have an END command somewhere".to_string())
    }

    let mut res = RegisterMachine::new(isv);
    res.push_vec(initial_state);
    Ok(res)
}

pub fn parse_file(path: PathBuf) -> Result<RegisterMachine, String> {
    match File::open(path) {
        Ok(f) => {
            let br = BufReader::new(f);
            parse_buf(br)
        },
        Err(u) => {
            Err(u.to_string())
        }
    }
}
