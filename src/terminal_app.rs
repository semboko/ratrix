use crossterm::{
    cursor::{Hide, MoveTo},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize},
};
use std::{io::{self, stdout, Stdout}, time::Duration};

use crate::tetris_engine::TetrisEngine;

#[derive(Debug)]
pub struct App {
    event_buffer: Vec<Event>,
    rerender_required: bool,
    exit: bool,
    sout: Stdout,
    init_terminal_size: (u16, u16),
}

impl App {
    pub fn new() -> Self {
        return Self {
            event_buffer: vec![],
            rerender_required: true,
            sout: stdout(),
            exit: false,
            init_terminal_size: terminal::size().unwrap(),
        };
    }

    fn handle_key<F>(&mut self, f: F) -> io::Result<()>
    where
        F: FnOnce(KeyCode) -> (),
    {
        let e = event::read()?;
        if let event::Event::Key(key) = e {
            match key.code {
                KeyCode::Esc => self.exit = true,
                _ => {
                    self.rerender_required = true;
                    f(key.code)
                },
            }
        }
        Ok(())
    }

    pub fn setup(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.sout, Hide, EnterAlternateScreen)?;
        println!("\x1b[?1049h"); // Enter Alternate Screen Mode
        Ok(())
    }

    pub fn teardown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        let (cols, rows) = self.init_terminal_size;
        execute!(self.sout, SetSize(cols, rows), LeaveAlternateScreen)?;
        println!("\x1b[?1049l"); // Leave Alternate Screen Mode
        Ok(())
    }

    pub fn run(&mut self, engine: &mut TetrisEngine) -> io::Result<()> {
        while !self.exit {
            // Mainloop:
            // 1. Handle key events
            if event::poll(Duration::from_millis(16))?{
                self.handle_key(|key: KeyCode| match key {
                    _ => {}
                })?;
            }

            // 2. Refresh screen if needed
            if self.rerender_required {
                execute!(self.sout, Clear(terminal::ClearType::All), MoveTo(0, 0))?;
                println!("renrer");
                self.rerender_required = false;
            }
        }
        Ok(())
    }
}
