use hoc;
use std::{env, path::Path, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        hoc::run(false);
    } else if args[1] == "--find-renames-and-copies" {
        hoc::run(true);
    } else {
        let mut name = None;
        if let Some(file_name) = Path::new(&args[0]).file_name() {
            name = file_name.to_str()
        };
        eprintln!(
            "usage: {} [--find-renames-and-copies]",
            name.unwrap_or("hoc")
        );
        process::exit(1);
    }
}
