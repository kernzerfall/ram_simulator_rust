use super::Instruction;
use super::{Mult, CMult, IndMult};

use crate::state::State;

impl Instruction for Mult {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let value = state.get_reg(self.operand);
        state.set_acc(acc * value);
        state.inc_pc();
    }
}

impl Instruction for CMult {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        state.set_acc(acc * self.operand);
        state.inc_pc();
    }
}

impl Instruction for IndMult {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_reg(address);
        state.set_acc(acc * value);
        state.inc_pc();
    }   
}