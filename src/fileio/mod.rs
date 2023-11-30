use std::fs;
use regex::Regex;
use serde::{Serialize, Deserialize};
use config_file::FromConfigFile;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data{
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config{
    pub step: bool,
    pub filepath: String,
    pub maxcycles: i32,
    pub outputpath: String
}

pub fn load_config() -> Result<Data, config_file::ConfigFileError> {
    let cfg: Result<Data, config_file::ConfigFileError> = Data::from_config_file("config/settings.toml");
    cfg
}

pub fn load_bin(filepath: &str) -> [u8; 4096]{
    // Loads a file and returns it as an array
    let mut output: [u8; 4096] = [0; 4096];
    
    let file = fs::read_to_string(filepath).expect("Error, failed to read");

    let seperator = Regex::new(r" |\n").expect("Invalid regex");
    let file_arr: Vec<_> = seperator.split(&file).into_iter().collect();

    for i in 0..4096{
        if i < file_arr.len(){
            output[i] = u8::from_str_radix(file_arr[i], 16).expect("Error, not hex");

        } else {
            output[i] = 0;
        }
    }

    output
}

