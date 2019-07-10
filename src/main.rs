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
    println!("\n  The filename that was read in is:\n  {}", filename); // for testing
    let argdescs: &[(&str, asciilines::AsciiFn)] = &[
        ("--draw_canvas", asciilines::draw_canvas),
        ("--add_to_canvas", asciilines::add_to_canvas),
        ("--get_dims", asciilines::get_dims),
        ("--funct4", asciilines::funct4),
    ];

    let argit = argdescs.iter();  //this worked...

    // create outfile to check
    // start with similar name
    let mut outfile_name = filename.to_string(); //println!("\noutfile_name: {:?}\n", outfile_name);
    let mut outfile_len = filename.len() - 3;  //println!("\noutfile_len: {:?}\n", outfile_len); 
    let ext: &mut String = &mut outfile_name[outfile_len..].to_string();
    if ext == "tvg" {
        outfile_name = outfile_name[0..(outfile_len-1)].to_string() + "_my.out";  // println!("\nfinal outfile name: {:?}\n", outfile_name);  
    }

    // create file path
    let path = Path::new(&outfile_name);
    let display = path.display();
    // open a file in write-only mode; returns io::Result<File>
    let mut outfile = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(outfile) => outfile,
    };

    // convert file into String for slicing
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("\nContents of file:\n{}\n", contents); // for testing

    // iterate over each slice of the file to pass into functions
    let mut canvas = String::new();
    let mut count = 0;
    let canvas: &mut String = &mut asciilines::add_to_canvas(&[contents.to_string()]).unwrap();
    //println!("\ncanvas= {:?}\n", canvas);
    //--println!("\nTesting backtrace... after add_to_canvas call\n");
    // now write to file!
    match outfile.write_all(&canvas.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successful write to {}\n", display),
        //Ok(_) => {}, // println!("successful write to {}\n", display),
    } 

    // println!("\n  PRINTING RETURNED BASE CANVAS:");
    println!("Printing {} contents: \n", display);
    let str_canvas = &canvas[..];
    for cline in str_canvas.lines() {
        println!("  {}", cline)
    }


    //println!("\nReading the input: \n"); // for testing
    //let tvg_data: Vec<String> = std::io::stdin()
    //    .lock()
    //    .lines()
    //    .map(|s| {
    //        let s = s.unwrap_or_else(|e| {
    //            eprintln!("error reading input: {}", e);
    //            exit(-1);
    //        });
    //        s.parse::<String>().unwrap_or_else(|e| {
    //            eprintln!("error parsing number {}: {}", s, e);
    //            exit(-1);
    //        })
    //    })
    //    .collect();
    
    // --------------------------------------------

    //let rendering = argdescs
    //    .iter()
    //    .find(|(a, _)| a == filename)
    //    .unwrap_or_else(|| usage())
    //    .1;

    //println!("\nReading the input: \n"); // for testing

    // Read the input.
    //use std::io::BufRead;
    //let tvg_data: Vec<String> = std::io::stdin()
    //    .lock()
    //    .lines()
    //    .map(|s| {
    //        let s = s.unwrap_or_else(|e| {
    //            eprintln!("error reading input: {}", e);
    //            exit(-1);
    //        });
    //        s.parse::<String>().unwrap_or_else(|e| {
    //            eprintln!("error parsing number {}: {}", s, e);
    //            exit(-1);
    //        })
    //    })
    //    .collect();

    //println!("\nRendering the input: \n"); // for testing

    // Run the tvg, show the rendering if any.
    //if let Some(result) = rendering(&tvg_data) {
    //    println!("{}", result);
    //}
}


