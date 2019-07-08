// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::fmt::Write;

///! Functions to help render a .tvg file 
///! on standard output

/// Type of asciilines function. If the function
/// is ill-defined, `None` will be returned.
pub type AsciiFn = fn(&[String]) -> Option<String>;

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
    println!("\n\tfunction::draw_canvas called... \n");
    // get the canvas size
    //let mut dims = dims.to_owned();
    let mut pos = 0;
    let mut coord = 0;
    let mut xdim: u32 = 0;
    let mut ydim: u32 = 0;
    let mut xydims = String::new();
    //let mut v = String::new();
    let mut canvas = String::new();
    //------
    //let count = dims.len();  //println!("nums.len() = {}", count); //println!("Printing nums: \n{:?}", nums);
    //if dims.len() != 0 {
    //    for dim in &dims[..] {   //println!("  {}", num); 
    //        v = dim.to_string().chars().collect();
    //    }
    //} else {
    //    canvas.insert_str(pos, "CANVAS COULDNT BE DRAWN");
    //}

    //println!("\n\tfor val in v.chars()... \n");
    //for val in v.chars() {
    //    match pos {
    //        0 => {xdim = val.to_digit(16).unwrap(); pos+=1},
    //        2 => {ydim = val.to_digit(16).unwrap(); pos+=1},
    //        _ => {println!("{}", val); pos+=1},
    //    }
    //}
    //---------------new stuff to replace above junk
    if dims.len() != 0 {
        xydims = get_dims(&dims).unwrap();
        for dim in xydims.lines() {  //println!("dim = {}", dim);
            match pos {
                0 => {xdim = dim.parse().unwrap(); pos+=1},
                1 => {ydim = dim.parse().unwrap(); pos+=1},
                _ => {pos+=1},
            }
        }
        println!("!!!!get_dims put into xydims: \n{}", xydims);
    }


    // ----- 
    let mut curr_str = String::new();
    for x in 0..xdim { //build row  //println!("\nx = {}", x);
        curr_str = "".to_string();
        for y in 0..ydim{ // build column // println!("\ny = {}", y);
            curr_str = curr_str + ".";
            coord +=1;
        }
        //use std::fmt::Write;
        writeln!(&mut canvas, "{}", curr_str).unwrap();
        coord+=1;
    }  //println!("xdim = {}", xdim); //println!("ydim = {}", ydim); //println!("pos = {}", pos);
    //println!("\nPrinting CANVAS: \n{}\n", canvas);
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
    println!("\n\tfunction::add_to_canvas called... \n");
    println!("Printing passed in args: \n {:?} \n", args);
    //get dims
    let args = args.to_owned();
    let mut dims = String::new();
    let mut chars = String::new();
    let mut position = 0;
    let mut count = 0;
    if !args.is_empty() {
        for arg in &args[..] {
            for line in arg.lines() {
                //println!("line = \n{}", line);
                match count {
                    0 => {dims = get_dims(&[line.to_string()]).unwrap(); count+=1},
                    _ => {println!("line{}:{}\n", count, line); count +=1},
                }
            }
        }
    } else {
        chars = "Passed in values empty in function 2".to_string();
    }
    //println!("dims = \n{}", dims);
    println!("\tadd funct returning: \n{}", chars);
    Some(chars)
}

            //for val in &args[..] {
            //    chars.insert_str(position, val);
            //    position += val.len();
            //}
            
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
pub fn get_dims(dims: &[String]) -> Option<String> {
    println!("\n\tfunction::get_dims called... \n");
    let mut dims = dims.to_owned();
    let mut pos = 0;
    let mut coord = 0;
    let mut xydims = String::new();
    let mut v = String::new();
    let mut canvas = String::new();

    if dims.len() != 0 {
        for dim in &dims[..] {   //println!("  {}", num); 
            v = dim.to_string().chars().collect();
        }
    } else {
        xydims = "".to_string();
    }
    //println!("\n\tfor val in v.chars()... \n");
    for val in v.chars() {
        //println!("val= {}", val);
        match pos {
            //0 => {let xdim = val.to_string(); xydims = xydims + &xdim; pos+=1},
            //0 => {writeln!(&mut xydims, "{}", &val.to_string()).unwrap(); pos+=1},
            //);let xdim = val.to_string(); xydims = xydims + &xdim; pos+=1},
            0 => {let xdim = &(val.to_string()); writeln!(&mut xydims, "{}", xdim); pos+=1},
            2 => {let ydim = &(val.to_string()); writeln!(&mut xydims, "{}", ydim); pos+=1},
            //2 => {let ydim = val.to_string(); xydims = xydims + &ydim; pos+=1},
            _ => {println!("{}", val); pos+=1},
        }
    }
    println!("Returning xydims values: \n{}", xydims);
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
    println!("\n\tfunction::funct4 called... \n");
    let mut nums = nums.to_owned();
    if !nums.is_empty() {
    //    for val in &nums[..] {
    //        sum += val.powf(2.0);
    //    }
    //    norm = sum.sqrt();
        println!("num is not empty: {:?}", nums);
        //Some(nums)
        Some("nums is not empty".to_string())
    } else {
    //}
    //Some(norm)
        Some("Function 4 returning".to_string())
    }
}

//--Add tests here
