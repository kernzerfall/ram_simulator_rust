use std::env::args;
use std::path::Path;
use std::process::exit;

use ram_simulator::*;

fn main() {
    let argv: Vec<String> = args().collect(); 

    println!("\x1b[34mRegister Machine Simulator CLI -- kernzerfall 2022\x1b[0m");

    if argv.len() != 2 {
        println!("\n\x1b[31mUsage\x1b[0m: rscli /path/to/ram-program.s\n");
        exit(1);
    }

    println!("Running \x1b[35m'{}'\x1b[0m", &argv[1]);

    let mut another_ram = text::Deserializer::parse_file(Path::new(&argv[1]).to_path_buf())
        .expect("File should contain a valid assembly program");
    another_ram.run();

    // let ser = Serializer::to_string(another_ram);
    // Serializer::dump(another_ram);
    // Serializer::dump(ram);
}