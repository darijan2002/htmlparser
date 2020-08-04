use std::env;
use std::io;
use std::fs;
use std::ops::Deref;
use std::str::FromStr;

fn main() {
    let vec: Vec<String> = env::args().collect();
    let args = &vec[1..];

    // println!("{:?}", args);
    assert!(!args.is_empty(), "no arguments");
    assert!(
        args.iter().all(|s| s.ends_with(".html")),
        "invalid arguments"
    );

    args.iter().for_each(|x| {
        let mut content = fs::read_to_string(x).expect(format!("error reading {:?}", x).as_str());
        content = content.lines().map(|line| line.trim().chars()).flatten().collect();
        println!("contents of {:?}", x);
        println!("{:?}", content);
        // let mut start;
        println!("{:?}", content.chars().enumerate().filter(|&c| c.1 == '<').collect::<Vec<(usize, char)>>());


    });
}
