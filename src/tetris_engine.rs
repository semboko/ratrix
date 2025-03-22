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
    // The `u16` integer encodes a 4x4 Tetris piece (tetromino) using bitwise representation.
    //
    // Interpretation of the `u16` layout:
    //
    //    |r4| |r3| |r2| |r1|
    // 0b ----_----_----_----
    //
    // Each 4-bit group (`rn`) represents a row of the tetromino, starting from the bottom (`r1`) to the top (`r4`).
    //
    // Example: The L-shaped piece in binary representation:
    // 0b_0000_0000_0010_1110
    //
    // This corresponds to the following 4x4 grid:
    //
    //  1110   →  ███░
    //  1000   →  █░░░
    //  0000   →  ░░░░
    //  0000   →  ░░░░
    //
    // Important Constraints:
    // - Each shape should be **aligned to the top-left corner** of the 4x4 matrix.

    // The pieces I've already check is labeled `OK`
    // The rest were produced by ChatGPT

    match (piece, orientation) {
        // T-Piece
        (Tetromino::T, Orientation::N) => 0b_0010_0110_0010_0000,
        (Tetromino::T, Orientation::E) => 0b_0000_0111_0010_0000,
        (Tetromino::T, Orientation::S) => 0b_0010_0110_0010_0000,
        (Tetromino::T, Orientation::W) => 0b_0000_0100_0111_0000,

        // I-Piece
        (Tetromino::I, Orientation::N) => 0b_1000_1000_1000_1000, // OK
        (Tetromino::I, Orientation::E) => 0b_0000_0000_0000_1111, // OK
        (Tetromino::I, Orientation::S) => 0b_1000_1000_1000_1000, // OK
        (Tetromino::I, Orientation::W) => 0b_0000_0000_0000_1111, // OK

        // O-Piece (always the same)
        (Tetromino::O, _) => 0b_0000_0000_1100_1100, // OK

        // L-Piece
        (Tetromino::L, Orientation::N) => 0b_0000_0000_1000_1110, // OK
        (Tetromino::L, Orientation::E) => 0b_0000_1100_1000_1000, // OK
        (Tetromino::L, Orientation::S) => 0b_0000_0000_1110_0010, // OK
        (Tetromino::L, Orientation::W) => 0b_0000_0100_0100_1100, // OK

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
        let piece_row = (piece >> i * 4) * 0xf;
        if piece_row > 0 {
            result += 1;
        }
    }
    result
}

