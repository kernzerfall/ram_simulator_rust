use std::io::BufWriter;

/**!
 * An example of a hardcoded program.
 * This program computes `ceil(ld(c(1))`
 */

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

    let bw = BufWriter::new(std::io::stdout());

    let mut ram = RegisterMachine::new(isl);
    ram.push_vec(vec![2,3]);
    ram.run(bw);
}