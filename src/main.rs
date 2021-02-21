use hoc;
use std::{env, path::Path, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    let find_renames_and_copies = if args.len() == 1 {
        false
    } else if args[1] == "--find-renames-and-copies" {
        true
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
    };

    match hoc::hoc(find_renames_and_copies) {
        Ok(count) => println!("{}", count),
        Err(_) => {
            eprintln!("an error occurred");
            process::exit(1);
        }
    }
}
