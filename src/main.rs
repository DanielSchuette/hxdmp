use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::process::exit;
use std::str::from_utf8;

const COLS: usize = 16;

fn main() {
    let mut buf: [u8; COLS] = [0; COLS];
    let mut byte_counter = 0;

    // get the file path
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <FILE>", args[0]);
        exit(0);
    }
    let path = args.get(1).unwrap();

    // check if file exists
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            let msg = get_err(err, "cannot open file", &path);
            println!("{}: {}.", args[0], msg);
            exit(1);
        }
    };

    // read `COLS' bytes at a time and print them like `hexdump -C'
    loop {
        let read = match file.read(&mut buf) {
            Ok(r) => r,
            Err(err) => {
                let msg = get_err(err, "cannot read file", &path);
                println!("{}: {}.", args[0], msg);
                exit(1);
            }
        };
        print!("{:08x}", byte_counter);
        if !(read > 0) {
            break;
        } else {
            print!("  ");
        }
        for (idx, byte) in buf.iter().enumerate() {
            if (idx + 1) > read {
                // fill remaining cols to align ascii output
                for i in 0..(COLS - read) {
                    if i % (COLS / 2) == 0 {
                        print!("    ");
                    } else {
                        print!("   ");
                    }
                }
                break;
            }
            print!("{:02x}", byte);
            byte_counter += 1;
            if byte_counter % (COLS / 2) == 0 {
                print!("  ");
            } else {
                print!(" ");
            }
        }
        print!("|");
        for (idx, c) in from_utf8(&buf).expect("Invalid string").chars().enumerate()
        {
            if (idx + 1) > read {
                break;
            }
            if c == '\n' || c == '\t' || c == '\r' {
                print!(".");
            } else {
                print!("{}", c);
            }
        }
        print!("|\n");
    }
    print!("\n");
}

fn get_err(err: io::Error, msg: &str, path: &String) -> String {
    let err = format!("error: {} {}, {}", msg, path, err);
    let (err, _) = err.split_at(err.find('(').unwrap() - 1);
    err.to_string().to_lowercase()
}
