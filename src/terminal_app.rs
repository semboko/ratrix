use crossterm::event::{self, KeyCode};
use std::{
    io::{self},
    time::Duration,
};

use crate::renderer;
use crate::tetris_engine::TetrisEngine;

#[derive(Debug)]
pub struct App {
    rerender_required: bool,
    exit: bool,
    renderer: renderer::Renderer,
}

impl App {
    pub fn new() -> Self {
        return Self {
            rerender_required: true,
            exit: false,
            renderer: renderer::Renderer::new(),
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
        self.renderer.setup()?;
        Ok(())
    }

    pub fn teardown(&mut self) -> io::Result<()> {
        self.renderer.teardown()?;
        Ok(())
    }

    pub fn run(&mut self, engine: &mut TetrisEngine) -> io::Result<()> {
        while !self.exit {
            // Mainloop:
            // 1. Handle key events
            if event::poll(Duration::from_millis(16))? {
                self.handle_key(|key: KeyCode| match key {
                    KeyCode::Right => engine.move_current_shape(1, 0),
                    KeyCode::Left => engine.move_current_shape(-1, 0),
                    KeyCode::Up => engine.rotate(),
                    KeyCode::Down => engine.move_current_shape(0, 1),
                    _ => {}
                })?;
            }

            // 2. Perform engine changes
            engine.update();

            // 2.1 Render is required if engine was changed
            if engine.changed {
                self.rerender_required = true;
            }

            // 3. Refresh screen if needed
            if self.rerender_required {
                self.renderer.flush_changes(&(engine.get_state()))?;
                self.rerender_required = false;
                engine.changed = false;
            }
        }
        Ok(())
    }
}
