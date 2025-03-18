enum Tetromino {
    T,
    I,
    O,
    L,
    J,
    S,
    Z,
}

enum Orientation {
    N,
    E,
    S,
    W,
}

fn get_tetromino_representation(piece: &Tetromino, orientation: &Orientation) -> u16 {
    match (piece, orientation) {
        // T-Piece
        (Tetromino::T, Orientation::N) => 0b_0010_0110_0010_0000,
        (Tetromino::T, Orientation::E) => 0b_0000_0111_0010_0000,
        (Tetromino::T, Orientation::S) => 0b_0010_0110_0010_0000,
        (Tetromino::T, Orientation::W) => 0b_0000_0100_0111_0000,

        // I-Piece
        (Tetromino::I, Orientation::N) => 0b_0000_1111_0000_0000,
        (Tetromino::I, Orientation::E) => 0b_0010_0010_0010_0010,
        (Tetromino::I, Orientation::S) => 0b_0000_1111_0000_0000,
        (Tetromino::I, Orientation::W) => 0b_0010_0010_0010_0010,

        // O-Piece (always the same)
        (Tetromino::O, _) => 0b_0000_0110_0110_0000,

        // L-Piece
        (Tetromino::L, Orientation::N) => 0b_0000_0001_0111_0000,
        (Tetromino::L, Orientation::E) => 0b_0010_0010_0011_0000,
        (Tetromino::L, Orientation::S) => 0b_0000_0010_0111_0000,
        (Tetromino::L, Orientation::W) => 0b_0000_0110_0010_0010,

        // J-Piece
        (Tetromino::J, Orientation::N) => 0b_0000_0111_0001_0000,
        (Tetromino::J, Orientation::E) => 0b_0000_0011_0010_0010,
        (Tetromino::J, Orientation::S) => 0b_0000_0100_0111_0000,
        (Tetromino::J, Orientation::W) => 0b_0000_0010_0010_0110,

        // S-Piece
        (Tetromino::S, Orientation::N) => 0b_0000_0110_0011_0000,
        (Tetromino::S, Orientation::E) => 0b_0000_0010_0110_0100,
        (Tetromino::S, Orientation::S) => 0b_0000_0110_0011_0000,
        (Tetromino::S, Orientation::W) => 0b_0000_0010_0110_0100,

        // Z-Piece
        (Tetromino::Z, Orientation::N) => 0b_0000_0011_0110_0000,
        (Tetromino::Z, Orientation::E) => 0b_0000_0100_0110_0010,
        (Tetromino::Z, Orientation::S) => 0b_0000_0011_0110_0000,
        (Tetromino::Z, Orientation::W) => 0b_0000_0100_0110_0010,
    }
}

pub struct TetrisEngine {
    playfield: [u16; 20],
    piece_position: [u8; 2],
    piece_orientation: Orientation,
    active_piece: Tetromino,
    pub changed: bool,
}

impl TetrisEngine {
    pub fn new() -> Self {
        return Self {
            playfield: [0; 20],
            piece_position: [4, 0], // TODO: Should be different for every tetramino!
            changed: true,
            active_piece: Tetromino::L, // TODO: Should be a random tetramino!
            piece_orientation: Orientation::N,
        };
    }

    // Converts a 10-bit encoded integer into a visual representation of tiles using emoji.
    // Each bit represents a tile: 1 (🟧) is a filled tile, 0 (⬜) is an empty tile.
    fn to_string(&self, line: &u16) -> String {
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

    fn piece_to_vec(&self, piece: &u16) -> Vec<String> {
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

    pub fn move_current_shape(&mut self, dx: isize, dy: isize) {
        if let Some(new_x) = (self.piece_position[0] as isize + dx).try_into().ok() {
            self.piece_position[0] = new_x;
        }

        if let Some(new_y) = (self.piece_position[1] as isize + dy).try_into().ok() {
            self.piece_position[1] = new_y;
        }

        self.changed = true;
    }

    pub fn blit_tile(&mut self, x: usize, y: usize) {
        self.playfield[y] = (1 << (9 - x)) | self.playfield[y];
        self.changed = true;
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];

        // Populate the grid cells of the playfield
        for row in 0..20 {
            let row = self.playfield[row];
            result.push(self.to_string(&row))
        }

        // Merge the active piece into playfield
        let (px, py) = (
            self.piece_position[0] as usize,
            self.piece_position[1] as usize,
        );
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        let piece_vec = self.piece_to_vec(&piece);
        for (row_offset, piece_row) in piece_vec.iter().enumerate() {
            let target_row = py + row_offset;
            if target_row >= self.playfield.len() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_board_state() {
        let tetris = TetrisEngine::new();
        let board_str = tetris.get_lines();
        // The first 3 lines must contain the active_piece
        // TODO: Currently it's always an L piece, but it should be a random piece in the future
        assert_eq!(board_str[0], "⬜⬜⬜⬜🟧🟧🟧⬜⬜⬜");
        assert_eq!(board_str[1], "⬜⬜⬜⬜⬜⬜🟧⬜⬜⬜");

        // The rest of the lines should be empty
        for i in 2..20 {
            assert_eq!(board_str[i], "⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
        }
    }

    #[test]
    fn board_blit_tile() {
        let mut tetris = TetrisEngine::new();
        tetris.blit_tile(0, 0);
        let board_str = tetris.get_lines();
        assert_eq!(board_str[0], "🟧⬜⬜⬜🟧🟧🟧⬜⬜⬜");
        assert_eq!(board_str[1], "⬜⬜⬜⬜⬜⬜🟧⬜⬜⬜");
        // The rest of the lines should be empty
        for i in 3..20 {
            assert_eq!(board_str[i], "⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
        }
    }

    #[test]
    fn change_is_true_when_blit_happened() {
        let mut tetris = TetrisEngine::new();
        tetris.blit_tile(0, 0);
        assert_eq!(tetris.changed, true);
    }
}
