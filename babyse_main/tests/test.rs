use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use babyse_derive::{BabyDeserialize, BabySerialize};
use babyse_main::*;

#[derive(Debug, BabySerialize, BabyDeserialize)]
pub struct Foo {
    x: i32,
    y: String,
}

#[derive(Debug, BabySerialize, BabyDeserialize)]
pub struct Bar {
    a: String,
    b: i32,
}

#[test]
#[ignore]
fn foo_se() {
    Foo::register_type();
    let foo = Foo {
        x: 42,
        y: "bonjour".into(),
    };
    let s = foo.serialize();
    // println!("{s}");
    let b = deserialize_any(&s).unwrap();
    let f: Foo = *b.downcast::<Foo>().expect("wrong type");
    // println!("f.x: {}, f.y: {}", f.x, f.y);
}

#[test]
#[ignore]
fn bar_se() {
    Bar::register_type();
    let bar = Bar {
        a: "hello".to_string(),
        b: 1,
    };
    let s = bar.serialize();
    // println!("{s}");
    let b = deserialize_any(&s).unwrap(); // returns Option<Box<dyn Any>>
    let bar: Bar = *b.downcast::<Bar>().expect("wrong type");
    // println!("b.a: {}, b.b: {}", bar.a, bar.b);
}

#[test]
fn foo_bar_se() {
    Foo::register_type();
    Bar::register_type();

    let foo = Foo {
        x: 42,
        y: "bonjour".into(),
    };
    let bar = Bar {
        a: "hello".to_string(),
        b: 1,
    };
    let s_foo: String = foo.serialize();
    let s_bar: String = bar.serialize();
    println!("foo serialized: {s_foo}");
    println!("bar serialized: {s_bar}");

    let path = "output.txt";
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    let _ = file.write_all(format!("{}\n", s_foo).as_bytes()).unwrap();
    let _ = file.write_all(format!("{}\n", s_bar).as_bytes()).unwrap();
    println!("serialized data writen to {path}");

    let file = OpenOptions::new().read(true).open(path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::<String>::new();
    for line in BufRead::lines(reader) {
        let line = line.unwrap();
        lines.push(line);
    }

    if let Some(s) = lines.iter().nth(0) {
        let b = deserialize_any(s).unwrap();
        let foo_new: Foo = *b.downcast::<Foo>().expect("wrong type");
        println!("foo deserialized from {path}: f.x = {}, f.y = {}", foo_new.x, foo_new.y);
    }
    if let Some(s) = lines.iter().nth(1) {
        let b = deserialize_any(s).unwrap();
        let bar_new: Bar = *b.downcast::<Bar>().expect("wrong type");
        println!("bar deserialized from {path}: b.a = {}, b.b = {}", bar_new.a, bar_new.b);
    }

    let _ = fs::remove_file(path);
}
