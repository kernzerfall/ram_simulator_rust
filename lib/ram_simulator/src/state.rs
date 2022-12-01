use std::io::Write;

use crate::text::Serializable;

/// Keeps track of the RAM's current state.
#[derive(Debug, Clone, Copy)]
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
impl State {
    /// Creates an empty (new) state.
    pub const fn initial() -> State {
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
    pub fn print_registers<T: Write>(&self, output: &mut T) -> Result<(), String> {
        for i in 0..self.highest_register+1 {
            match output.write(
                format!("r{:}: {}", i, self.registers[i]).as_bytes()
            ) {
                Ok(_) => {},
                Err(u) => return Err(
                    format!("Could not write to buffer: {}", u.to_string())
                )
            };

            if i != self.highest_register {
                match output.write(b", ") {
                    Ok(_) => {},
                    Err(u) => return Err(
                        format!("Could not write to buffer: {}", u.to_string())
                    )
                };
            }
        }

        Ok(())
    }

    /// Resets the machine's state to the initial one
    pub fn reset(&mut self) {
        self.pc = 0;
        self.registers = [0; 1024];
        self.highest_register = 0;
        self.running = false;
        self.steps = 0;
    }

    /// Overwrites the machine's state
    pub fn overwrite(&mut self, new: &State) {
        self.reset();


        self.pc = new.pc;
        self.highest_register = new.highest_register;
        self.running = new.running;
        self.steps = new.steps;
        
        for i in 0..self.highest_register+1 {
            self.registers[i] = new.registers[i];
        }
    }
}

/// Implement Serialization funcs for state objects
impl Serializable for State {
    fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(
            // The state we get has already has its step/pc incremented internally
            format!("Step {:2} -- PC: {:2}, ", self.steps-1, self.pc).as_str()
        );

        for rn in 0..self.highest_register+1 {
            res.push_str(
                format!("r{}: {}", rn, self.registers[rn]).as_str()
            );

            if rn != self.highest_register {
                res.push_str(", ");
            }
        }

        res
    }

    fn dump(&self) {
        print!("Step {:2} -- PC: {:2}, ", self.steps-1, self.pc);

        for rn in 0..self.highest_register+1 {
            print!("r{}: {}, ", rn, self.registers[rn])
        }

        print!("\x08\x08\x20\x20");
    }
}

impl State {
    const SEPARATOR: char = '<';
    pub fn to_wasm_comm_str(&self) -> String {
        let mut res = String::new();
        res.push(match self.running {
            true => 'r',
            false => 's'
        });
        res.push(Self::SEPARATOR);

        res.push_str(&self.steps.to_string());
        res.push(Self::SEPARATOR);

        res.push_str(&self.pc.to_string());
        res.push(Self::SEPARATOR);

        res.push_str(&self.highest_register.to_string());
        res.push(Self::SEPARATOR);

        for i in 0..self.highest_register+1 {
            res.push_str(&self.registers[i].to_string());
            
            if i < self.highest_register {
                res.push(Self::SEPARATOR);
            }
        }

        res
    }

    pub fn from_wasm_comm_str(istr:  &str) -> Result<State, String> {
        let mut res = State::initial();

        let mut tokens = istr.split(Self::SEPARATOR);

        let mut next_token = tokens.next();

        if next_token.is_none() {
            return Err("Expected a running/stopped at pos 0".to_string());
        } else {
            match next_token.unwrap() {
                "r" => res.running = true,
                "s" => res.running = false,
                _ => return Err(
                    format!("r? {}", next_token.unwrap())
                )
            }
        }

        next_token = tokens.next();
        if next_token.is_none() {
            return Err("Expected steps at pos 1".to_string());
        } else {
            match next_token.unwrap().parse::<usize>() {
                Ok(steps) => res.steps = steps,
                Err(pie) => {
                    return Err(
                        format!("step number {} -- {}", next_token.unwrap(), pie.to_string())
                    );
                }
            }
        }
        
        next_token = tokens.next();
        if next_token.is_none() {
            return Err("Expected a PC at pos 2".to_string());
        } else {
            match next_token.unwrap().parse::<usize>() {
                Ok(pc) => res.set_pc(pc),
                Err(pie) => {
                    return Err(
                        format!("pc {}: {}", next_token.unwrap(), pie.to_string())
                    );
                }
            }
        }


        next_token = tokens.next();
        if next_token.is_none() {
            return Err("Expected Highest Register at pos 3".to_string());
        } else {
            match next_token.unwrap().parse::<usize>() {
                Ok(highest) => res.highest_register = highest,
                Err(pie) => {
                    return Err(
                        format!("highest register {} - {}", next_token.unwrap(), pie.to_string())
                    );
                }
            }
        }

        for i in 0..res.highest_register+1 {
            next_token = tokens.next();
            if next_token.is_none() {
                return Err("Expected more register values".to_string());
            } else {
                match next_token.unwrap().parse::<u128>() {
                    Ok(rv) => res.registers[i] = rv,
                    Err(pie) => {
                        return Err(
                            format!("register value {}: {} -- {}", i, next_token.unwrap(), pie.to_string())
                        );
                    }
                }
            }
        }

        Ok(res)
    }
}