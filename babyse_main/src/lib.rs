use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub trait BabySerialize {
    fn serialize(&self) -> String;
}

pub trait BabyDeserialize: Sized {
    fn deserialize(s: &str) -> Self;
    fn register_type()
    where
        Self: 'static,
    {
        let name = std::any::type_name::<Self>().rsplit("::").next().unwrap(); // zhenlu: get the pure name
        register(name, |s| Box::new(Self::deserialize(s)));
    }
}

type FactoryFn = fn(&str) -> Box<dyn std::any::Any>;

static REGISTRY: OnceLock<RwLock<HashMap<&'static str, FactoryFn>>> = OnceLock::new();

fn registry() -> &'static RwLock<HashMap<&'static str, FactoryFn>> {
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

pub fn register(name: &'static str, factory: FactoryFn) {
    registry().write().unwrap().insert(name, factory);
}

pub fn deserialize_any(serialized_str: &str) -> Option<Box<dyn std::any::Any>> {
    let brace_pos = serialized_str.find('{').unwrap();
    let before_brace = &serialized_str[..brace_pos].trim();
    let struct_type = before_brace.split_whitespace().last().unwrap();

    registry()
        .read()
        .unwrap()
        .get(struct_type)
        .map(|f| f(serialized_str))
}