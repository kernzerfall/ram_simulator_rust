use super::Instruction;
use super::{Add, CAdd, IndAdd};

use crate::state::State;

impl Instruction for Add {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let value = state.get_reg(self.operand);
        state.set_acc(acc + value);
        state.inc_pc();
    }
}

impl Instruction for CAdd {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        state.set_acc(acc + self.operand);
        state.inc_pc();
    }
}

impl Instruction for IndAdd {
    fn exec(&self, state: &mut State) {
        let acc = state.get_acc();
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_reg(address);
        state.set_acc(acc + value);
        state.inc_pc();
    }   
}