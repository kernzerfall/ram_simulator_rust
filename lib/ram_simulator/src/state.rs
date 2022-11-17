/// Keeps track of the RAM's current state.
#[derive(Debug)]
pub struct State {
    /// `Program Counter`: the offset of the next instruction to be executed
    pc: usize,
    /// Registers (128-bit * 1024 = 16KiB)
    registers: [u128; 1024],
    /// Highest register used (read from or written to)
    highest_register: usize,
    /// Machine running? (Is END reached?)
    running: bool,
    /// Steps up to now
    steps: usize,
}

/// Methods for the State struct.
#[allow(dead_code)]
impl State {
    /// Creates an empty (new) state.
    pub fn initial() -> State {
        State {
            pc: 0,
            registers: [0; 1024],
            highest_register: 0,
            running: false,
            steps: 0,
        }
    }

    /// Returns the contents of the accumulator (r0).
    pub fn get_acc(&self) -> u128 {
        self.registers[0]
    }

    /// Sets the contents of the accumulator (r0).
    pub fn set_acc(&mut self, value: u128) {
        self.registers[0] = value;
    }

    /// Returns the contents of the given register.
    pub fn get_reg(&mut self, index: usize) -> u128 {
        if self.highest_register < index {
            self.highest_register = index;
        }
        self.registers[index]
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, index: usize, value: u128) {
        if self.highest_register < index {
            self.highest_register = index;
        }
        self.registers[index] = value;
    }

    /// Sets the program counter to the given value.
    pub fn set_pc(&mut self, value: usize) {
        self.pc = value;
    }

    /// Returns the program counter.
    pub fn get_pc(&self) -> usize {
        self.pc
    }

    /// Increments the program counter by 1.
    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    /// Returns the highest register used.
    pub fn get_highest_register(&self) -> usize {
        self.highest_register
    }

    /// Set running to true when the machine starts
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Set running to false when the END instruction is reached
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the machine is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Increment the number of steps
    pub fn inc_steps(&mut self) {
        self.steps += 1;
    }

    /// Get the number of steps
    pub fn get_steps(&self) -> usize {
        self.steps
    }

    /// Prints registers up to the highest register used
    pub fn print_registers(&self) {
        for i in 0..self.highest_register+1 {
            print!("r{:}: {}, ", i, self.registers[i]);
        }
        print!("\x08\x08\x20");
    }

    /// Resets the machine's state to the initial one
    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 1024];
        self.highest_register = 0;
        self.running = false;
        self.steps = 0;
    }
}