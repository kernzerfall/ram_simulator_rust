use instruction::InstructionVec;

pub mod state;
pub mod comparison;
pub mod instruction;

pub struct RegisterMachine {
    machine_state: state::State,
    program: InstructionVec,
}

impl RegisterMachine {
    pub fn new(isv: InstructionVec) -> RegisterMachine {
        RegisterMachine {
            machine_state: state::State::initial(),
            program: isv,
        }
    }

    pub fn reset(&mut self) {
        self.machine_state.reset();
    }

    pub fn load_program(&mut self, isv: InstructionVec) {
        self.program = isv;
    }

    pub fn push_vec(&mut self, vec: Vec<u128>) {
        for (i, &v) in vec.iter().enumerate() {
            self.machine_state.set_reg(i+1, v);
        }
    }

    pub fn run(&mut self) {
        self.machine_state.start();
        print!("\x1b[33mInitial Configuration -- ");
        self.machine_state.print_registers();
        println!("\x1b[0m");

        while self.machine_state.is_running() {
            let pc = self.machine_state.get_pc();

            print!("Step {:2} -- PC: {:2}, ", self.machine_state.get_steps(), self.machine_state.get_pc());
            
            self.program.exec_instruction(pc, &mut self.machine_state);
            self.machine_state.inc_steps();
            
            self.machine_state.print_registers();
            println!();
        }
    } 
}