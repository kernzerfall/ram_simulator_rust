use super::Instruction;
use super::{Store, IndStore};

use crate::state::State;

impl Instruction for Store {
    fn exec(&self, state: &mut State) {
        let value = state.get_acc();
        state.set_reg(self.operand, value);
        state.inc_pc();
    }
}

impl Instruction for IndStore {
    fn exec(&self, state: &mut State) {
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_acc();
        state.set_reg(address, value);
        state.inc_pc();
    }
}