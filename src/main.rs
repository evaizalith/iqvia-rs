use std::fs;
use std::fs::File;
use std::io::{stdout, Write};
use clap::Parser;
use polars::prelude::*;
use polars::{df};

const SEP : u8 = '|' as u8;

#[derive(Parser)]
struct Args {
    input_file: String,
    output_file: String ,
    n_workers: u8,
}

fn process(filename: String, out: &File) -> Result<(), ()> {
    let lazyfile = LazyCsvReader::new(filename).with_separator(SEP).finish();

    match lazyfile {
        Ok(_) => {},
        Err(_) => { return Err(()) },
    };


     

    Ok(())
}

fn main() {
    let args = Args::parse();

    let f = fs::read_dir(&args.input_file).unwrap();

    let mut i: f32 = 1.0;
    let n: usize = f.count();

    println!("Processing {n} files");

    let output = File::create_new(args.output_file + "out.csv").expect("Unable to create output file");

    if let Ok(files) = fs::read_dir(&args.input_file) { 
        for file in files {
            let p = file.unwrap().path();
            let ext = &p.clone().extension().unwrap().to_os_string();
            let path = p.clone().as_mut_os_str().to_str().unwrap().to_string();

            match ext.to_str() {
                Some(".csv") => process(path.to_string(), &output).expect("Unable to process file {}"),
                _ => (),
            }

            let x : i32 = ((i / n as f32) * 100.0) as i32;
            print!("\r{x}%");
            _ = stdout().flush();
            i += 1.0;

        }
    }

    println!("");
    println!("All done! <3");
}
