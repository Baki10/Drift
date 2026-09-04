#![allow(non_snake_case)]

pub mod Colors {
    use ratatui::style::Color;
    use std::sync::OnceLock;

    pub static BACKGROUND: OnceLock<Color> = OnceLock::new();
    pub static FOREGROUND_1: OnceLock<Color> = OnceLock::new();
    pub static FOREGROUND_2: OnceLock<Color> = OnceLock::new();
    pub static CURSOR: OnceLock<Color> = OnceLock::new();
}

pub mod utils {
    use std::{fs, io};
    use ratatui::style::Color;

    use crate::Colors;

    static FILE_SIZES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    pub fn init_config(config_path: String) -> Result<(), io::Error> {

        let file_contents = fs::read_to_string(config_path)?;

        for line in file_contents.split("\n") {

            let config_line = line.replace(" ", "");
            let raw_config: Vec<&str> = config_line.split("=").collect();

            if raw_config.len() == 2 {

                let config_field = raw_config[0].to_ascii_lowercase();
                let config_field_value = raw_config[1].to_ascii_lowercase();
                let config_color = hex_to_color(config_field_value).unwrap();

                match config_field.as_str() {
                    "background" => Colors::BACKGROUND.set(config_color).unwrap(),
                    "foreground_1" => Colors::FOREGROUND_1.set(config_color).unwrap(),
                    "foreground_2" => Colors::FOREGROUND_2.set(config_color).unwrap(),
                    "cursor" => Colors::CURSOR.set(config_color).unwrap(),
                    _ => continue,
                }

            }

        }

        Ok(())
    }

    fn hex_to_color(hex: String) -> Result<Color, std::num::ParseIntError> {
        let hex = hex.trim_start_matches("#");
        let r: u8 = u8::from_str_radix(&hex[0..2], 16)?;
        let g: u8 = u8::from_str_radix(&hex[2..4], 16)?;
        let b: u8 = u8::from_str_radix(&hex[4..6], 16)?;
        Ok(Color::Rgb(r, g, b))
    }

    pub fn vec_to_string(vec: &Vec<String>) -> String {
        let mut string: String = String::new();

        for entry in vec {
            string.push_str(&entry);
            string.push('\n');
        }

        return string;
    }

    pub fn find_file_paths(path: &str, filename: String, ignorecase: bool) -> Vec<String> {
        //deep_search_directory
        let path_list = deep_scan(path).unwrap();

        let filter_results = filter_vec(&path_list, &filename, ignorecase);
        return filter_results;
    }

    pub fn filter_vec(list: &Vec<String>, filter_string: &String, ignore_case: bool) -> Vec<String> {
        let mut list_copy: Vec<String> = Vec::new();
        let mut filtered_list: Vec<String> = Vec::new();
        let filter: String;

        if ignore_case {
            for entry in list {
                list_copy.push(entry.clone().to_lowercase());
            }
            filter = filter_string.to_lowercase();
        } else {
            filter = filter_string.to_lowercase();
        }

        for entry in list_copy {
            if entry.contains(&filter) {
                filtered_list.push(entry);
            }
        }
        return filtered_list;
    }

    pub fn scan_directory(path: &String) -> Result<Vec<String>, io::Error> {
        let mut paths: Vec<String> = Vec::new();

        let fileList = fs::read_dir(path)?;

        for entry in fileList {
            let entry = entry?;

            let path = entry.path();

            let path_str = path.to_str().unwrap();
            paths.push(path_str.to_string());
        }

        Ok(paths)
    }

    fn deep_scan(path: &str) -> Result<Vec<String>, io::Error> {

        let mut paths: Vec<String> = Vec::new();
        let file_reading = fs::read_dir(path);

        match file_reading {
            Ok(file_list) => {

                for entry in file_list {

                let entry = entry?;
                let path = entry.path();
                let path_string = path.to_str();

                match path_string {
                    Some(path) => {

                        paths.push(path.to_string());
                        if entry.file_type()?.is_dir() {
                            let mut sub_paths = deep_scan(path)?;
                            paths.append(&mut sub_paths);
                        }}
                        None => return Ok(Vec::new())
                    };
                }
            },

            Err(_E) => {},
        }

        
        Ok(paths)
    }

    pub fn convert_file_size(byte_size: u64) -> String {
        
        let output: String;
        let mut size_output: f32 = byte_size as f32;
        let mut size_index: usize = 0;

        while size_output >= 1024.0 {
            if size_index < 4 {
                size_output = size_output/1024.0;
                size_index += 1;
            } else {
                break;
            }
        }
        output = format!("{}{}", size_output, FILE_SIZES[size_index]);
        return output;
    }
}