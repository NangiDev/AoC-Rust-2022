/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use core::fmt;
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
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

impl Score {
    fn update_score(&mut self, line: String) {
        let values: Vec<String> = line.split("    ").map(|f| f.to_string()).collect();
        for v in values {
            if v.contains(':') {
                let v = v.split(':').collect::<Vec<&str>>();
                if v[0].eq("l1") {
                    self.latest1 = v[1].to_string();
                } else if v[0].eq("l2") {
                    self.latest2 = v[1].to_string();
                } else if v[0].eq("b1") {
                    self.best1 = v[1].to_string();
                } else if v[0].eq("b2") {
                    self.best2 = v[1].to_string();
                }
            }
        }
    }

    pub fn write_self_to_file(&mut self, mut file: File) {
        let buf = BufReader::new(&file).lines();

        let mut line_found = false;
        for line in buf {
            match line {
                Ok(l) => {
                    if l[..2].eq(&self.day) {
                        line_found = true;
                        self.update_score(l);
                        dbg!(&self);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read benchmark file: {}", e);
                    process::exit(1);
                }
            }
        }

        if !line_found {
            let line = format!(
                "{}    l1:{}    b1{}    l2:{}    b2:{}\n",
                self.day, self.latest1, self.best1, self.latest2, self.best2
            );
            let _ = file.write_all(line.as_bytes());
        }
    }
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
    OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
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
            println!("Created empty bechmark file \"{}\"", &benchmark_path);
        }
        Err(e) => {
            eprintln!("Failed to create bechmark file: {}", e);
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

    let mut score = Score {
        day: day_padded,
        latest1: stdout[1].to_string(),
        best1: stdout[1].to_string(),
        latest2: stdout[3].to_string(),
        best2: stdout[3].to_string(),
    };

    let file = create_file(&benchmark_path).unwrap();
    score.write_self_to_file(file);

    //println!("{}", &score)
}
