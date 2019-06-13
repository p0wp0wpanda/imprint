//use std::env;
use std::fs;

fn main() {
    let filename = "img1.jpeg"; //size 800 X 800
    let contents = fs::read(filename).unwrap();

    let magic_number = &contents[0..8];
    let check_magic_number: Vec<u8> = vec![137, 80, 78, 71, 13, 10, 26, 10];

    //Check the first 8 bytes aka magic numbers of the file to check if it is a .png
    check_png(check_magic_number, magic_number);

    let ihdr_check: Vec<u8> = vec![73, 72, 68, 82]; //IHDR chunk
    let plte_check: Vec<u8> = vec![80, 76, 84, 69]; //PLTE chunk
    let idat_check: Vec<u8> = vec![73, 68, 65, 84]; //IDAT chunk
    let iend_check: Vec<u8> = vec![73, 69, 78, 68]; //IEND chunk

    //Get starting index of IHDR chunk
    let ihdr_index = check_index(ihdr_check, &contents); 
    println!("IHDR index : {}", ihdr_index);

    //Get starting index of PLTE chunk
    let plte_index = check_index(plte_check, &contents);
    println!("PLTE index : {}", plte_index);

    //Get startig index of IDAT chunk
    let idat_index = check_index(idat_check, &contents);
    println!("IDAT index : {}", idat_index);

    //Get index of IEND chunk
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

fn check_png(check_vector: Vec<u8>, magic_number: &[u8]) {
    assert_eq!(check_vector, magic_number, "\nError : Not a png file");
}

