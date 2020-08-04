use std::env;

fn main() {
    let vec: Vec<String> = env::args().collect();
    let args = &vec[1..];

    // println!("{:?}", args);
    assert!(!args.is_empty(), "no arguments");
    assert!(
        args.iter().all(|s| s.ends_with(".html")),
        "invalid arguments"
    );
}
