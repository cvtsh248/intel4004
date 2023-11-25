use std::fs;
use regex::Regex;

pub fn load_bin(filepath: &str) -> [u8; 4096]{
    let mut output: [u8; 4096] = [0; 4096];
    
    let file = fs::read_to_string(filepath).expect("Error, failed to read");

    let seperator = Regex::new(r" |\n").expect("Invalid regex");
    let file_arr: Vec<_> = seperator.split(&file).into_iter().collect();

    for i in 0..4096{
        if i < file_arr.len(){
            output[i] = u8::from_str_radix(file_arr[i], 16).expect("Error, not hex");
            // output[i] = file_arr[i].parse::<u8>().unwrap();
        } else {
            output[i] = 0;
        }
    }

    output
}