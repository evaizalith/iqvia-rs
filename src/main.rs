#![feature(lazy_cell)]
use std::fs;
use std::io::{self, stdout, Write};
use clap::Parser;
use polars::prelude::*;
use polars::df;

#[derive(Parser)]
struct Args {
    input_file: String,
    output_file: String,
    n_workers: u8,
}

//fn process(filename: String) {
//
//}

fn main() {
    let args = Args::parse();

    let f = fs::read_dir(args.input_file).unwrap();
    let files = f.copied();

    let i: i32 = 0;
    let n: usize = f.count();
    let mut lock = stdout().lock();

    for file in files {
        //println!("Name: {}", file.unwrap().path().display())
        
        let x : i32 = i / n as i32;
        write!(lock, "\r").unwrap();
        write!(lock, "{x}%").unwrap();
        io::stdout().flush().unwrap();
    }
}
