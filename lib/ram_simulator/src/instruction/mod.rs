use crate::program::Comparison;
use crate::state::{State};

pub struct InstructionVec {
    pub instructions: Vec<Box<dyn Instruction>>
}

pub trait Instruction: StringRepr {
    fn exec(&self, state: &mut State);
}

pub trait StringRepr {
    fn command_name(&self) -> String;
    fn to_string(&self) -> String;
}

macro_rules! make_single_operand_instruction {
    ( $( $name:ident, $argtype:ident ), * ) => {
        $(
            #[derive(Debug)]
            pub struct $name {
                operand: $argtype,
            }

            impl $name {
                pub fn operand(&self) -> usize {
                    self.operand as usize
                }

                pub fn new(op: $argtype) -> $name {
                    $name {
                        operand: op
                    }
                }
            }

            impl StringRepr for $name {
                fn command_name(&self) -> String {
                    stringify!($name).to_string().to_uppercase()
                }

                fn to_string(&self) -> String {
                    format!("{} {}", self.command_name(), self.operand())
                }
            }
        )*
    };
}

pub mod load;
pub mod store;
pub mod add;
pub mod sub;
pub mod mult;
pub mod div;
pub mod jmp;
pub mod end;

make_single_operand_instruction![
    Load,       usize,
    CLoad,      u128,
    IndLoad,    usize,

    Store,      usize,
    IndStore,   usize,

    Add,        usize,
    CAdd,       u128,
    IndAdd,     usize,

    Sub,        usize,
    CSub,       u128,
    IndSub,     usize,

    Mult,       usize,
    CMult,      u128,
    IndMult,    usize,

    Div,        usize,
    CDiv,       u128,
    IndDiv,     usize,

    Jmp,        usize
];


pub struct End {

}

pub struct CondJmp {
    comparison:     Comparison,
    value:          u128,
    jmp_address:    usize,
}

impl InstructionVec {
    pub fn exec_instruction(&self, index: usize, state: &mut State) {
        self.instructions.get(index).unwrap().exec(state)
    }

    pub fn push_instruction<T>(&mut self, instruction: T) where T: Instruction + 'static {
        self.instructions.push(Box::new(instruction));
    }

    pub fn get_boxed_instruction(&self, index: usize) -> &Box<dyn Instruction> {
        self.instructions.get(index).unwrap()
    }

    pub fn new() -> InstructionVec {
        InstructionVec { instructions: Vec::new() }
    }
}

#[macro_export]
macro_rules! ivec_push_multiple {
    ( $vec:ident, $($act:expr), * ) => {
        $(
            $vec.push_instruction($act);
        )*
    };
}