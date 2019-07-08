// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

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
pub fn draw_canvas(nums: &[String]) -> Option<String> {
    println!("\n\tfunction::draw_canvas called... \n");
    // get the canvas size
    let mut nums = nums.to_owned();
    let mut pos = 0;
    let mut coord = 0;
    let mut xdim: u32 = 0;
    let mut ydim: u32 = 0;
    let mut v = String::new();
    let mut canvas = String::new();
    let count = nums.len();  //println!("nums.len() = {}", count); //println!("Printing nums: \n{:?}", nums);
    if count != 0 {
        for num in &nums[..] {   //println!("  {}", num); 
            v = num.to_string().chars().collect();
        }
    } else {
        v.insert_str(pos, "CANVAS COULDNT BE DRAWN");
    }

    println!("\n\tfor val in v.chars()... \n");
    //let vlen = v.len();  //  println!("vlen= {}", vlen);
    for val in v.chars() {
        match pos {
            0 => {xdim = val.to_digit(16).unwrap(); pos+=1},
            2 => {ydim = val.to_digit(16).unwrap(); pos+=1},
            _ => {println!("{}", val); pos+=1},
        }
    }

    let mut curr_str = String::new();
    println!("\n\tfor x in xdim... \n");
    for x in 0..xdim { //build row
        println!("\nx = {}", x);
        curr_str = "".to_string();
        println!("\n\tfor y in ydim... \n");
        for y in 0..ydim{ // build column
            println!("\ny = {}", y);
            //canvas.insert_str(coord, ".");
            //curr_str.insert_str(coord, ".");
            curr_str = curr_str + ".";
            coord +=1;
        }
        //canvas.insert_str(coord, "\r");
        //canvas.insert_str(coord, "\n");
        use std::fmt::Write;
        writeln!(&mut canvas, "{}", curr_str).unwrap();
        //canvas = writeln!(&mut canvas, "{}", curr_str).unwrap();
        coord+=1;
    }
    //println!("xdim = {}", xdim);
    //println!("ydim = {}", ydim);
    //println!("pos = {}", pos);
    //println!("\n\tdraw_returning {:?} \n", v);
    //Some(v)
    println!("\nPrinting CANVAS: \n{}\n", canvas);
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
/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), funct2(&[String::from("5 6")])); 
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6 7".to_string()), funct2(&[String::from("5 6 7")]));
/// ```
pub fn funct2(nums: &[String]) -> Option<String> {
    println!("\n\tfunction::funct2 called... \n");
    //let mut sigma = 0.0;
    let mut chars = String::new();
    //let xbar = funct1(nums).unwrap() as f64; //no error since mean will be Some
    let mut position = 0;
    if !nums.is_empty() {
        for val in &nums[..] {
            chars.insert_str(position, val);
            position += val.len();
    //        let temp = (val - xbar).powf(2.0);
            //chars.push(val.as_bytes());
    //        sigma = funct1(&sqnums[..]).unwrap().sqrt() as f64;
        }
    //    Some(sigma)
    } else {
        chars = "Passed in values empty in function 2".to_string();
    }
    //Some("Function 2 returning".to_string())
    Some(chars)
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
/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), funct3(&[String::from("5 6")]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("".to_string()), funct3(&[String::from("")]));
/// ```
pub fn funct3(nums: &[String]) -> Option<String> {
    println!("\n\tfunction::funct3 called... \n");
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    println!("{:?}", nums);
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
//    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
//    let mut index = nums.len();
//    if index != 0 {
//        index = (index - 1) / 2;
//        let med = nums[index];
//        Some(med)
//    } else {
//        None
//    }
    Some("Function 3 returning".to_string())
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
