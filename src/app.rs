use crossterm::event::{Event, self, KeyCode};
use tui::{backend::Backend, Terminal};
use crate::ui;
use std::{io, time::{Duration, Instant}};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
        }
    }

    pub fn on_input(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }


}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {                    
                KeyCode::Char(c) => app.on_input(c),
                KeyCode::Left => {},
                KeyCode::Right => {},
                KeyCode::Up => {},
                KeyCode::Down => {}
                _ => {}
            }
        }
        
        if app.should_quit {
            return Ok(());
        }
    }
}