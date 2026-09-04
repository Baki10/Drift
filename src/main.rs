#![allow(non_snake_case)]

use std::{io, panic};

use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::{self, event};
use ratatui::layout::Constraint;
use ratatui::style::Stylize;
use ratatui::widgets::{self, Block, Paragraph, Widget};
use ratatui::text::Line;
use ratatui::style::Color;

mod core;
use Drift::Colors;
use Drift::utils;
use crate::core::browser::Browser;

fn main() -> Result<(), io::Error> {

    crossterm::terminal::enable_raw_mode()?;
    let mut terminal  = init_terminal();
    utils::init_config("C:\\Users\\Branko\\Documents\\Projects\\Drift\\src\\config".to_string())?;

    let (BACKGROUND, FOREGROUND_1, FOREGROUND_2, _CURSOR) = init_constants()?;

    let block_style = Block::bordered()
            .title("Browser".fg(FOREGROUND_1).bold())
            .title_alignment(ratatui::layout::HorizontalAlignment::Center)
            .border_style(FOREGROUND_2)
            .border_type(widgets::BorderType::Rounded)
            .bg(BACKGROUND);

    let mut block_height: u16 = 0;
    let mut browser: Browser = Browser::new(String::from("C:\\Users\\Branko\\Desktop"))?;



    loop {
 
        let file_info: String = browser.get_entry_size_string();
        let file_info_title = Line::from(file_info).left_aligned().fg(FOREGROUND_1).bold();
        let path_title = browser.get_path().fg(FOREGROUND_1).bold();

        let mut block = block_style.clone().title_bottom(path_title);
        block = block.title_bottom(file_info_title);

        let lines = browser.generate_lines();
        let mut paragraph = Paragraph::new(lines);
        paragraph = paragraph.scroll((browser.get_offset(), 0));


        terminal.draw(|frame| {

            let inside_block = block.inner(frame.area());
            block_height = frame.area().height;

            frame.render_widget(block.clone(), frame.area());
            frame.render_widget(paragraph, inside_block);

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

    if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {

                match key.code {
                    event::KeyCode::Down => {
                        browser.move_down(block_height);
                    }
                    event::KeyCode::Up => {
                        browser.move_up();
                    }
                    event::KeyCode::Right => {
                        browser.enter_directory();
                    }
                    event::KeyCode::Left => {
                        browser.back_directory();
                    }
                    event::KeyCode::Esc => {
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

fn init_constants() -> Result<(Color, Color, Color, Color), io::Error> {

    let BACKGROUND: Color;
    let FOREGROUND_1: Color;
    let FOREGROUND_2: Color;
    let CURSOR: Color;
    
    if let Some(background) = Colors::BACKGROUND.get() {
        BACKGROUND = *background;
    } else {
        return Err(io::ErrorKind::InvalidData.into());
    }
    if let Some(foreground_1) = Colors::FOREGROUND_1.get() {
        FOREGROUND_1 = *foreground_1;
    } else {
        return Err(io::ErrorKind::InvalidData.into());
    }
    if let Some(foreground_2) = Colors::FOREGROUND_2.get() {
        FOREGROUND_2 = *foreground_2;
    } else {
        return Err(io::ErrorKind::InvalidData.into());
    }
    if let Some(cursor) = Colors::CURSOR.get() {
        CURSOR = *cursor;
    } else {
        return Err(io::ErrorKind::InvalidData.into());
    }

    Ok((BACKGROUND, FOREGROUND_1, FOREGROUND_2, CURSOR))
}

fn init_terminal() -> ratatui::Terminal<CrosstermBackend<io::Stdout>> {
    let stdio = io::stdout();
    let backend = CrosstermBackend::new(stdio);
    let terminal = ratatui::Terminal::new(backend);

    match terminal {
        Ok(terminal) => terminal,
        Err(_e) => {panic!("Couldn\'t open terminal!")}
    }
}