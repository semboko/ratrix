use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, Show},
    event::{self, KeyCode},
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize, SetTitle},
};
use std::{
    io::{self, BufWriter, Stdout},
    time::Duration,
};

use crate::tetris_engine::TetrisEngine;

#[derive(Debug)]
pub struct App {
    rerender_required: bool,
    exit: bool,
    sout: BufWriter<Stdout>,
    init_terminal_size: (u16, u16),
}

impl App {
    pub fn new() -> Self {
        return Self {
            rerender_required: true,
            sout: io::BufWriter::new(io::stdout()),
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
                _ => f(key.code),
            }
        }
        Ok(())
    }

    pub fn setup(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.sout, Hide, EnterAlternateScreen, SetTitle("Ratrix"))?;
        println!("\x1b[?1049h"); // Enter Alternate Screen Mode
        Ok(())
    }

    pub fn teardown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        let (cols, rows) = self.init_terminal_size;
        execute!(self.sout, SetSize(cols, rows), Show, LeaveAlternateScreen)?;
        println!("\x1b[?1049l"); // Leave Alternate Screen Mode
        Ok(())
    }

    pub fn run(&mut self, engine: &mut TetrisEngine) -> io::Result<()> {
        let mut frames_counter = 0;
        while !self.exit {
            // Mainloop:
            // 1. Handle key events
            if event::poll(Duration::from_millis(16))? {
                self.handle_key(|key: KeyCode| match key {
                    KeyCode::Right => engine.move_current_shape(1, 0),
                    KeyCode::Left => engine.move_current_shape(-1, 0),
                    KeyCode::Up => engine.move_current_shape(0, -1),
                    KeyCode::Down => engine.move_current_shape(0, 1),
                    _ => {}
                })?;
            }

            // 2. Perform engine changes

            // 2.1 Render is required if engine was changed
            if engine.changed {
                self.rerender_required = true;
            }

            // 3. Refresh screen if needed
            if self.rerender_required {
                // 2.1 Clear the screen
                execute!(self.sout, Clear(terminal::ClearType::All), MoveTo(0, 0))?;

                // 2.2 Draw stuff
                for line in engine.get_lines() {
                    println!("{}", line);
                    execute!(self.sout, MoveToColumn(0))?;
                }

                // 2.3 Render is not required anymore until the next key press is detected
                self.rerender_required = false;
                engine.changed = false;
                execute!(
                    self.sout,
                    SetTitle(format!("Ratrix ({} frame)", frames_counter))
                )?;
                frames_counter += 1;
            }
        }
        Ok(())
    }
}
