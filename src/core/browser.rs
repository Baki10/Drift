use std::{fs, io};
use ratatui::style::Stylize;

use crate::utils;

pub struct Browser {
    path: String,
    cursor: u16,
    offset: u16,
    file_entries: Vec<String>,
}

impl Browser {

    pub fn new(current_path: String) -> Result<Self, io::Error> {

        let try_scan  = utils::scan_directory(&current_path);

        match try_scan {
            Ok(files) => {
                Ok(
                Browser {
                    path: current_path,
                    cursor: 0,
                    offset: 0,
                    file_entries: files,
                })
            },
            Err(_e) => {Err(io::ErrorKind::NotADirectory.into())}
        }


    }

    pub fn enter_directory(&mut self) {

        let try_scan = utils::scan_directory(&self.file_entries[self.cursor as usize]);

        match try_scan {
            Ok(new_entries) => {
                self.path = self.file_entries[self.cursor as usize].clone();
                self.file_entries = new_entries;
                self.cursor = 0;
                self.offset = 0;
            },
            Err(_e) => {}
        }
    }

    pub fn back_directory(&mut self) {
        let split: Vec<&str> = self.path.split('\\').collect();
        let mut new_path: String = String::new();

        if split.len() == 2 && split[1] == "" {
            return;
        }

        for index in 0..split.len()-1 {
            new_path.push_str(split[index]);
            new_path.push('\\');
        }

        if split.len() > 2 {
            new_path.pop();
        }

        let try_scan = utils::scan_directory(&new_path);

        match try_scan {
            Ok(new_entries) => {
                self.path = new_path;
                self.file_entries = new_entries;
                self.cursor = 0;
                self.offset = 0;
            },
            Err(_e) => {}
        }
    }

    pub fn move_down(&mut self, block_height: &u16) {

        let line_number: u16 = self.file_entries.len() as u16;

        if line_number+3 > *block_height {
            if self.offset < line_number - block_height + 3 {
                self.offset += 1;
            }
        }

        if self.cursor < line_number-1 {
            self.cursor += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn generate_lines(&mut self) -> Vec<ratatui::text::Line<'static>> {

        let mut lines = Vec::new();
        for (index, string) in self.file_entries.iter().enumerate() {
            
            let mut output = string.clone();
            output.replace_range(0..self.path.len(), "");

            if output.starts_with("\\") {
                output.replace_range(0..1, "");
            }

            if index == self.cursor as usize {
                lines.push(ratatui::text::Line::from(output).bg(ratatui::style::Color::Blue).fg(ratatui::style::Color::White));
            } else {
                lines.push(ratatui::text::Line::from(output));
            }
        }
        return lines;
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_offset(&self) -> u16 {
        self.offset.clone()
    }

    pub fn get_entry_data(&self) -> Result<fs::Metadata, io::Error> {
        let data = std::fs::metadata(self.file_entries[self.cursor as usize].clone());
        return data;
    }
}