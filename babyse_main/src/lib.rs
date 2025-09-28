pub trait BabySerialize {
    fn serialize(&self) -> String;
}

pub trait BabyDeserialize: Sized {
    fn deserialize(s: &str) -> Self;
}
