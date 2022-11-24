use std::io::{BufReader};
use state::State;
use text::{deserializer, Serializable};

use wasm_bindgen::prelude::*;
use ram_simulator::*;

#[wasm_bindgen]
extern {
    fn ram_post_res(r: &str, t: &str);
    fn ram_update_gl_state(r: &str);
    fn get_gl_state() -> String;
}

#[no_mangle]
#[wasm_bindgen]
pub fn run_machine(program: &str, max_depth: usize) {
    let br = BufReader::new(program.as_bytes());
    let mut another_ram: RegisterMachine;
    let mut state: ram_simulator::state::State = State::initial();

    let from_state = &get_gl_state();
    
    if from_state.len() > 7 {
        ram_post_res(
            &format!("Continuing from state {}", from_state),
            "ramStateInfo"
        );
        match State::from_wasm_comm_str(from_state) {
            Ok(s) => {
                state = s
            },
            Err(u) => {
                ram_post_res(
                    &format!("State was neither empty nor valid, err: {}. Starting from initial.", u),
                    "ramStateInfo"
                );
            }
        }
    }
    match deserializer::parse_buf(br) {
        Ok(m) => {
                another_ram = m;
                if state.get_steps() > 0 {
                    another_ram.set_state(state);
                }

                for _ in 0..max_depth {
                    match &mut another_ram.step() {
                        Ok(res_st) => {
                            if !another_ram.has_not_ended() {
                                ram_post_res(&"Machine halted.", "ramGoodResult");
                                break;
                            }

                            ram_post_res(&res_st.to_string(), "");
                    },
                        Err(m_err) => {
                            ram_post_res(m_err, "ramBadResult");
                            ram_update_gl_state(&another_ram.get_state().to_wasm_comm_str());
                            return;
                        }
                    }
                }
                ram_update_gl_state(&another_ram.get_state().to_wasm_comm_str());
                if another_ram.has_not_ended() {
                    ram_post_res("The machine hasn't halted yet.", "ramStateInfo");
                }
        },
        Err(e) => {
            ram_post_res(&e, "ramBadResult");
        }
    }
}