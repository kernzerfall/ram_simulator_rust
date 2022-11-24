use std::io::{BufReader};
use state::State;
use text::{deserializer, Serializable};

use wasm_bindgen::prelude::*;
use ram_simulator::*;

static mut RAM: RegisterMachine = RegisterMachine::new_empty();
static mut INIT_STATE: State = State::initial();

#[wasm_bindgen]
extern {
    fn ram_post_res(r: &str, t: &str);
}

#[no_mangle]
#[wasm_bindgen]
pub fn init_machine(program: &str) {
    let br = BufReader::new(program.as_bytes());

    match deserializer::parse_buf(br) {
        Ok(m) => unsafe {
                RAM = m;
                INIT_STATE = RAM.get_state().clone();
        },
        Err(e) => {
            ram_post_res(&e, "ramBadResult");
        }
    }
}

#[no_mangle]
#[wasm_bindgen]
pub fn reset_machine() {
    unsafe {
        RAM.set_state(INIT_STATE);
    }
}

#[no_mangle]
#[wasm_bindgen]
pub unsafe fn run_machine(max_depth: usize) {
    if !RAM.has_not_ended() {
        ram_post_res("Machine has already reached an END command", "ramBadResult");
        return;
    }

    if RAM.get_state().get_steps() > 0 { 
        ram_post_res(
            &format!("Continuing from state {}", RAM.get_state().to_wasm_comm_str()),
            "ramStateInfo"
        );
    }

    for _ in 0..max_depth {
        match &mut RAM.step() {
            Ok(res_st) => {
                if !RAM.has_not_ended() {
                    ram_post_res(&"Machine halted.", "ramGoodResult");
                    break;
                }

                ram_post_res(&res_st.to_string(), "");
        },
            Err(m_err) => {
                ram_post_res(m_err, "ramBadResult");
                return;
            }
        }
    }

    if RAM.has_not_ended() {
        ram_post_res("The machine hasn't halted yet.", "ramStateInfo");
    }
}