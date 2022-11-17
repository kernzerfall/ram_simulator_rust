use super::Instruction;
use super::{Load, IndLoad, CLoad};

use crate::state::State;

impl Instruction for Load {
    fn exec(&self, state: &mut State) {
        let value = state.get_reg(self.operand);
        state.set_acc(value);
        state.inc_pc();
    }
}

impl Instruction for CLoad {
   fn exec(&self, state: &mut crate::state::State) {
       state.set_acc(self.operand);
       state.inc_pc();
   }
}

impl Instruction for IndLoad {
    fn exec(&self, state: &mut State) {
        let address = state.get_reg(self.operand) as usize;
        let value = state.get_reg(address);
        state.set_acc(value);
        state.inc_pc();
    }
}