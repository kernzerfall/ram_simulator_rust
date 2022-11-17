use register_machine;

fn main() {
    let mut insts: Vec<register_machine::program::Instruction> = Vec::new();
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::CLoad,
        1,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Store,
        3,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Load,
        2,
    ));
    insts.push(register_machine::program::Instruction::new_triple(
        register_machine::program::InstructionType::CondJmp,
        0,
        register_machine::program::Comparison::Eq,
        10,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::CSub,
        1,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Store,
        2,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Load,
        3,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Mult,
        1,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Store,
        3,
    ));
    insts.push(register_machine::program::Instruction::new_single(
        register_machine::program::InstructionType::Jmp,
        2,
    ));
    insts.push(register_machine::program::Instruction::end());

    let mut machine = register_machine::RegisterMachine::new(insts);

    machine.push(vec![2,3]);
    machine.run();
}