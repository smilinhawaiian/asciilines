//--------------------------------------------------------------------
// COPIED FROM STATS/SRC/MAIN.RS
// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.


// THIS IS A STUB TO START THE ASCII PROGRAM WITH

use std::process::exit;

///! CURRENTLY STUBBED FOR STRUCTURAL INFORMATION ONLY
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
    if args.len() != 2 {
        usage();
    }
    let target = &args[1];
    let argdescs: &[(&str, asciilines::AsciiFn)] = &[
        ("--funct1", asciilines::funct1),
        ("--funct2", asciilines::funct2),
        ("--funct3", asciilines::funct3),
        ("--funct4", asciilines::funct4),
    ];
    let rendering = argdescs
        .iter()
        .find(|(a, _)| a == target)
        .unwrap_or_else(|| usage())
        .1;

    // Read the input.
    use std::io::BufRead;
    let nums: Vec<f64> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| {
            let s = s.unwrap_or_else(|e| {
                eprintln!("error reading input: {}", e);
                exit(-1);
            });
            s.parse::<f64>().unwrap_or_else(|e| {
                eprintln!("error parsing number {}: {}", s, e);
                exit(-1);
            })
        })
        .collect();

    // Run the tvg, show the result if any.
    if let Some(result) = rendering(&nums) {
        println!("{}", result);
    }
}


