use std::io::{self, BufWriter, Stdout};

use crossterm::{
    cursor::{Hide, MoveTo, MoveToColumn, Show},
    execute,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen, SetSize, SetTitle},
};

use crate::tetris_engine::GameState;

#[derive(Debug)]
pub struct Renderer {
    sout: BufWriter<Stdout>,
    init_terminal_size: (u16, u16),
}

impl Renderer {
    pub fn new() -> Self {
        return Self {
            sout: io::BufWriter::new(io::stdout()),
            init_terminal_size: terminal::size().unwrap(),
        };
    }

    pub fn setup(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.sout, Hide, EnterAlternateScreen, SetTitle("Ratrix"))?;
        // println!("\x1b[?1049h"); // Enter Alternate Screen Mode
        Ok(())
    }

    pub fn flush_changes(&mut self, state: &GameState) -> io::Result<()> {
        // 2.1 Clear the screen
        execute!(self.sout, Clear(terminal::ClearType::All), MoveTo(0, 0))?;

        // 2.2 Draw stuff
        for line in self.get_playfield_lines(&state) {
            println!("{}", line);
            execute!(self.sout, MoveToColumn(0))?;
        }
        Ok(())
    }

    fn render_line(&self, line: &u16) -> String {
        let mut result = String::from("");
        for i in (0..10).rev() {
            if (line >> i & 1) == 1 {
                result.push_str("🟧");
            } else {
                result.push_str("⬜");
            }
        }
        result
    }

    fn render_piece(&self, piece: &u16) -> Vec<String> {
        let mut result = Vec::new();
        let mut min_col = 4;
        let mut max_col = 0;
        let mut min_row = 4;
        let mut max_row = 0;

        // Determine the bounding box of the piece (non-empty columns and rows)
        for row in 0..4 {
            let line = (piece >> (row * 4)) & 0b1111;
            if line != 0 {
                min_row = min_row.min(row);
                max_row = max_row.max(row);
            }
            for col in 0..4 {
                if (line >> (3 - col)) & 1 == 1 {
                    min_col = min_col.min(col);
                    max_col = max_col.max(col);
                }
            }
        }

        // Convert the cropped piece into a vector of strings
        for row in min_row..=max_row {
            let line = (piece >> (row * 4)) & 0b1111;
            let mut row_str = String::new();
            for col in min_col..=max_col {
                if (line >> (3 - col)) & 1 == 1 {
                    row_str.push_str("🟧");
                } else {
                    row_str.push_str("⬜");
                }
            }
            result.push(row_str);
        }

        result
    }

    fn get_playfield_lines(&self, state: &GameState) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        // Populate the grid cells of the playfield
        for row in 0..20 {
            let row = state.playfield[row];
            result.push(self.render_line(&row))
        }

        // Merge the active piece into playfield
        let (px, py) = (
            state.piece_position[0] as usize,
            state.piece_position[1] as usize,
        );
        let piece_vec = self.render_piece(&state.active_piece);
        for (row_offset, piece_row) in piece_vec.iter().enumerate() {
            let target_row = py + row_offset;
            if target_row >= state.playfield.len() {
                continue; // Avoid out-of-bounds access!
            }
            let mut playfield_row: Vec<char> = result[target_row].chars().collect();
            let piece_chars: Vec<char> = piece_row.chars().collect();
            for (col_offset, &piece_char) in piece_chars.iter().enumerate() {
                let target_col = px + col_offset;
                if target_col >= playfield_row.len() || piece_char == '⬜' {
                    continue;
                }
                playfield_row[target_col] = piece_char;
            }
            result[target_row] = playfield_row.iter().collect();
        }

        result
    }

    pub fn teardown(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        let (cols, rows) = self.init_terminal_size;
        execute!(self.sout, SetSize(cols, rows), Show, LeaveAlternateScreen)?;
        // println!("\x1b[?1049l"); // Leave Alternate Screen Mode
        Ok(())
    }
}
