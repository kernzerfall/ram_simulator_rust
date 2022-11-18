use super::Deserializer;
macro_rules! gen_single_arg_instr {
    ($instr:ident, $argtype:ident, $tkarr:expr, $line:expr) => {
        $instr::new(
            $tkarr.next()
                .expect(format!("Line {}: {} needs an argument", $line+1, "$instr").as_str())
                .parse::<$argtype>()
                .unwrap()
        )
    };
}

impl Deserializer {
    pub fn parse_file(path: PathBuf) -> std::io::Result<RegisterMachine> {
        let file = File::open(path)?;
        let br = BufReader::new(file);

        let mut isv = InstructionVec::new();
        let mut initial_state = Vec::new();

        let mut init_cmd = false;
        let mut end_cmd = false;

        for (i, line) in br.lines().enumerate() {

            let uw = line.unwrap();
            let mut tokens = uw.split_whitespace();
            
            
            let instruction = tokens.next()
                .expect(format!("Expected an instruction on line {}", i+1).as_str())
                .trim();

            match instruction.trim().to_uppercase().as_str() {
                "INIT" => if i == 0 {
                    init_cmd = true;
                    for tkn in tokens {
                        initial_state.push(tkn.trim().parse::<u128>().expect("Init arguments must be numbers"))
                    }
                } else {
                    panic!("INIT called inside program")
                },

                "LOAD" => isv.push_instruction(
                    gen_single_arg_instr!(Load, usize, tokens, i)
                ),

                "INDLOAD" => isv.push_instruction(
                    gen_single_arg_instr!(IndLoad, usize, tokens, i)
                ),

                "CLOAD" => isv.push_instruction(
                    gen_single_arg_instr!(CLoad, u128, tokens, i)
                ),

                "STORE" => isv.push_instruction(
                    gen_single_arg_instr!(Store, usize, tokens, i)
                ),

                "INDSTORE" => isv.push_instruction(
                    gen_single_arg_instr!(IndStore, usize, tokens, i)
                ),

                "ADD" => isv.push_instruction(
                    gen_single_arg_instr!(Add, usize, tokens, i)
                ),

                "INDADD" => isv.push_instruction(
                    gen_single_arg_instr!(IndAdd, usize, tokens, i)
                ),

                "CADD" => isv.push_instruction(
                    gen_single_arg_instr!(CAdd, u128, tokens, i)
                ),

                "SUB" => isv.push_instruction(
                    gen_single_arg_instr!(Sub, usize, tokens, i)
                ),

                "INDSUB" => isv.push_instruction(
                    gen_single_arg_instr!(IndSub, usize, tokens, i)
                ),

                "CSUB" => isv.push_instruction(
                    gen_single_arg_instr!(CSub, u128, tokens, i)
                ),

                "MULT" => isv.push_instruction(
                    gen_single_arg_instr!(Mult, usize, tokens, i)
                ),

                "INDMULT" => isv.push_instruction(
                    gen_single_arg_instr!(IndMult, usize, tokens, i)
                ),

                "CMULT" => isv.push_instruction(
                    gen_single_arg_instr!(CMult, u128, tokens, i)
                ),

                "DIV" => isv.push_instruction(
                    gen_single_arg_instr!(Div, usize, tokens, i)
                ),

                "INDDIV" => isv.push_instruction(
                    gen_single_arg_instr!(IndDiv, usize, tokens, i)
                ),

                "CDIV" => isv.push_instruction(
                    gen_single_arg_instr!(CDiv, u128, tokens, i)
                ),

                "GOTO" | "JMP" => isv.push_instruction(
                    gen_single_arg_instr!(Jmp, usize, tokens, i)
                ),

                "END" => {
                    end_cmd = true;
                    isv.push_instruction(
                        End::new()
                    )
                },

                "IF" => {
                    let tokens = COND_JMP_PARSER.captures(&uw).expect(
                        format!("Line {}: IF statement invalid", i+1).as_str()
                    );
                    let comp = Comparison::str_to_comp(&tokens[1]);
                    let value = tokens[2].parse::<u128>().expect(
                        format!("Line {}: IF condition must have an integer value", i+1).as_str()
                    );
                    let addr = tokens[3].parse::<usize>().expect(
                        format!("Line {}: IF condition must have an integer goto address", i+1).as_str()
                    );

                    isv.push_instruction(
                        CondJmp::new(comp, value, addr)
                    )
                }

                _ => todo!("impl {}", instruction),
            };
        }

        if !init_cmd {
            panic!("The program must start with INIT, even if it's empty")
        }
        if !end_cmd {
            panic!("The program must have an END command somewhere")
        }

        let mut res = RegisterMachine::new(isv);
        res.push_vec(initial_state);
        Ok(res)
    }