fn get_piece_width(piece: &u16) -> u8 {
    let mut max_width = 0;

    // Iterate through each of the 4 rows
    for i in 0..4 {
        let row = (piece >> (i * 4)) & 0b1111; // Extract the 4-bit row

        let mut leftmost = 4; // Leftmost occupied column (initialize to max)
        let mut rightmost = 0; // Rightmost occupied column

        for j in 0..4 {
            if (row >> j) & 1 == 1 {
                leftmost = leftmost.min(j);
                rightmost = rightmost.max(j);
            }
        }

        if leftmost < 4 {
            // If there is at least one occupied tile in this row
            max_width = max_width.max(rightmost - leftmost + 1);
        }
    }

    max_width as u8
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
            let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
            if get_piece_width(&piece) + new_x <= 10 {
                self.piece_position[0] = new_x;
            }
        }

        if let Some(new_y) = (self.piece_position[1] as isize + dy).try_into().ok() {
            self.piece_position[1] = new_y;
        }

        self.changed = true;
    }

    fn get_positioned_piece_row(&self, piece: &u16, i: &u8) -> u16 {
        // Extracts the i-th row from a piece and position it into
        // a 10 bit wide row in accordance with its current x-position.

        // Extracting i-th the row
        let mut piece_row = (piece >> (i * 4)) & 0xf;
        // Removing the extra zeroes from the right
        let width = get_piece_width(&piece);
        piece_row >>= 4 - width;
        // Shifting to the leftmost position
        piece_row <<= 10 - width;
        // Shifting to the action x position
        piece_row >>= self.piece_position[0];
        piece_row
    }

    fn can_move_down(&self) -> bool {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        let piece_height = get_piece_height(&piece);
        if (self.piece_position[1] + piece_height) > 19 {
            return false;
        }
        for i in 0..4 {
            let piece_row = self.get_positioned_piece_row(&piece, &i);
            let target_y = (self.piece_position[1] + i + 1) as usize;
            if target_y > 19 {
                break;
            }
            let playfield_row = self.playfield[target_y];
            if (piece_row & playfield_row) != 0 {
                return false;
            }
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

        if self.can_move_down() {
            self.piece_position[1] += 1;
        } else {
            self.lock_active_piece();
            // TODO: Generate a new active_piece
            self.piece_position = [4, 0];
        }

        self.last_update = current_time;
        self.changed = true;
    }

    fn lock_active_piece(&mut self) {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        for i in 0..4 {
            let shift_x = 7 - self.piece_position[0];
            let piece_row = (((piece >> (i * 4)) & 0xf) << shift_x) >> 1;
            if piece_row == 0 {
                continue;
            }
            let target_y = (self.piece_position[1] + i) as usize;
            self.playfield[target_y] |= piece_row;
        }
    }

    fn lock_tile(&mut self, x: usize, y: usize) {
        self.playfield[y] = (1 << (9 - x)) | self.playfield[y];
        self.changed = true;
    }

    pub fn rotate(&mut self) {
        match self.piece_orientation {
            Orientation::N => self.piece_orientation = Orientation::E,
            Orientation::E => self.piece_orientation = Orientation::S,
            Orientation::S => self.piece_orientation = Orientation::W,
            Orientation::W => self.piece_orientation = Orientation::N,
        }
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
    fn board_lock_tile() {
        let mut tetris = TetrisEngine::new();
        tetris.lock_tile(0, 0);
        let board_str = tetris.get_lines();
        assert_eq!(board_str[0], "🟧⬜⬜⬜🟧🟧🟧⬜⬜⬜");
        assert_eq!(board_str[1], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");
        // The rest of the lines should be empty
        for i in 3..20 {
            assert_eq!(board_str[i], "⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
        }
    }

    #[test]
    fn change_is_true_when_piece_lock_happened() {
        let mut tetris = TetrisEngine::new();
        tetris.lock_tile(0, 0);
        assert_eq!(tetris.changed, true);
    }

    #[test]
    fn engine_can_be_updated() {
        let mut tetris = TetrisEngine::new();
        tetris.update();
        // The initial update shouldn't change the position of the active piece,
        // since not enough time elapsed from the 'last_update'
        assert_eq!(tetris.piece_position[1], 0);
        // Mock the scenario when 1 second elapsed from the previous step
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

    #[test]
    fn can_lock_at_some_random_position() {
        let mut tetris = TetrisEngine::new();
        tetris.piece_position = [3, 18];
        tetris.lock_active_piece();
        assert_eq!(tetris.playfield[18], 0b0001110000); // ░░░███░░░░
        assert_eq!(tetris.playfield[19], 0b0001000000); // ░░░█░░░░░░
    }

    #[test]
    fn piece_is_locked_at_the_extreme_right() {
        let mut tetris = TetrisEngine::new();
        tetris.piece_position = [7, 18];
        tetris.lock_active_piece();
        assert_eq!(tetris.playfield[18], 0b0000000111); // ░░░░░░░███
        assert_eq!(tetris.playfield[19], 0b0000000100); // ░░░░░░░█░░
    }

    #[test]
    fn update_should_lock_the_piece_in_the_bottom() {
        let mut tetris = TetrisEngine::new();
        tetris.piece_position = [7, 18];
        tetris.last_update -= 1.0;
        tetris.update();
        assert_eq!(tetris.playfield[18], 0b0000000111); // ░░░░░░░███
        assert_eq!(tetris.playfield[19], 0b0000000100); // ░░░░░░░█░░
    }

    #[test]
    fn can_not_move_down_if_piece_under() {
        // The following scenario is tested:
        // 16 ░░░░░░░▒▒▒  → ▒ Active piece is the L-shape on the line 16
        // 17 ░░░░░░░▒░░
        // 18 ░░░░░░░███  → █ There are locked tiles in the playfield under the active piece
        // 19 ░░░░░░░█░░
        // Active piece CAN'T move down 🚫

        let mut tetris = TetrisEngine::new();
        tetris.lock_tile(7, 18);
        tetris.lock_tile(8, 18);
        tetris.lock_tile(9, 18);
        tetris.lock_tile(7, 19);
        tetris.piece_position = [7, 16];
        assert_eq!(tetris.can_move_down(), false);
    }
    #[test]
    fn can_move_down_tricky_case_1() {
        // The following scenario is tested:
        // 16 ░░░░░░░▒▒▒  → ▒ Active piece is the L-shape on the line 16
        // 17 ░░░░░░░▒░░
        // 18 ░░░░░░░░██  → █ There are locked tiles in the playfield under the active piece.
        // 19 ░░░░░░░██░
        // Active piece CAN move down 👍

        let mut tetris = TetrisEngine::new();
        tetris.lock_tile(8, 18);
        tetris.lock_tile(9, 18);
        tetris.lock_tile(7, 19);
        tetris.lock_tile(8, 19);
        tetris.piece_position = [7, 16];
        assert_eq!(tetris.can_move_down(), true);
    }

    #[test]
    fn can_move_down_tricky_case_2() {
        // The following scenario is tested:
        // 17 ▒░░░░░░░░░  → The E-oriented L-shape is positioned on the line 17
        // 18 ▒░░░░░░░░░    and it's about to make an invalid move down
        // 19 ▒▒░░░░░░░░
        // Active piece CAN'T move down 🚫

        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [0, 17];
        assert_eq!(tetris.can_move_down(), false);
    }

    #[test]
    fn piece_height_is_correctly_calculated() {
        let north_l = get_tetromino_representation(&Tetromino::L, &Orientation::N);
        let east_l = get_tetromino_representation(&Tetromino::L, &Orientation::E);
        assert_eq!(get_piece_height(&north_l), 2);
        assert_eq!(get_piece_height(&east_l), 3);
    }

    #[test]
    fn piece_width_is_correctly_calculated() {
        let north_l = get_tetromino_representation(&Tetromino::L, &Orientation::N);
        let east_l = get_tetromino_representation(&Tetromino::L, &Orientation::E);
        assert_eq!(get_piece_width(&north_l), 3);
        assert_eq!(get_piece_width(&east_l), 2);
    }

    #[test]
    fn rotation_works() {
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        assert_eq!(tetris.changed, true); // Changed!
        let lines = tetris.get_lines();
        assert_eq!(lines[0], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");
        assert_eq!(lines[1], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");
        assert_eq!(lines[2], "⬜⬜⬜⬜🟧🟧⬜⬜⬜⬜");
        tetris.rotate();
        assert_eq!(tetris.changed, true);
        let lines = tetris.get_lines();
        assert_eq!(lines[0], "⬜⬜⬜⬜⬜⬜🟧⬜⬜⬜");
        assert_eq!(lines[1], "⬜⬜⬜⬜🟧🟧🟧⬜⬜⬜");
        assert_eq!(lines[2], "⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
        tetris.rotate();
        assert_eq!(tetris.changed, true);
        let lines = tetris.get_lines();
        assert_eq!(lines[0], "⬜⬜⬜⬜🟧🟧⬜⬜⬜⬜");
        assert_eq!(lines[1], "⬜⬜⬜⬜⬜🟧⬜⬜⬜⬜");
        assert_eq!(lines[2], "⬜⬜⬜⬜⬜🟧⬜⬜⬜⬜");
        tetris.rotate();
        assert_eq!(tetris.changed, true);
        let lines = tetris.get_lines();
        assert_eq!(lines[0], "⬜⬜⬜⬜🟧🟧🟧⬜⬜⬜");
        assert_eq!(lines[1], "⬜⬜⬜⬜🟧⬜⬜⬜⬜⬜");
        assert_eq!(lines[2], "⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜");
    }

    #[test]
    fn move_right_from_the_extreme_right_position() {
        let mut tetris = TetrisEngine::new();
        tetris.piece_position = [7, 0];
        tetris.move_current_shape(1, 0);
        assert_eq!(tetris.piece_position, [7, 0]);
        tetris.last_update -= 1.0;
        tetris.update(); // Update shouldn't crash the game
    }

    #[test]
    fn rightmost_position_of_2tile_wide_piece_doesnt_crash_game() {
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [8, 0];
        tetris.can_move_down();
    }

    #[test]
    fn aligned_row_with_piece() {
        let mut tetris = TetrisEngine::new();
        let piece = &get_tetromino_representation(&Tetromino::L, &Orientation::N);
        tetris.piece_position[0] = 2;
        let row = tetris.get_positioned_piece_row(&piece, &0);
        assert_eq!(row, 0b0011100000);
        let row = tetris.get_positioned_piece_row(&piece, &1);
        assert_eq!(row, 0b0010000000);
    }

    #[test]
    fn can_move_down_tricky_case_3() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░░█░░░░░
        // 15 ░░░░█░░░░░
        // 16 ░░░░██░░░░
        // 17 ░░░▓░░░░░░
        // 18 ░░░▓░░░░░░
        // 19 ░░░▓▓░░░░░
        // The upper piece should be able to move down 👍
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [3, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [4, 14];
        assert_eq!(tetris.can_move_down(), true);
    }
}
