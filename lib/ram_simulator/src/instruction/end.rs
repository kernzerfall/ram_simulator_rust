use super::Instruction;
use super::StringRepr;
use super::End;

use crate::state::State;

impl Instruction for End {
    fn exec(&self, state: &mut State){
        state.stop();
    }
}

impl StringRepr for End {
    fn command_name(&self) -> String {
        "END".to_string()
    }

    fn to_string(&self) -> String {
        self.command_name()
    }
}

impl End {
    pub fn new() -> End {
        End{}
    }
}