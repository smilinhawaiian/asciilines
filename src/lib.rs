//--------------------------------------------------------
// ALL THESE THINGS WERE IN THE STATS/SRC/LIB.RS FILE
// Copyright © 2019 Sharice Mayer
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
//
// THIS IS A STUB TO START THE ASCII PROGRAM WITH

///! CURRENTLY STUBBED FOR STRUCTURAL INFORMATION ONLY
///! Functions to help render a .tvg file 
///! on standard output

/// Type of asciilines function. If the function
/// is ill-defined, `None` will be returned.
pub type AsciiFn = fn(&[String]) -> Option<String>;

/// Gets the canvas size from the first line of the tvg file.
/// This line contains the integer dimensions: 
/// rows columns
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("5 6".to_string()), funct1(&[5 6]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some("0 0".to_string()), funct1(&[0 0]));
/// ```
pub fn funct1(nums: &[String]) -> Option<String> {
    // get the canvas size
    //let count = nums.len() as f64;
    //let mut arithmetic = 0.0;
    //let mut sum = 0.0;
    //if count != 0.0 {
    //    for num in &nums[..] {
    //        sum += num;
    //    }
    //    arithmetic = sum / count;
    //}
    //Some(arithmetic)
    Some("function 1 returning".to_string())
}

//#[test]
//fn test_funct1_100() {
//    assert_eq!(Some(100.0), funct1(&[75.5, 100.5, 95.5, 265.5, -37.0]));
//}

//#[test]
//fn test_funct1_single() {
//    assert_eq!(Some(25.0), funct1(&[25.0]));
//}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(None, funct2(&[]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some(0.0), funct2(&[1.0, 1.0]));
/// ```
pub fn funct2(nums: &[String]) -> Option<String> {
    //let mut sigma = 0.0;
    //let mut sqnums = Vec::new();
    //let xbar = funct1(nums).unwrap() as f64; //no error since mean will be Some
    //if !nums.is_empty() {
    //    for val in &nums[..] {
    //        let temp = (val - xbar).powf(2.0);
    //        sqnums.push(temp);
    //        sigma = funct1(&sqnums[..]).unwrap().sqrt() as f64;
    //    }
    //    Some(sigma)
    //} else {
    //    None
    //}
    Some("Function 2 returning".to_string())
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

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(None, funct3(&[]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some(0.0), funct3(&[0.0, 0.5, -1.0, 1.0]));
/// ```
pub fn funct3(nums: &[String]) -> Option<String> {
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

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use asciilines::*;
/// assert_eq!(Some(0.0), funct4(&[]));
/// ```
/// ```
/// # use asciilines::*;
/// assert_eq!(Some(5.0), funct4(&[-3.0, 4.0]));
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
    //let mut norm = 0.0;
    //let mut sum = 0.0;
    if !nums.is_empty() {
    //    for val in &nums[..] {
    //        sum += val.powf(2.0);
    //    }
    //    norm = sum.sqrt();
        println!("num is not empty: {:?}", nums);
        //Some("nums is empty".to_string())
    };
    //}
    //Some(norm)
    Some("Function 4 returning".to_string())
}

//--Add tests here
