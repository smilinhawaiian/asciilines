// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


use std::process::exit;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::error::Error;

///! Accept a single .tvg file command line argument and 
///! render the file as ASCII on standard output

/// Report proper usage and exit.
fn usage() -> ! {
    eprintln!("asciilines: usage: asciilines [--draw_canvas|--add_to_canvas|--get_dims]");
    exit(1);
}

/// Do the rendering.
fn main() {
    // Process the argument -- get tvg file input
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        usage();
    }
    let filename = &args[1];

    // print filename argument for verification //--for testing
    println!("\n\nThe filename read in was:\n  {}\n", filename);

    // create outfile to check
    let mut outfile_name = filename.to_string();
    let outfile_len = filename.len() - 3; 
    let ext: &mut String = &mut outfile_name[outfile_len..].to_string();
    if ext == "tvg" {
        outfile_name = outfile_name[0..(outfile_len-1)].to_string() + ".out";  
    }

    // create file path for new output file
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

    // print tvg file contents for verification //--for testing
    println!("\nContents of file: ");
    for line in contents.lines() {  println!("  {}", line) }
    println!("\n");

    // pass file contents to function to draw and return canvas
    let canvas: &mut String = &mut asciilines::add_to_canvas(&[contents.to_string()]).unwrap();

    // write canvas to file
    match outfile.write_all(&canvas.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successful write!\nContents of {}:", display),
    }; 

    // print canvas outfile results for verification //--for testing
    let str_canvas = &canvas[..];
    for cline in str_canvas.lines() {  println!("  {}", cline)  }
    println!("\n");

}


