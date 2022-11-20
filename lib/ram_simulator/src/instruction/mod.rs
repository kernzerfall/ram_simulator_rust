use crate::comparison::Comparison;
use crate::state::{State};
use crate::text::Serializable;

/// Wrapper for a program (as a list of instructions)
pub struct InstructionVec {
    pub instructions: Vec<Box<dyn Instruction>>
}

/// Trait that defines an executable instruction
pub trait Instruction: StringRepr {
    fn exec(&self, state: &mut State);
}

/// Forces (most) instructions to have a defined string representation
// TODO: Implement this for the two special cases below
pub trait StringRepr {
    fn command_name(&self) -> String;
    fn to_string(&self) -> String;
}

/// Serialization functions for Instructions
impl <T> Serializable for T where T: Instruction {
    fn to_string(&self) -> String {
        self.to_string()
    }

    fn dump(&self) {
        println!("{}", self.to_string())
    }
}

/// Serialization functions for Instruction Vector Wrappers
impl Serializable for InstructionVec {
    fn to_string(&self) -> String {
        let mut res = String::new();
        for inst in self.instructions.iter() {
            res.push_str(inst.to_string().as_str());
            res.push('\n');
        }

        res
    }

    fn dump(&self) {
        for inst in self.instructions.iter() {
           println!("{}", inst.to_string().as_str());
        }
    }
}

/// Automatically generates single operand instructions
macro_rules! make_single_operand_instruction {
    ( $( $name:ident, $argtype:ident ), * ) => {
        $(
            #[derive(Debug, Clone, Copy)]
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

// Modules hold the concrete implementation of the instructions
// i.e. fn exec(...)
pub mod load;
pub mod store;
pub mod add;
pub mod sub;
pub mod mult;
pub mod div;
pub mod jmp;
pub mod end;

make_single_operand_instruction![
    /*
     *  Non-prefixed operations use the value of the register specified in the operand
     *  C-prefixed operations load/add/subtract/... a hard-coded constant to/from the accumulator
     *  IND-prefixed operations use the value of a referenced register (register -> register -> value)
     */

    /* Load values on to the accumulator (r0) */
    Load,       usize,
    CLoad,      u128,
    IndLoad,    usize,

    /* Store the value that the accumulator currently holds */
    Store,      usize,
    IndStore,   usize,

    /* ADD Functions */
    Add,        usize,
    CAdd,       u128,
    IndAdd,     usize,

    /* SUB Functions */
    Sub,        usize,
    CSub,       u128,
    IndSub,     usize,

    /* MULT Functions */
    Mult,       usize,
    CMult,      u128,
    IndMult,    usize,

    /* DIV Functions */
    Div,        usize,
    CDiv,       u128,
    IndDiv,     usize,

    /* JMP Instruction */
    Jmp,        usize
];


/* END instruction */
// This is the only instruction with no arguments: no point in defining a macro for it
pub struct End {}

/* Conditional Jump Instrunction */
// IF r0?value THEN GOTO jmp_address
pub struct CondJmp {
    comparison:     Comparison,
    value:          u128,
    jmp_address:    usize,
}

impl InstructionVec {
    /// Executes the instruction at `index` on the machine defined by `state`
    pub fn exec_instruction(&self, index: usize, state: &mut State) {
        self.instructions.get(index).unwrap().exec(state)
    }

    /// Pushes `instruction` on to the `InstructionVec`
    pub fn push_instruction<T>(&mut self, instruction: T) where T: Instruction + 'static {
        self.instructions.push(Box::new(instruction));
    }

    /// Returned the boxed instruction at `index`
    pub fn get_boxed_instruction(&self, index: usize) -> &Box<dyn Instruction> {
        self.instructions.get(index).unwrap()
    }

    /// Empty InstructionVec constructor
    pub fn new() -> InstructionVec {
        InstructionVec { instructions: Vec::new() }
    }
}

/// Pushes a list of instructions onto an InstructionVec
#[macro_export]
macro_rules! ivec_push_multiple {
    ( $vec:ident, $($act:expr), * ) => {
        $(
            $vec.push_instruction($act);
        )*
    };
}