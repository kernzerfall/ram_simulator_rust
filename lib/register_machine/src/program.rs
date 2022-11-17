#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum InstructionType {
    Load,
    Store,

    Add,
    Sub,
    Mult,
    Div,

    IndLoad,
    IndStore,
    IndAdd,
    IndSub,
    IndMult,
    IndDiv,

    CLoad,
    CAdd,
    CSub,
    CMul,
    CDiv,

    Jmp,
    CondJmp,

    End,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Comparison {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Comparison {
    pub fn compare<T: PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Comparison::Eq => a == b,
            Comparison::Lt => a < b,
            Comparison::Le => a <= b,
            Comparison::Gt => a > b,
            Comparison::Ge => a >= b,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    instruction_type: InstructionType,
    operand: u128,

    // Comp and goto_address are only used for conditional jumps
    comp: Comparison,
    goto_address: usize,
}

impl Instruction {
    pub fn end() -> Instruction {
        Instruction {
            instruction_type: InstructionType::End,
            operand: 0,
            comp: Comparison::Eq,
            goto_address: 0,
        }
    }

    pub fn new_triple(instruction_type: InstructionType, operand: u128, comp: Comparison, goto_address: usize) -> Instruction {
        if instruction_type != InstructionType::CondJmp {
            panic!("Only triple instruction is be CondJmp");
        }

        Instruction {
            instruction_type,
            operand,
            comp,
            goto_address,
        }
    }

    pub fn new_single(instruction_type: InstructionType, operand: u128) -> Instruction {
        if instruction_type == InstructionType::CondJmp {
            panic!("CondJump is a triple instruction");
        }

        Instruction {
            instruction_type,
            operand,
            comp: Comparison::Eq,
            goto_address: 0,
        }
    }

    pub fn exec(&self, machine_state: &mut crate::state::State) {
        print!("Step {:2} -- ", machine_state.get_steps());
        print!("PC: {:2}, ", machine_state.get_pc());
        
        match &self.instruction_type {
            // Load operations
            InstructionType::Load => {
                let value = machine_state.get_reg(self.operand as usize);
                machine_state.set_acc(value);
            },
            InstructionType::CLoad => {
                machine_state.set_acc(self.operand);
            },
            InstructionType::IndLoad => {
                let address = machine_state.get_reg(self.operand as usize);
                let value = machine_state.get_reg(address as usize);
                machine_state.set_acc(value);
            },

            // Store operations
            InstructionType::Store => {
                let acc = machine_state.get_acc();
                machine_state.set_reg(self.operand as usize, acc);
            },
            InstructionType::IndStore => {
                let address = machine_state.get_reg(self.operand as usize) as usize;
                let acc = machine_state.get_acc();
                machine_state.set_reg(
                    address, 
                    acc,
                );
            },

            // Add operations
            InstructionType::Add => {
                let acc = machine_state.get_acc();
                let value = machine_state.get_reg(self.operand as usize);
                machine_state.set_acc(acc + value);
            },
            InstructionType::CAdd => {
                let acc = machine_state.get_acc();
                machine_state.set_acc(acc + self.operand);
            },
            InstructionType::IndAdd => {
                let acc = machine_state.get_acc();
                let address = machine_state.get_reg(self.operand as usize) as usize;
                let value = machine_state.get_reg(address);
                machine_state.set_acc(acc + value);
            },


            // Sub operations
            InstructionType::Sub => {
                let acc = machine_state.get_acc();
                let value = machine_state.get_reg(self.operand as usize);
                if acc < value {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc - value);
                }
            },
            InstructionType::CSub => {
                let acc = machine_state.get_acc();
                if acc < self.operand {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc - self.operand);
                }
            },
            InstructionType::IndSub => {
                let acc = machine_state.get_acc();
                let address = machine_state.get_reg(self.operand as usize) as usize;
                let value = machine_state.get_reg(address);
                if acc < value {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc - value);
                }
            },

            // Mult operations
            InstructionType::Mult => {
                let acc = machine_state.get_acc();
                let value = machine_state.get_reg(self.operand as usize);
                machine_state.set_acc(acc * value);
            },
            InstructionType::CMul => {
                let acc = machine_state.get_acc();
                machine_state.set_acc(acc * self.operand);
            },
            InstructionType::IndMult => {
                let acc = machine_state.get_acc();
                let address = machine_state.get_reg(self.operand as usize) as usize;
                let value = machine_state.get_reg(address);
                machine_state.set_acc(acc * value);
            },

            // Div operations
            InstructionType::Div => {
                let acc = machine_state.get_acc();
                let value = machine_state.get_reg(self.operand as usize);
                if value == 0 {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc / value);
                }
            },
            InstructionType::CDiv => {
                let acc = machine_state.get_acc();
                if self.operand == 0 {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc / self.operand);
                }
            },
            InstructionType::IndDiv => {
                let acc = machine_state.get_acc();
                let address = machine_state.get_reg(self.operand as usize) as usize;
                let value = machine_state.get_reg(address);
                if value == 0 {
                    machine_state.set_acc(0);
                } else {
                    machine_state.set_acc(acc / value);
                }
            },

            // Jmp operations
            InstructionType::Jmp => {
                machine_state.set_pc(self.operand as usize);
            },
            InstructionType::CondJmp => {
                let acc = machine_state.get_acc();

                if self.comp.compare(acc, self.operand) {
                    machine_state.set_pc(self.goto_address);
                } else {
                    machine_state.inc_pc();
                }
            },

            // END
            InstructionType::End => {
                machine_state.stop();
            },
        }
        
        machine_state.print_registers();
        println!("");

        if self.instruction_type != InstructionType::CondJmp && self.instruction_type != InstructionType::Jmp {
            machine_state.inc_pc();
        }

        machine_state.inc_steps();
    }
}
