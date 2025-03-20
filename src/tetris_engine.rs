use std::time::{SystemTime, UNIX_EPOCH};

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

    // The u16 integers should be interpreted as follows
    // 
    //   |r4| |r3| |r2| |r1|
    // 0b----_----_----_----
    //
    // where rn is the nth row of the shape.
    // 
    // For example, the bytes of the L shape: 
    // 0b_0000_0000_0010_1110
    // can be converted into this matrix:
    // 1110  which is : ███░
    // 1000             █░░░
    // 0000             ░░░░
    // 0000             ░░░░

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
        (Tetromino::L, Orientation::N) => 0b_0000_1000_1110_0000,  // OK  The only pieces, which I tested, 
        (Tetromino::L, Orientation::E) => 0b_0000_1100_1000_1000,  // OK  the rest are produced by GPT
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

fn get_current_time() -> f64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => return n.as_secs_f64(),
        Err(_) => panic!("System time error"),
    }
}

fn get_piece_height(piece: &u16) -> u8 {
    let mut result: u8 = 0;
    for i in 0..4 {
        let piece_row = (piece >> i) * 0xf;
        if piece_row > 0 {
            result += 1;
        }
    }
    result
}

pub struct TetrisEngine {
    playfield: [u16; 20],
    piece_position: [u8; 2],
    piece_orientation: Orientation,
    active_piece: Tetromino,
    pub changed: bool,
    last_update: f64,
}

impl TetrisEngine {
    pub fn new() -> Self {
        return Self {
            playfield: [0; 20],
            piece_position: [4, 0], // TODO: The initial position should be different for every tetramino!
            changed: true,
            active_piece: Tetromino::L,
            piece_orientation: Orientation::N,
            last_update: get_current_time(),
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

    fn can_move_down(&self) -> bool {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        let piece_height = get_piece_height(&piece);
        if (self.piece_position[1] + piece_height) > 21 {
            return false;
        }
        true
    }

    pub fn update(&mut self) {
        // TODO: the idle time actually depends on the speed, but it's not added yet
        let idle_time = 0.5;
        let current_time = get_current_time();
        if current_time < self.last_update + idle_time {
            // Not enough time elapsed from the previous update
            return;
        }

        if self.can_move_down(){
            self.piece_position[1] += 1;
        } else {
            // TODO: 1. Merge the active_piece into the playfield
            // TODO: 2. Generate a new active_piece
            // TODO: 3. Set new position of the active_piece
        }

        self.last_update = current_time;
        self.changed = true;
    }

    pub fn blit_tile(&mut self, x: usize, y: usize) {
        self.playfield[y] = (1 << (9 - x)) | self.playfield[y];
        self.changed = true;
    }

    // TODO: This is the part of the renderer layer
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
        assert_eq!(board_str[1], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");

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
        assert_eq!(board_str[1], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");
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

    #[test]
    fn engine_can_be_updated() {
        let mut tetris = TetrisEngine::new();
        tetris.update();
        // The initial update shouldn't change the position of the active piece,
        // since not enough time elapsed from the 'last_update'
        assert_eq!(tetris.piece_position[1], 0);
        // Mock the scenario that 1 second elapsed from the previous step
        tetris.last_update -= 1.0;
        // The update should affect the y position of the piece now!
        tetris.update();
        assert_eq!(tetris.piece_position[1], 1);
        assert_eq!(tetris.changed, true);
    }

    #[test]
    fn can_move_down_returns_true_on_empty_playfield() {
        let tetris = TetrisEngine::new();
        assert!(tetris.can_move_down());
    }

    #[test]
    fn can_move_down_returns_false_on_the_bottom() {
        let mut tetris = TetrisEngine::new();
        tetris.piece_position[1] = 18;
        // The next move will cause the shape to be under the playfield
        assert!(!tetris.can_move_down());
    }
}
