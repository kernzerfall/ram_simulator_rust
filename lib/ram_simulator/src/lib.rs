pub mod state;
pub mod program;

pub struct RegisterMachine {
    machine_state: state::State,
    program: Vec<program::Instruction>,
}

#[allow(dead_code)]
impl RegisterMachine {
    pub fn new(prog: Vec<program::Instruction>) -> RegisterMachine {
        RegisterMachine {
            machine_state: state::State::initial(),
            program: prog,
        }
    }

    pub fn load_program(&mut self, program: Vec<program::Instruction>) {
        self.program = program;
    }

    pub fn push(&mut self, args: Vec<u128>){
        for (i, &arg) in args.iter().enumerate() {
            self.machine_state.set_reg(i+1, arg)
        }
    }

    pub fn run(&mut self) {
        self.machine_state.start();
        print!("\x1b[33mInitial Configuration -- ");
        self.machine_state.print_registers();
        println!("\x1b[0m");
        while self.machine_state.is_running() {
            let instruction = &self.program[self.machine_state.get_pc() as usize];
            instruction.exec(&mut self.machine_state);
        }
    } 
}