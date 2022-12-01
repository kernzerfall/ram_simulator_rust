use core::panic;
use std::env::args;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::process::exit;

use text::Serializable;
use ram_simulator::*;

fn main() {
    let argv: Vec<String> = args().collect(); 

    println!("\x1b[34mRegister Machine Simulator CLI -- kernzerfall 2022\x1b[0m");

    if argv.len() != 3 {
        println!("\n\x1b[31mUsage\x1b[0m: rscli [cs] /path/to/ram-program.s\n");
        exit(1);
    }

    println!("Running \x1b[35m'{}'\x1b[0m", &argv[2]);

    let mut another_ram = text::deserializer::parse_file(Path::new(&argv[2]).to_path_buf())
        .expect("File should contain a valid assembly program");

    match argv[1].chars().nth(0).expect("A valid argument") {
        'c' => {
            let bw = BufWriter::new(std::io::stdout());
            let res = another_ram.run(bw);
            if res.is_some() {
                panic!("{}", res.unwrap());
            }
        },
        's' => {
            while another_ram.has_not_ended() {
                // Wait for enter
                std::io::stdin().read_line(&mut String::new()).unwrap();
                // Run step
                match another_ram.step() {
                    Ok(s) => { s.dump() },
                    Err(u) => { println!("{}", u) }
                }
                std::io::stdout().flush().unwrap();
            }
        },
        _ => panic!("Unknown argument {}", &argv[1])
    }

    // let ser = Serializer::to_string(another_ram);
    // Serializer::dump(another_ram);
    // Serializer::dump(ram);
}