// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::fmt::Write;
//use std::fmt::Debug;

///! Functions to help render a .tvg file 
///! on standard output

/// Type of asciilines function. If the function
/// is ill-defined, `None` will be returned.
pub type AsciiFn = fn(&[String]) -> Option<String>;

/// Struct to hold command data for drawing in canvas.
/// sym: character to be printed
/// rs: row starting coordinate
/// cs: column starting coordinate
/// dir: line direction(h=horizontal;v=vertical)
/// to print sym in the canvas
/// dis: distance(number of rows or columns) to print
#[derive(Debug, Clone)]
pub struct Command {
        pub sym: String,
        pub rs:  i32,
        pub cs:  i32,
        pub dir: String,
        pub dis: i32,
}

/// Draw base canvas from tvg file input.
/// Canvas size input from the first line of the tvg file.
/// This line contains the canvas dimensions: 
/// rows columns
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some(".....\n.....\n.....\n".to_string()), draw_canvas(&[String::from("3 5")]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("".to_string()), draw_canvas(&[String::from("0 0")]));
/// ```
pub fn draw_canvas(dims: &[String]) -> Option<String> {
    // declare variables for storing canvas and dimensions
    let mut xydims: Vec<u32> = Vec::new();
    let mut canvas = String::new();
    
    // get dimensions for canvas
    if dims.len() != 0 {
        for line in get_dims(&dims).unwrap().lines() {
            xydims.push(line.parse().unwrap());
        }
        // build base canvas
        for _x in 0..xydims[0] {  // build rows
            let mut curr_str = "".to_string();
            for _y in 0..xydims[1] {  // build columns
                curr_str = curr_str + ".";
            }
            writeln!(&mut canvas, "{}", curr_str).unwrap();
        }
    }
    Some(canvas)
}

//#[test]
//fn test_draw_canvas_01() {
//    assert_eq!(Some(""), draw_canvas(&[""]));
//}


