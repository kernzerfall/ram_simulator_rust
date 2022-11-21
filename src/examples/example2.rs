use std::io::BufWriter;

use ram_simulator::*;
use ram_simulator::instruction::*;
use comparison::Comparison;

fn main() {
    let mut isl = InstructionVec::new();
    ivec_push_multiple!(isl,
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

    let bw = BufWriter::new(std::io::stdout());

    let mut ram = RegisterMachine::new(isl);
    ram.push_vec(vec![2,3]);
    ram.run(bw);
}