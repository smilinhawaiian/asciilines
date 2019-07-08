// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


use std::process::exit;
use std::fs;
use std::io::BufRead;
use std::path::Path;

///! Accept a single .tvg file command line argument and 
///! render the file as ASCII on standard output

/// Report proper usage and exit.
fn usage() -> ! {
    eprintln!("asciilines: usage: asciilines [--funct1|--funct2|--funct3|--funct4]");
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
    println!("\nThe filename that was read in is:\n{}", filename); // for testing
    let argdescs: &[(&str, asciilines::AsciiFn)] = &[
        ("--canvas", asciilines::draw_canvas),
        ("--funct2", asciilines::funct2),
        ("--funct3", asciilines::funct3),
        ("--funct4", asciilines::funct4),
    ];

    let argit = argdescs.iter();  //this worked...

    //for a_pair in argdescs.iter() { //this loop also valid
    //    println!("\na_pair loop running..\n");
    //}

    // create outfile to check
    let mut outfile_name = filename.to_string();
    let mut out_name = filename.to_string();
    println!("\noutfile_name: {:?}\n", outfile_name);
    let mut outfile_len = filename.len();
    println!("\noutfile_len: {:?}\n", outfile_len);
    outfile_len -= 3;
    println!("\noutfile_len: {:?}\n", outfile_len);
    //let &mut ext = &mut outfile_name[outfile_len..];
    //let mut tvg = false;
    let ext: &mut String = &mut outfile_name[outfile_len..].to_string();
    println!("\next: {:?}\n", ext);
    if ext == "tvg" {
        out_name = outfile_name[0..(outfile_len-1)].to_string();
        println!("\nout_name: {:?}\n", out_name);
    }
    //if &mut outfile_name[outfile_len..] == "tvg" {
        //println!("ext = TVG\n");
    //    tvg = true;
    //}
    //let ext = &outfile_name[outfile_len..];
    //if ext == "tvg" { // change outfile name
        //println!("ext = {:?}\n", ext);
    //}
    //outfile_name = outfile_name[

    // convert file into String for slicing
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("\nContents of file read in:\n{}\n", contents); // for testing

    // iterate over each slice of the file to pass into functions
    // to do stuff
    let mut count = 0;
    for line in contents.lines() {
        println!("line {}: {}", count, line);
        let nline: &[String] = &[line.to_string()];
        if count == 0 { // canvas parameters
            println!("canvas parameters here \n");
            asciilines::draw_canvas(nline);
            //let mut rendering = argit.find(|(a, _)| a == nline);
        } else {    // render command
            println!("rendering to be done here \n");
        }
        count+=1;
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

    println!("\nRendering the input: \n"); // for testing

    // Run the tvg, show the rendering if any.
    //if let Some(result) = rendering(&tvg_data) {
    //    println!("{}", result);
    //}
}


