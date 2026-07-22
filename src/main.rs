#![allow(non_snake_case)]

use clap::{Parser};
use colored::*;
use std::{fs::{self, DirEntry}, vec};

#[derive(Parser, Debug)]
#[command(name="Drift")]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value_t = String::from("."))]
    path: String,

    #[arg(short, long, default_value_t = true)]
    ignorecase: bool,

    #[arg(short, long, default_value_t = 1)]
    count: u8
}




fn main() {
    let args = Args::parse();

    let mut path_list: Vec<String> = vec![];

    let _result = scan_directory(args.path.as_str(), &mut path_list);
    

    let mut match_list: Vec<String> = vec![];
    let mut regex: Vec<String> = vec![];
    let filter_results = filter_matches(&path_list, &args.filename, args.ignorecase);

    for result in filter_results {
        match_list.push(path_list[result.0].clone());
        regex.push(result.1);
    }

    print_results(&regex, &match_list);

}


fn filter_matches(list_reference: &Vec<String>, filter_reference: &String, ignoreCase: bool) -> Vec<(usize, String)> {

    let list = list_reference.clone();
    let mut modifed_list: Vec<String> = Vec::new();
    let mut output: Vec<(usize, String)> = Vec::new();
    let filter: String;

    if ignoreCase {
        for i in 0..list.len() {
            modifed_list.push(list[i].to_lowercase());
        }
        filter = filter_reference.clone().to_lowercase();
    } else {
        filter = filter_reference.clone();
    }

    for index in 0..list.len() {
        if modifed_list[index].contains(&filter) {
            let regex_index = modifed_list[index].find(&filter).unwrap();
            let regex = &list[index][regex_index..regex_index+filter.len()];
            output.push((index, regex.to_string()));
        }
    }

    return output;
}


fn print_results(regex: &Vec<String>, match_list: &Vec<String>) {
    print!("\n\n\n");

    let mut output: String = String::new();
    for index in 0..match_list.len() {
        let mut line: String = String::new();

        let splits_list: Vec<&str> = match_list[index].split(&regex[index]).collect();
        for i in 0..splits_list.len()-1 {
            line.push_str(format!("{}{}", splits_list[i], regex[index].bright_blue()).as_str());
        }
        line.push_str(splits_list[splits_list.len()-1]);

        output.push_str(&line);
        output.push('\n');
    }
    print!("{}\n\n\n", output);
}

fn scan_directory(path: &str, paths: &mut Vec<String>) -> std::io::Result<()> {

    let mut result: Result<(), std::io::Error> = Ok(());
    let fileList = fs::read_dir(path)?;
    for entry in fileList {
        let entry = entry?;

        let intermidate = entry.path();
        let path_option = intermidate.to_str();

        match path_option {
            Some(entry_path) => result = search_path(entry, entry_path, paths),
            None => return Ok(()),
        };

    }

    return result;
}

fn search_path(entry: DirEntry, entry_path: &str, paths: &mut Vec<String>) -> std::io::Result<()> {

    let fileType = entry.file_type()?;
    let mut result: Result<(), std::io::Error> = Ok(());
    if fileType.is_dir() {
        result = scan_directory(entry_path, paths);
    }
    paths.push(entry_path.to_string());

    return result;
}