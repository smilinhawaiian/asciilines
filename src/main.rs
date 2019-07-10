// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


use std::process::exit;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::BufRead;
use std::path::Path;
use std::error::Error;

///! Accept a single .tvg file command line argument and 
///! render the file as ASCII on standard output

/// Report proper usage and exit.
fn usage() -> ! {
    eprintln!("asciilines: usage: asciilines [--draw_canvas|--add_to_canvas|--get_dims|--funct4]");
    exit(1);
}

/// Do the rendering.
fn main() {
    // Process the argument.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        usage();
    }
    //println!("\nPrinting args: \n{:?}\n", args); // for testing
    let filename = &args[1];
    println!("\n\nThe filename read in was:\n  {}\n", filename); // for testing
    //let argdescs: &[(&str, asciilines::AsciiFn)] = &[
    //    ("--draw_canvas", asciilines::draw_canvas),
    //    ("--add_to_canvas", asciilines::add_to_canvas),
    //    ("--get_dims", asciilines::get_dims),
    //    ("--funct4", asciilines::funct4),
    //];

    // create outfile to check
    let mut outfile_name = filename.to_string();
    let mut outfile_len = filename.len() - 3; 
    let ext: &mut String = &mut outfile_name[outfile_len..].to_string();
    if ext == "tvg" {
        outfile_name = outfile_name[0..(outfile_len-1)].to_string() + ".out";  
    }

    // create file path
    let path = Path::new(&outfile_name);
    let display = path.display();
    // open a file in write-only mode; returns io::Result<File>
    let mut outfile = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(outfile) => outfile,
    };
    // convert file into String for slicing
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    //println!("\nContents of file:\n{}\n", contents); // for testing
    println!("\nContents of file: "); // for testing
    for line in contents.lines() {  println!("  {}", line) }
    println!("\n");

    // iterate over each slice of the file to pass into functions
    let mut canvas = String::new();
    let mut count = 0;
    let canvas: &mut String = &mut asciilines::add_to_canvas(&[contents.to_string()]).unwrap();
    //--println!("\ncanvas= {:?}\n", canvas);
    //--println!("\nTesting backtrace... after add_to_canvas call\n");
    // now write to file!
    match outfile.write_all(&canvas.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successful write!\nContents of {}:", display),
        //Ok(_) => {},
    } 

    // println!("\n  PRINTING RETURNED BASE CANVAS:");
    //println!("Printing {} contents: \n", display);
    let str_canvas = &canvas[..];
    for cline in str_canvas.lines() {
        println!("  {}", cline)
    }
    println!("\n");

}


