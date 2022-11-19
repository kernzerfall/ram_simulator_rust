mod deserializer;

pub trait Serializable {
    fn to_string(&self) -> String;
    fn dump(&self);
}

pub struct Deserializer {
}