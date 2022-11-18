use crate::RegisterMachine;

use super::Serializer;

impl Serializer {
    pub fn to_string(machine: RegisterMachine) -> String {
        let mut res = String::new();
        for inst in machine.program.instructions {
            res.push_str(inst.to_string().as_str());
            res.push('\n');
        }

        res
    }

    pub fn dump(machine: RegisterMachine) {
        for inst in machine.program.instructions {
            println!("{}", inst.to_string().as_str());
        }
    }
}