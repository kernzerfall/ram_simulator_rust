use ram_simulator::*;
use ram_simulator::instruction::*;
use comparison::Comparison;

fn main() {
    let mut isl = InstructionVec::new();
    ivec_push_multiple!(isl, 
        CLoad::new(1),
        Store::new(3),
        Load::new(2),
        CondJmp::new(Comparison::Eq, 0, 11),
        CSub::new(1),
        Store::new(2),
        Load::new(3),
        Mult::new(1),
        Store::new(3),
        Jmp::new(3),
        End::new()
    );

    let mut isl2 = InstructionVec::new();
    ivec_push_multiple!(isl2,
        CLoad::new(0),
        Store::new(2),
        CLoad::new(1),
        Store::new(3),
        Load::new(3),
        Sub::new(1),
        CondJmp::new(Comparison::Gt, 0, 15),
        Load::new(2),
        CAdd::new(1),
        Store::new(2),
        Load::new(3),
        CMult::new(2),
        Store::new(3),
        Jmp::new(5),
        Load::new(2),
        CSub::new(1),
        Store::new(1),
        End::new()
    );

    let mut ram = RegisterMachine::new(isl);
    ram.push_vec(vec![2,3]);
    ram.run();

    ram.load_program(isl2);
    ram.push_vec(vec![16385]);
    ram.run();
}