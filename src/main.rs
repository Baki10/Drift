#![allow(non_snake_case)]

use std::{io, panic};

use ratatui::{Terminal, backend::CrosstermBackend, crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::enable_raw_mode}, layout::Constraint, style::Stylize, widgets::{Block, Paragraph, Widget}};

mod core;
use Drift::utils::{self, convert_file_size};
use crate::core::browser::Browser;


fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;
    let mut terminal  = init_terminal();

    let b = Block::bordered()
            .title("Browser".fg(ratatui::style::Color::White).bold())
            .title_alignment(ratatui::layout::HorizontalAlignment::Center)
            .border_style(ratatui::style::Color::Blue)
            .bg(ratatui::style::Color::Black);

    let mut lines: Vec<ratatui::text::Line>;
    let mut paragraph;

    let mut block_height: u16 = 0;
    let mut browser: Browser = Browser::new(String::from("C:\\"))?;

    loop {

        let file_metadata: std::fs::Metadata = browser.get_entry_data()?;
        let file_info: String = convert_file_size(file_metadata.len());
        //file_info.push_str(file_metadata.file_type().);

        let mut block = b.clone().title_bottom(browser.path.clone().fg(ratatui::style::Color::White).bold());
        block = block.title_bottom(ratatui::text::Line::from(file_info).left_aligned().fg(ratatui::style::Color::White).bold());

        lines = browser.file_entries.
        iter().
        enumerate().
        map(|(index, string)| {

            let mut output = string.clone();
            output.replace_range(0..browser.path.len(), "");

            if index == browser.cursor as usize {
                ratatui::text::Line::from(output.clone()).bg(ratatui::style::Color::Blue).fg(ratatui::style::Color::White)
            } else {
                ratatui::text::Line::from(output.clone())
            }
        }).collect();
        paragraph = Paragraph::new(lines);
        paragraph = paragraph.scroll((browser.offset, 0));

        terminal.draw(|f| {

            f.render_widget(block.clone(), f.area());

            let inside_block = block.inner(f.area());
            block_height = f.area().height;

            f.render_widget(paragraph, inside_block);

        })?;

        if key_events(&mut browser, &block_height)? {
            break;
        }
    }

    terminal.clear()?;
    ratatui::restore();

    Ok(())
}

fn key_events(browser: &mut Browser, block_height: &u16) -> Result<bool, io::Error> {

    let mut should_break = false;

    if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {

                match key.code {
                    KeyCode::Down => {
                        browser.move_down(block_height);
                    }
                    KeyCode::Up => {
                        browser.move_up();
                    }
                    KeyCode::Right => {
                        browser.enter_directory();
                    }
                    KeyCode::Left => {
                        browser.back_directory();
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

fn _popup_window<T: Widget>(frame: &mut ratatui::Frame<'_>, window_title: String, horizontal_percent: u16, vertical_percent: u16, widget: T) {
    let popup_area = frame.area().centered(
        Constraint::Percentage(horizontal_percent),
        Constraint::Percentage(vertical_percent));

    let popup_block = Block::bordered().title(window_title);
    let inner_area = popup_block.inner(popup_area);

    frame.render_widget(popup_block, popup_area);
    frame.render_widget(widget, inner_area);
}

fn init_terminal() -> Terminal<CrosstermBackend<io::Stdout>> {
    let stdio = io::stdout();
    let backend = CrosstermBackend::new(stdio);
    let terminal = Terminal::new(backend);

    match terminal {
        Ok(terminal) => terminal,
        Err(_e) => {panic!("Couldn\'t open terminal!")}
    }
}