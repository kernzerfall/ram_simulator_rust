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
