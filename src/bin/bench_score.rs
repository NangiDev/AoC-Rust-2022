/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use std::process::{self, Command};

struct Args {
    day: u8,
    _year: Option<i16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        _year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(_e) => {
            eprintln!("Need to specify a day (as integer). example: `cargo bm 7`");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", args.day);
    // cargo solve --release

    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(day_padded)
        .arg("--release")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
}
