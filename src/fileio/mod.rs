use std::fs;
use regex::Regex;
use serde::{Serialize, Deserialize};
use config_file::FromConfigFile;

use crate::i4004emu::CPU;

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
    // Loads configuration file
    let cfg: Result<Data, config_file::ConfigFileError> = Data::from_config_file("config/settings.toml");
    cfg
}

pub trait SaveData {
    fn save_struct(&self, destination: &str);
}
impl SaveData for CPU{
    fn save_struct(&self, destination: &str) {
        if destination != ""{
            // RAM stuff
            fs::create_dir_all(format!("{}/ram/ram_data_char", destination)).expect("Failed to create output dir");
            let mut dest = format!("{}/ram/ram_data_char/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.ram_d).expect("Failed to write");

            fs::create_dir_all(format!("{}/ram/ram_status_char", destination)).expect("Failed to create output dir");
            dest = format!("{}/ram/ram_status_char/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.ram_s).expect("Failed to write");

            fs::create_dir_all(format!("{}/ram/ram_out", destination)).expect("Failed to create output dir");
            dest = format!("{}/ram/ram_out/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.ram_o).expect("Failed to write");

            fs::create_dir_all(format!("{}/ram/ram_address_reg", destination)).expect("Failed to create output dir");
            dest = format!("{}/ram/ram_address_reg/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.ram_addr.to_le_bytes()).expect("Failed to write");

            fs::create_dir_all(format!("{}/ram/ram_bank", destination)).expect("Failed to create output dir");
            dest = format!("{}/ram/ram_bank/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.ram_bank.to_le_bytes()).expect("Failed to write");

            // ROM stuff
            fs::create_dir_all(format!("{}/rom/rom_data", destination)).expect("Failed to create output dir");
            dest = format!("{}/rom/rom_data/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.rom).expect("Failed to write");

            fs::create_dir_all(format!("{}/rom/rom_io_lines", destination)).expect("Failed to create output dir");
            dest = format!("{}/rom/rom_io_lines/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.rom_io).expect("Failed to write");

            fs::create_dir_all(format!("{}/rom/rom_page", destination)).expect("Failed to create output dir");
            dest = format!("{}/rom/rom_page/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.rom_page.to_le_bytes()).expect("Failed to write");

            // Stack
            fs::create_dir_all(format!("{}/stack/stack_data", destination)).expect("Failed to create output dir");
            dest = format!("{}/stack/stack_data/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            let s = [self.stack[0].to_le_bytes(), self.stack[1].to_le_bytes(), self.stack[2].to_le_bytes()].concat();
            fs::write(dest, s).expect("Failed to write");

            fs::create_dir_all(format!("{}/stack/stack_ptr", destination)).expect("Failed to create output dir");
            dest = format!("{}/stack/stack_ptr/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.rom_page.to_le_bytes()).expect("Failed to write");

            // Misc
            fs::create_dir_all(format!("{}/misc/accumulator", destination)).expect("Failed to create output dir");
            dest = format!("{}/misc/accumulator/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.acc.to_le_bytes()).expect("Failed to write");

            fs::create_dir_all(format!("{}/misc/carry", destination)).expect("Failed to create output dir");
            dest = format!("{}/misc/carry/{}_{}.log", destination, self.cycle.to_string(), self.pc.to_string());
            fs::write(dest, self.carry.to_le_bytes()).expect("Failed to write");

        } else {
            // Do nothing
        }
    }
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

