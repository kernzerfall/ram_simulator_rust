use super::Instruction;
use super::{Sub, CSub, IndSub};

use crate::state::State;

impl Instruction for Sub {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let value = state.get_reg(self.operand);
        if acc < value {
            state.set_acc(0);
        } else {
            state.set_acc(acc - value);
        }
        state.inc_pc();
    }
}

impl Instruction for CSub {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        if acc < self.operand {
            state.set_acc(0);
        } else {
            state.set_acc(acc - self.operand);
        }
        state.inc_pc();
    }
}

impl Instruction for IndSub {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_reg(address);
        if acc < value {
            state.set_acc(0);
        } else {
            state.set_acc(acc - value);
        }
        state.inc_pc();
    }
}
