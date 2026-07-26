#![allow(non_snake_case)]



pub mod utils {
    use std::{fs, io};

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

    pub fn scan_directory(path: &str) -> Result<Vec<String>, io::Error> {
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
}