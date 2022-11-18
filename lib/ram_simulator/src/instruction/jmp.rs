use super::Instruction;
use super::StringRepr;
use super::{Jmp, CondJmp};

use crate::comparison::Comparison;
use crate::state::State;

impl Instruction for Jmp {
    fn exec(&self, state: &mut State) {
        state.set_pc(self.operand - 1);
    }
}

impl Instruction for CondJmp {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        if self.comparison.compare(acc, self.value) {
            state.set_pc(self.jmp_address - 1);
        } else {
            state.inc_pc();
        }
    }

    
}

impl CondJmp {
    pub fn new(comp: Comparison, value: u128, addr: usize) -> CondJmp {
        CondJmp { comparison: comp, value: value, jmp_address: addr }
    }
}

impl StringRepr for CondJmp {
    fn command_name(&self) -> String { 
        "COND_JMP".to_string()
    }

    fn to_string(&self) -> String {
        let comp: String = match self.comparison {
            Comparison::Eq => "=",
            Comparison::Ge => ">=",
            Comparison::Gt => ">",
            Comparison::Le => "<=",
            Comparison::Lt => "<",
        }.to_string();

        format!("IF c(0){}{} THEN GOTO {}", comp, self.value, self.jmp_address)
    }
}