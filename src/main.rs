use ram_simulator::{RegisterMachine};
use ram_simulator::program::{Instruction, InstructionType, Comparison};

fn main() {
    let mut insts: Vec<Instruction> = Vec::new();
    insts.push(Instruction::new_single(
        InstructionType::CLoad,
        1,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Store,
        3,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Load,
        2,
    ));
    insts.push(Instruction::new_triple(
        InstructionType::CondJmp,
        0,
        Comparison::Eq,
        10,
    ));
    insts.push(Instruction::new_single(
        InstructionType::CSub,
        1,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Store,
        2,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Load,
        3,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Mult,
        1,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Store,
        3,
    ));
    insts.push(Instruction::new_single(
        InstructionType::Jmp,
        2,
    ));
    insts.push(Instruction::end());

    let mut machine = RegisterMachine::new(insts);

    machine.push(vec![2,3]);
    machine.run();
}