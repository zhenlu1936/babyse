use babyse_main::*;

#[derive(Debug, BabySerialize, BabyDeserialize)]
struct Foo {
    x: i32,
    y: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn babyse() {
        let foo = Foo {
            x: 42,
            y: "hi".into(),
        };
        let s = foo.serialize();
        println!("s");
        let f = Foo::deserialize(&s);
        println!("{} {}", f.x, f.y);
    }
}
