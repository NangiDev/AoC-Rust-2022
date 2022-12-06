/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use core::fmt;
use std::{
    fs::{File, OpenOptions},
    process::{self, Command},
};

struct Args {
    day: u8,
    _year: Option<i16>,
}

#[derive(Debug)]
struct Score {
    day: String,
    latest1: String,
    best1: String,
    latest2: String,
    best2: String,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Day: {}\n\tPart 1:\n\t\tLatest: {}\n\t\tBest  : {}\n\tPart 2:\n\t\tLatest: {}\n\t\tBest  : {}",
            self.day, self.latest1, self.best1, self.latest2, self.best2
        )
    }
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        _year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new().write(true).create(true).open(path)
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(_e) => {
            eprintln!("Need to specify a day (as integer). example: `cargo bm 7`");
            process::exit(1);
        }
    };

    let benchmark_path = "src/benchmarks/table.txt".to_string();
    match create_file(&benchmark_path) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &benchmark_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {}", e);
            process::exit(1);
        }
    }

    // cargo solve --release
    let day_padded = format!("{:02}", args.day);
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(&day_padded)
        .arg("--release")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    let stdout: Vec<String> = stdout
        .split('🎄')
        .filter(|f| !f.is_empty())
        .map(|f| {
            if f.contains(':') {
                let n = f.split(':').last().unwrap();
                &n[..=n.len() - 7] //Clean garbage in end of string
            } else {
                f
            }
        })
        .map(|f| f.replace(['(', ')', ' ', '\n'], ""))
        .collect();

    let score = Score {
        day: day_padded,
        latest1: stdout[1].to_string(),
        best1: stdout[1].to_string(),
        latest2: stdout[3].to_string(),
        best2: stdout[3].to_string(),
    };

    println!("{}", &score)
}
