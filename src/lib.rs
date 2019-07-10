// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::fmt::Write;
use std::fmt::Debug;

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

/// Draw canvas from tvg file input.
/// Canvas size input from the first line of the tvg file.
/// This line contains the integer dimensions: 
/// rows columns
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), draw_canvas(&[String::from("5 6")]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("0 0".to_string()), draw_canvas(&[String::from("0 0")]));
/// ```
pub fn draw_canvas(dims: &[String]) -> Option<String> {
    ////println!("\n\tfunction::draw_canvas called... ");
    // get the canvas size
    //--println!("dims= {:?}",dims);
    let mut xydims: Vec<u32> = Vec::new();
    let mut canvas = String::new();
    // get dimentions for canvas
    if dims.len() != 0 {
        for line in get_dims(&dims).unwrap().lines() {  //--println!("printing line.. {:?}", line);
            xydims.push(line.parse().unwrap()); //println!("line=\n{}",line);
        }
    }
    // build base canvas
    let mut curr_str = String::new(); //--println!("xydims = {:?}", xydims);
    for x in 0..xydims[0] { //build rows
        curr_str = "".to_string();
        for y in 0..xydims[1] { // build columns
            curr_str = curr_str + ".";
        }
        writeln!(&mut canvas, "{}", curr_str).unwrap();
    }  //println!("xdim = {}", xdim); //println!("ydim = {}", ydim); //println!("pos = {}", pos);
    //println!("\nPrinting CANVAS: \n{}\n", canvas);
    ////println!("\n\tReturning from draw_canvas()");
    Some(canvas)
}

//#[test]
//fn test_funct1_100() {
//    assert_eq!(Some(100.0), funct1(&[75.5, 100.5, 95.5, 265.5, -37.0]));
//}

//#[test]
//fn test_funct1_single() {
//    assert_eq!(Some(25.0), funct1(&[25.0]));
//}

// assert_eq!(None, funct2(&[]));
/// Adds characters to canvas as given by tvg file.
/// char rowstart colstart direction length
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("* 5 6 h 2".to_string()), add_to_canvas(&[String::from("* 5 6 h 2")])); 
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("# 5 6 v 1".to_string()), add_to_canvas(&[String::from("# 5 6 v 1")]));
/// ```
pub fn add_to_canvas(args: &[String]) -> Option<String> {
    //println!("\nTesting backtrace...\n");
    ////println!("\n\tfunction::add_to_canvas called... ");
    //println!("Printing passed in args: \n {:?} \n", args);
    //get dims
    let mut args = args.to_owned();
    let mut dims = String::new();
    let mut xydims: Vec<i32> = Vec::new();
    let mut canvas = String::new();
    let mut count = 0;
    if !args.is_empty() {
        for slice in &args[..] {
            //println!("arg={:?}", arg);
            for arg in slice.lines(){
                let line = arg.to_string(); //println!("line=\n{}", line);
                if count == 0 {  //--println!("if count == 0...\n");
                    // build_base_canvas
                    dims = line.to_string();
                    //--println!("dims= {}", dims);
                    //println!("Testing backtrace...");
                    canvas = draw_canvas(&[dims]).unwrap(); //println!("base returned =\n{}", canvas);
                    //--println!("Testing backtrace...");
                    for dim in get_dims(&[line]).unwrap().lines() {
                        xydims.push(dim.parse().unwrap());
                    }
                    count +=1;
                } else {  //println!("Need to implement parsing here\n {}", line);
                    //--println!("count > 0 {}...\n", count);
                    //println!("...\n");
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
                    //--println!("command created...\n");
                    let min = 0;
                    let cmax = if xydims[1]-1 < 0 { min } else { xydims[1]-1 };// check #col ==> x dimension
                    let cs = if curr_command.cs > cmax { cmax } else if curr_command.cs < 0 { min } else { curr_command.cs };
                    let rmax = if xydims[0]-1 < 0 { min } else { xydims[0]-1 };// check #row ==> y dimension
                    let rs = if curr_command.rs > rmax { rmax } else if curr_command.rs < 0 { min } else { curr_command.rs };
                    let idist = curr_command.dis;
                    let isym = curr_command.sym;
                    ////println!("\tcolmax={}, colstart={}, rowmax={}, rowstart={}, dist={}\n", cmax, cs, rmax, rs, idist);
                    if curr_command.dir == "h" { //get direction  //println!("\thorizontal movement"); // horizontal calc
                        let hdist = if cmax-cs+1 > idist { idist } else { cmax+1-cs };
                        ////println!("\thorizontal path length from ({}-{} , {}) = {}\n", cs, cmax, rs, hdist);
                        //canvas[xind][yind] == curr_command.sym for each in path
                        for row in 0..xydims[0] {  //render canvas rs==row to care about
                            let mut curr_str = String::new();
                            curr_str = "".to_string();
                            for col in 0..xydims[1] {
                                let element = canvas.as_bytes()[position] as char;
                                let element = element.to_string();
                                ////println!("current element: {} ", element);
                                let curr_char: &String  = if row==rs && col >= cs && col <=cmax { &isym } else { &element };
                                ////println!("current_char = {}\n", curr_char);
                                curr_str = curr_str + curr_char;
                                position +=1;
                            }
                            writeln!(&mut new_canvas, "{}", curr_str).unwrap();
                            position+=1;
                        }
                    } else {  //println!("\tvertical movement"); // vertical
                        let vdist = if rmax-rs+1 > idist { idist } else { rmax+1-rs };
                        ////println!("vertical path length from ({} , {}-{}) = {}\n", cs, rs, rmax, vdist);
                        for row in 0..xydims[0] {  //render canvas cs==col to care about
                            let mut curr_str = String::new();
                            curr_str = "".to_string();
                            for col in 0..xydims[1] {
                                let element: String = (canvas.as_bytes()[position] as char).to_string();
                                ////println!("current element: {}", element);
                                let curr_char: &String  = if col==cs && row >= rs && row <= rmax { &isym } else { &element };
                                ////println!("current_char = {}", curr_char);
                                curr_str = curr_str + curr_char;
                                position +=1;
                            }
                            position+=1;
                            writeln!(&mut new_canvas, "{}", curr_str).unwrap();
                        }
                    } // end if curr_command.dir == h or v
                    canvas = new_canvas;
                    //println!("\nprinting current canvas: \n{}", canvas);
                }  // end if count == 0 
                //println!("after if count, canvas: \n{}", canvas);
            }  // end for arg in slice.lines() 
            ////println!("after for arg in slice.lines(), canvas: \n{}", canvas);
        }  // end for slice in &args[..] 
    } else {
        canvas = "".to_string();
    }  // end if!args.is_empty()
    //--println!("after for arg in slice.lines(), canvas: \n{}", canvas);
    //println!("dims = \n{}", dims);
    ////println!("\tRETURNING new canvas from add_to_canvas: \n{}", canvas);
    ////println!("\n\tReturning from add_to_canvas() ");
    Some(canvas)
}

            
//#[test]
//fn test_funct2_97() {
//    assert_eq!(
//        97.0,
//        funct2(&[75.5, 100.5, 95.5, 265.5, -37.0]).unwrap().round()
//    );
//}

