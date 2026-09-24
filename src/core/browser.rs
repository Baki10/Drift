use std::{fs, io};

use crate::utils;
use crate::Colors;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{List, ListState};

pub struct Browser {
    path: String,
    file_entries: Vec<String>,
    list_state: ListState,
}

impl Browser {

    pub fn new(current_path: String) -> Result<Self, io::Error> {

        let try_scan  = utils::scan_directory(&current_path);
        match try_scan {
            Ok(files) => {
                Ok(
                Browser {
                    path: current_path,
                    file_entries: files,
                    list_state: ListState::default().with_selected(Some(0)),
                })
            },
            Err(_e) => {Err(io::ErrorKind::NotADirectory.into())}
        }
    }

    pub fn enter_directory(&mut self) {

        if self.file_entries.len() == 0 {return;}

        let new_path: String;
        if let Some(selected) = self.list_state.selected() {
            new_path = self.file_entries[selected].clone();
        } else {
            return;
        }

        let try_scan = utils::scan_directory(&new_path);
        match try_scan {
            Ok(new_entries) => {
                self.path = new_path;
                self.file_entries = new_entries;
                self.list_state.select_first();
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
                self.list_state.select_first();
            },
            Err(_e) => {}
        }
    }

    pub fn move_down(&mut self) {

        if self.list_state.selected().unwrap() < self.file_entries.len()-1 {
            self.list_state.select_next();
        } else {
            self.list_state.select_first();
        }
    }

    pub fn move_up(&mut self) {
        if self.list_state.selected() == Some(0) {
            self.list_state.select(Some(self.file_entries.len()-1 as usize));
        } else {
            self.list_state.select_previous();
        }
    }

    pub fn generate_list(&self) -> List<'static> {

        let mut items: Vec<String> = Vec::new();

        for mut entry in self.file_entries.clone() {

            entry.replace_range(0..self.path.len(), "");
            if entry.starts_with("\\") {
                entry.replace_range(0..1, "");
            }
            items.push(entry);
        }

        let mut list = List::new(items).style(Color::White).highlight_symbol("> ").scroll_padding(2);

        if let Some(cursor) = Colors::CURSOR.get() {

            let highlight_style = Style::new().bg(*cursor);
            list = list.highlight_style(highlight_style);
        }

        return list;
    }
    
    pub fn get_list_state(&self) -> ListState {
        return self.list_state;
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_entry_data(&self) -> Option<fs::Metadata> {
        let data: Option<fs::Metadata>;
        if self.file_entries.len() == 0 {
            data = None;
        } else {
            
            match std::fs::metadata(self.file_entries[self.list_state.selected().unwrap() as usize].clone()) {
                Ok(metadata) => data = Some(metadata),
                Err(error) => panic!("{}", error)
            }

        }
        return data;
    }

    pub fn get_entry_size_string(&self) -> String {
        match self.get_entry_data() {
            Some(file_metadata) => {
                return utils::convert_file_size(file_metadata.len());
            },
            None => return String::from("0B"),
        }
    }

}