/**!
 * Example of a program being deserialized from a file
 */

use std::path::Path;

use ram_simulator::*;
use text::Deserializer;

fn main() {
    let mut another_ram = Deserializer::parse_file(Path::new("src/examples/example3.s").to_path_buf())
        .expect("File should contain a valid assembly program");
    another_ram.run();

    // let ser = Serializer::to_string(another_ram);
    // Serializer::dump(another_ram);
    // Serializer::dump(ram);
}