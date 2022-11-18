#[derive(Debug)]
#[allow(dead_code)]

/// Partial order comparisons
pub enum Comparison {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Comparison {
    // Compares `a` und `b`
    pub fn compare<T: PartialOrd>(&self, a: T, b: T) -> bool {
        match self {
            Comparison::Eq => a == b,
            Comparison::Lt => a < b,
            Comparison::Le => a <= b,
            Comparison::Gt => a > b,
            Comparison::Ge => a >= b,
        }
    }

    pub fn str_to_comp(str: &str) -> Comparison {
        match str.trim() {
            "="  => Comparison::Eq,
            "<"  => Comparison::Lt,
            "<=" => Comparison::Le,
            ">"  => Comparison::Gt,
            ">=" => Comparison::Ge,
            _ => Comparison::Eq // Should never occur
        }
    }
}
