use super::Instruction;
use super::{Div, CDiv, IndDiv};

use crate::state::State;

impl Instruction for Div {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let value = state.get_reg(self.operand);
        if value == 0 {
            state.set_acc(0);
        } else {
            state.set_acc(acc / value);
        }
        state.inc_pc();
    }
}

impl Instruction for CDiv {
    fn exec(&self, state: &mut State) {
        if self.operand == 0 {
            state.set_acc(0);
            return;
        }

        let acc = state.get_acc();
        state.set_acc(acc * self.operand);
        state.inc_pc();
    }
}

impl Instruction for IndDiv {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_reg(address);
        if value == 0 {
            state.set_acc(0);
        } else {
            state.set_acc(acc / value);
        }
        state.inc_pc();
    }   
}