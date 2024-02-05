use std::{env, fs, path::Path, process::Command};

use brainfuck_sys::{complie, interpret};

fn main() {
    let args: Vec<_> = env::args().collect();
    
    for arg in args {
        let mut segments = arg.split("=");

        let Some(field) = segments.next() else { continue; };

        let Some(value) = segments.next() else { continue; };

        if field == "file" {
            let code = std::fs::read_to_string(value).unwrap();
            let source = complie(&code, 3000, "i16");
            if !Path::new("./target/build/").exists() { fs::create_dir_all("./target/build/").unwrap(); }
            fs::write("./target/build/brainfuck.rs", source).unwrap();
            println!("Successfully compiled {} to ./target/build/brainfuck.rs", value);
            Command::new("rustc").args(["./target/build/brainfuck.rs"]).output().unwrap();
            println!("Successfully ran linked ./target/build/brainfuck.rs");
        } else if field == "run" {
            interpret(value);
        }

    }

}