/// Builds canvas from tvg file rendering commands  
/// 1. Gets canvas dimensions from first line  
/// 2. Parses rendering commands into a struct for each successive line  
/// 3. Adds characters to canvas as given by each command  
/// num_rows num_cols  
/// char rowstart colstart direction length  
///
/// # Examples:  
///
/// ```
/// # use asciilines::*;  
/// assert_eq!(Some(".**.\n....\n....\n....\n".to_string()), add_to_canvas(&[String::from("4 4\n* 0 1 h 2")]));  
/// ```
/// ```
/// # use asciilines::*;  
/// assert_eq!(Some("#.....\n.#...\n.#...\n".to_string()), add_to_canvas(&[String::from("3 5\n# 1 1 v 4")]));  
/// ```
pub fn add_to_canvas(args: &[String]) -> Option<String> {
    ////println!("\n\tfunction::add_to_canvas called... ");
    //println!("Printing passed in args: \n {:?} \n", args);
    //get dims
    let mut args = args.to_owned();
    let mut dims = String::new();
    let mut xydims: Vec<i32> = Vec::new();
    let mut canvas = String::new();
    let mut count = 0;
    if !args.is_empty() {
        for slice in &args[..] { //println!("slice={:?}", slice);
            for arg in slice.lines(){
                let line = arg.to_string(); //println!("line=\n{}", line);
                if count == 0 {  //--println!("if count == 0...\n");
                    // build_base_canvas
                    dims = line.to_string();
                    canvas = draw_canvas(&[dims]).unwrap(); //println!("base returned =\n{}", canvas);
                    for dim in get_dims(&[line]).unwrap().lines() {
                        xydims.push(dim.parse().unwrap());
                    }
                    count +=1;
                } else {
                    let mut new_canvas = String::new();
                    let mut position = 0;
                    let mut iter = line.split_ascii_whitespace(); // separate args by spaces
                    let curr_command = Command {
                        sym: iter.next().unwrap().to_string(),
                        rs: iter.next().unwrap().to_string().parse().unwrap(),
                        cs: iter.next().unwrap().to_string().parse().unwrap(),
                        dir: iter.next().unwrap().to_string(),
                        dis: iter.next().unwrap().to_string().parse().unwrap(),
                    };  //println!("{:?}\n", iter.next());
                    //get canvas info
                    //println!("\n\tcurr_command:\n\t{:?}", curr_command);
                    // check for valid indices
                    // rs==ys ; xs==cs
                    let min = 0;
                    let cmax = if xydims[1]-1 < 0 { min } else { xydims[1]-1 };// check #col ==> x dimension
                    let xs = if curr_command.cs > cmax { cmax } else if curr_command.cs < 0 { min } else { curr_command.cs };
                    let rmax = if xydims[0]-1 < 0 { min } else { xydims[0]-1 };// check #row ==> y dimension
                    let ys = if curr_command.rs > rmax { rmax } else if curr_command.rs < 0 { min } else { curr_command.rs };
                    let idist = curr_command.dis;
                    let isym = curr_command.sym;
                    //println!("\n\tcolmax={}, colstart={}, rowmax={}, rowstart={}, dist={}\n", cmax, xs, rmax, ys, idist);
                    if curr_command.dir == "h" { //get direction  //println!("\thorizontal movement"); // horizontal calc
                        let hdist = if cmax-xs+1 > idist { idist } else { cmax+1-xs }; //xs == cs
                        //println!("\thorizontal path length from ({}-{} , {}) = {}\n", xs, cmax, ys, hdist);
                        let pmax = if cmax+1 > hdist {hdist-1} else {cmax};
                        for row in 0..xydims[0] {  //render canvas rs==row to care about
                            let mut curr_str = String::new();
                            curr_str = "".to_string();
                            for col in 0..xydims[1] {
                                let element = canvas.as_bytes()[position] as char;
                                let element = element.to_string();  //println!("current element: {} ", element);
                                let curr_char: &String  = if row==ys && col >= xs && col <=pmax && hdist > 0 { &isym } else { &element };
                                //println!("current_char = {}\n", curr_char);
                                curr_str = curr_str + curr_char;
                                position +=1;
                            }
                            writeln!(&mut new_canvas, "{}", curr_str).unwrap();
                            position+=1;
                        }
                    } else {  //println!("\tvertical movement"); // vertical
                        let vdist = if rmax-ys+1 > idist { idist } else { rmax+1-ys };
                        //println!("vertical path length from ({} , {}-{}) = {}\n", xs, ys, rmax, vdist);
                        let pmax = if rmax+1 > vdist {vdist-1} else {rmax};
                        for row in 0..xydims[0] {  //render canvas cs==col to care about
                            let mut curr_str = String::new();
                            curr_str = "".to_string();
                            for col in 0..xydims[1] {  //let element: String = (canvas.as_bytes()[position] as char).to_string();
                                let element = canvas.as_bytes()[position] as char;
                                let element = element.to_string();  // println!("current element: {}", element);
                                let curr_char: &String  = if ((col==xs) && (row >= ys) && (row <= pmax) && (count < vdist)) { &isym } else { &element };
                                //println!("current_char = {}", curr_char);
                                curr_str = curr_str + curr_char;
                                position +=1;
                            }
                            position+=1;
                            writeln!(&mut new_canvas, "{}", curr_str).unwrap();
                        }
                    } // end if curr_command.dir == h or v
                    canvas = new_canvas;
                }  // end if count == 0 
            }  // end for arg in slice.lines() 
        }  // end for slice in &args[..] 
    } else {
        canvas = "".to_string();
    }  // end if!args.is_empty()
    ////println!("\tRETURNING new canvas from add_to_canvas: \n{}", canvas);
    Some(canvas)
}
         
//#[test]
//fn test_add_to_canvas_01() {
//    assert_eq!(
//        97.0,
//        funct2(&[75.5, 100.5, 95.5, 265.5, -37.0]).unwrap().round()
//    );
//}


/// Parse and return the canvas dimensions specified by the tvg file
/// rows cols
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), get_dims(&[String::from("5 6")]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("".to_string()), get_dims(&[String::from("")]));
/// ```
pub fn get_dims(dims: &[String]) -> Option<String> { //--println!("\n\tfunction::get_dims called... ");
    let mut dims = dims.to_owned();
    let mut pos = 0;
    let mut coord = 0;
    let mut xydims = String::new();
    let mut v = String::new();
    let mut canvas = String::new();

    if dims.len() != 0 {
        for dim in &dims[..] { //--println!("for dim in &dims.. dim={:?}", dim);
            v = dim.to_string().chars().collect(); //--println!("v={:?}", v);
        }
    } else {
        xydims = "".to_string();
    }
    //println!("\n\tfor val in v.chars()... \n");
    for val in v.chars() {  //--println!("val= {}, pos= {}", val, pos);
        match pos {
            0 => {let xdim = &(val.to_string()); writeln!(&mut xydims, "{}", xdim); pos+=1},
            2 => {let ydim = &(val.to_string()); writeln!(&mut xydims, "{}", ydim); pos+=1},
            _ => {pos+=1},////{println!("{}", val); pos+=1},
        }
    }  //--println!("Returning xydims values: \n{}", xydims);
    //--println!("\n\tReturning from get_dims()");
    Some(xydims)
}

//#[test]
//fn test_get_dims_01() {
//    assert_eq!(Some("4 9".to_string()), funct3(&["4 9".to_string()]));
//}


