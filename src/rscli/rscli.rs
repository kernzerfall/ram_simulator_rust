use std::env::args;
use std::path::Path;
use std::process::exit;

use ram_simulator::*;

fn main() {
    let argv: Vec<String> = args().collect(); 

    println!("\x1b[34mRegister Machine Simulator CLI -- kernzerfall 2022\x1b[0m");

    if argv.len() != 3 {
        println!("\n\x1b[31mUsage\x1b[0m: rscli [cs] /path/to/ram-program.s\n");
        exit(1);
    }

    println!("Running \x1b[35m'{}'\x1b[0m", &argv[2]);

    let mut another_ram = text::Deserializer::parse_file(Path::new(&argv[2]).to_path_buf())
        .expect("File should contain a valid assembly program");

    match argv[1].chars().nth(0).expect("A valid argument") {
        'c' => another_ram.run(),
        _ => panic!("Unknown argument {}", &argv[1])
    }

    // let ser = Serializer::to_string(another_ram);
    // Serializer::dump(another_ram);
    // Serializer::dump(ram);
}