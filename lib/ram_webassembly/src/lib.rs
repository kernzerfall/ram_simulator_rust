use std::io::{BufReader, BufWriter};

use wasm_bindgen::prelude::*;
use ram_simulator::*;

#[wasm_bindgen]
extern {
    fn ram_post_res(r: &str);
}

#[no_mangle]
#[wasm_bindgen]
pub fn run_machine(program: &str) {
    let br = BufReader::new(program.as_bytes());

    let mut another_ram = text::deserializer::parse_buf(br)
        .expect("File should contain a valid assembly program");

    let bw = BufWriter::new(WASMCommunicator{});
    another_ram.run(bw);
}

struct WASMCommunicator {

}

impl std::io::Write for WASMCommunicator {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        ram_post_res(core::str::from_utf8(buf).expect("A valid string"));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}