//#[test]
//fn test_funct2_single() {
//    assert_eq!(Some(0.0), funct2(&[25.0]));
//}

// assert_eq!(None, funct3(&[]));
// assert_eq!(Some(0.0), funct3(&[0.0, 0.5, -1.0, 1.0]));
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
//fn test_funct3_95() {
//    assert_eq!(Some(95.5), funct3(&[75.5, 100.5, 95.5, 265.5, -37.0]));
//}

//#[test]
//fn test_funct3_single() {
//    assert_eq!(Some(25.0), funct3(&[25.0]));
//}

// assert_eq!(Some(0.0), funct4(&[]));
// assert_eq!(Some(5.0), funct4(&[-3.0, 4.0]));
/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), funct4(&[String::from("5 6")]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("".to_string()), funct4(&[String::from("")]));
/// ```
//pub fn funct4(nums: &[f64]) -> Option<f64> {
//    let mut norm = 0.0;
//    let mut sum = 0.0;
//    if !nums.is_empty() {
//        for val in &nums[..] {
//            sum += val.powf(2.0);
//        }
//        norm = sum.sqrt();
//    }
//    Some(norm)
//}

//#[test]
//fn test_funct4_311() {
//    assert_eq!(
//        311.0,
//        funct4(&[75.5, 100.5, 95.5, 265.5, -37.0]).unwrap().round()
//    );
//}

//#[test]
//fn test_funct4_single() {
//    assert_eq!(Some(25.0), funct4(&[25.0]));
//}


pub fn funct4(nums: &[String]) -> Option<String> {
    ////println!("\n\tfunction::funct4 called... ");
    let mut ret = String::new();
    let mut nums = nums.to_owned();
    if !nums.is_empty() {
    //    for val in &nums[..] {
    //        sum += val.powf(2.0);
    //    }
    //    norm = sum.sqrt();
        println!("num is not empty: {:?}", nums);
        //Some(nums)
        //Some("nums is not empty".to_string())
        ret = "nums_is_not_empty".to_string();
    } else {
    //}
    //Some(norm)
        //Some("Function 4 returning".to_string())
        ret = "empty".to_string();
    }
    ////println!("\n\tReturning from funct4()");
    Some(ret)
}

//--Add tests here
