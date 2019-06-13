//use std::env;
use std::fs;

fn main() {
    let filename = "img3.png"; //size 800 X 800
    let contents = fs::read(filename).unwrap();

    let ihdr_check: Vec<u8> = vec![73, 72, 68, 82];
    let plte_check: Vec<u8> = vec![80, 76, 84, 69];
    let idat_check: Vec<u8> = vec![73, 68, 65, 84];
    let iend_check: Vec<u8> = vec![73, 69, 78, 68];

    let ihdr_index = check_index(ihdr_check, &contents);
    println!("IHDR index : {}", ihdr_index);

    let plte_index = check_index(plte_check, &contents);
    println!("PLTE index : {}", plte_index);

    let idat_index = check_index(idat_check, &contents);
    println!("IDAT index : {}", idat_index);

    let iend_index = check_index(iend_check, &contents);
    println!("IEND index : {}", iend_index);
    
    // for (i, val) in contents.iter().enumerate() {
    //     if i % 16 == 0 {
    //         println!("");
    //     }
    //     if (64 < *val && *val < 91) || (96 < *val && *val < 123) {
    //         print!("{} ", *val as char);
    //     } else if 0 <= *val && *val <= 32 {
    //         if *val == 13 {
    //             print!("r ");
    //         } else if *val == 10 {
    //             print!("n ");
    //         }
    //     } else {
    //         print!("{} ", val);
    //     }
    // }

    // for i in 0..contents.len() {
    //     if contents[i] == 73 && contents[i+1] == 68 && contents[i+2] == 65 && contents[i+3] == 84 {
    //         idat_index = i;
    //         break;
    //     }
    // }
}

fn check_index(check_vector: Vec<u8>, file_contents: &[u8]) -> u32 {
    let mut index: u32 = 0;
    for i in 0..file_contents.len() {
        if file_contents[i] == check_vector[0] && file_contents[i+1] == check_vector[1] && file_contents[i+2] == check_vector[2] && file_contents[i+3] == check_vector[3] {
            index = i as u32;
        }
    }
    index
}
