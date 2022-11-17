#[derive(Debug)]
#[allow(dead_code)]
pub enum Comparison {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Comparison {
    pub fn compare<T: PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Comparison::Eq => a == b,
            Comparison::Lt => a < b,
            Comparison::Le => a <= b,
            Comparison::Gt => a > b,
            Comparison::Ge => a >= b,
        }
    }
}
