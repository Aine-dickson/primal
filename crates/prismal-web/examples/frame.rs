//! Prints what the web player gives its renderer: the layout of a presentation and its frame
//! description at a presentation instant.
//!
//! `cargo run -p prismal-web --example frame -- EXAMPLE PRESENTATION [SECONDS]`
//! (EXAMPLE is a key of `examples::all`: `rp06`, `rp07`, `rp08` ...)

use prismal_web::{examples, Player};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: frame EXAMPLE PRESENTATION [SECONDS]");
        std::process::exit(2);
    }
    let Some(ex) = examples::all().into_iter().find(|e| e.key == args[0]) else {
        eprintln!("no example {}", args[0]);
        std::process::exit(2);
    };
    let mut p = Player::load(&ex.source).unwrap_or_else(|d| panic!("{d}"));
    let layout = p.open(&args[1], false).unwrap_or_else(|d| panic!("{d}"));
    let at: f64 = args.get(2).map(|s| s.parse().expect("seconds")).unwrap_or(0.0);
    println!("{}", serde_json::to_string_pretty(&layout).unwrap());
    println!("{}", serde_json::to_string_pretty(&p.frame(at, 0.1)).unwrap());
    println!("{}", serde_json::to_string_pretty(&p.observations()).unwrap());
}
