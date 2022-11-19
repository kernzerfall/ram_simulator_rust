use instruction::InstructionVec;
use state::State;

pub mod state;
pub mod comparison;
pub mod instruction;
pub mod text;

/// The machine itself
pub struct RegisterMachine {
    machine_state: state::State,
    program: InstructionVec,
}

impl RegisterMachine {
    /// Creates a new RAM that has `isv` loaded as a program
    pub fn new(isv: InstructionVec) -> RegisterMachine {
        RegisterMachine {
            machine_state: state::State::initial(),
            program: isv,
        }
    }

    /// Resets the state of the RAM
    pub fn reset(&mut self) {
        self.machine_state.reset();
    }

    /// Loads a new InstructionVec `isv` onto the machine and resets it.
    pub fn load_program(&mut self, isv: InstructionVec) {
        self.reset();
        self.program = isv;
    }

    /// Pushes a vector onto the registers of the machine (useful to set initial state)
    pub fn push_vec(&mut self, vec: Vec<u128>) {
        for (i, &v) in vec.iter().enumerate() {
            self.machine_state.set_reg(i+1, v);
        }
    }

    /// Runs the machine (until it internally reaches `END`) and outputs register values on each step
    pub fn run(&mut self) {
        self.machine_state.start();

        // Output initial config in yellow
        print!("\x1b[33mInitial Configuration -- ");
        self.machine_state.print_registers();
        println!("\x1b[0m");

        // Let the machine run
        while self.machine_state.is_running() {
            let pc = self.machine_state.get_pc();

            print!("Step {:2} -- PC: {:2}, ", self.machine_state.get_steps(), pc + 1);
            
            self.program.exec_instruction(pc, &mut self.machine_state);
            self.machine_state.inc_steps();
            
            self.machine_state.print_registers();
            println!();
        }
    } 

    /// Runs the machine for a single step
    pub fn step(&mut self) -> Result<State, &str> {
        if !self.machine_state.is_running() && self.machine_state.get_steps() == 0 {
            self.machine_state.start();
        }

        if !self.machine_state.is_running() && self.machine_state.get_acc() != 0 {
            return Err("The machine has reached an END instruction")
        }

        let pc = self.machine_state.get_pc();
        self.program.exec_instruction(pc, &mut self.machine_state);
        self.machine_state.inc_steps();
            
        Ok(self.machine_state)
    }

    /// Returns whether the internal state of the machine has reached an END instruction
    pub fn has_not_ended(&self) -> bool {
        self.machine_state.is_running() || self.machine_state.get_steps() == 0 
    }
}