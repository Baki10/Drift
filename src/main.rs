#![allow(non_snake_case)]

use std::io;

use ratatui::{Terminal, backend::CrosstermBackend, crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::enable_raw_mode}, style::Stylize, widgets::{Block, Paragraph}};


use Drift::utils;

fn main() -> Result<(), io::Error> {
    //let args = Args::parse();
    //find_file("", String::from(""), true);

    enable_raw_mode()?;
    let stdio = io::stdout();
    let backend = CrosstermBackend::new(stdio);
    let mut terminal = Terminal::new(backend)?;

    let block = Block::bordered()
            .title("Directory Tree".bold())
            .title_alignment(ratatui::layout::HorizontalAlignment::Center)
            .border_style(ratatui::style::Color::Blue)
            .bg(ratatui::style::Color::Black);

    let mut current_path: &str = "C:\\";

    //let file_paths: Vec<String> = utils::find_file_paths("C:\\Users", String::from("main.rs"), true);
    let mut file_paths: Vec<String> = utils::scan_directory(current_path)?;
    let line_number = file_paths.len();


    let mut lines: Vec<ratatui::text::Line>;
    let mut paragraph;

    let mut change_path: bool = false;
    let mut cursor: u16 = 0;
    let mut offset: u16 = 0;
    let mut block_height: u16 = 0;

    loop {
        
        if change_path {
            current_path = file_paths[cursor as usize].as_str();
            file_paths = utils::scan_directory(current_path)?;
            cursor = 0;
            offset = 0;
            change_path = false;
        }

        lines = file_paths.
        iter().
        enumerate().
        map(|(index, string)| {
            if index == cursor as usize {
                ratatui::text::Line::from(string.clone()).bg(ratatui::style::Color::Blue).fg(ratatui::style::Color::Black)
            } else {
                ratatui::text::Line::from(string.clone())
            }
        }).collect();
        paragraph = Paragraph::new(lines);

        terminal.draw(|f| {

            f.render_widget(block.clone(), f.area());

            let inside_block = block.inner(f.area());

            block_height = f.area().height;

            f.render_widget(paragraph.clone().scroll((offset, 0)), inside_block);

        })?;

        if key_events(&mut offset, &(line_number as u16), &block_height, &mut cursor, &mut change_path)? {
            break;
        }
    }

    terminal.clear()?;
    ratatui::restore();
    Ok(())
}


fn key_events(offset: &mut u16, line_number: &u16, block_height: &u16, cursor: &mut u16, change_path: &mut bool) -> Result<bool, io::Error> {

    let mut should_break = false;
    if let Event::Key(key) = event::read()? {
            
            if key.kind == KeyEventKind::Press {

                match key.code {
                    KeyCode::Down => {

                        if *line_number+3 > *block_height {
                            if *offset < line_number - block_height + 3 {
                                *offset += 1;
                            }
                        }

                        if *cursor < *line_number-1 {
                            *cursor += 1;
                        }
                    }
                    KeyCode::Up => {
                        if *offset > 0 {
                            *offset -= 1;
                        }
                        if *cursor > 0 {
                            *cursor -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        *change_path = true;
                    }
                    KeyCode::Esc => {
                        should_break = true;
                    }
                    _ => {}
                }

            }

        }
    Ok(should_break)
}