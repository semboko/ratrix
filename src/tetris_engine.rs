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
        (Tetromino::T, Orientation::N) => 0b_0000_0000_1110_0100,
        (Tetromino::T, Orientation::E) => 0b_0000_1000_1100_1000,
        (Tetromino::T, Orientation::S) => 0b_0000_0000_0100_1110,
        (Tetromino::T, Orientation::W) => 0b_0000_0100_1100_0100,

        // I-Piece
        (Tetromino::I, Orientation::N) => 0b_1000_1000_1000_1000,
        (Tetromino::I, Orientation::E) => 0b_0000_0000_0000_1111,
        (Tetromino::I, Orientation::S) => 0b_1000_1000_1000_1000,
        (Tetromino::I, Orientation::W) => 0b_0000_0000_0000_1111,

        // O-Piece (always the same)
        (Tetromino::O, _) => 0b_0000_0000_1100_1100,

        // L-Piece
        (Tetromino::L, Orientation::N) => 0b_0000_0000_1000_1110,
        (Tetromino::L, Orientation::E) => 0b_0000_1100_1000_1000,
        (Tetromino::L, Orientation::S) => 0b_0000_0000_1110_0010,
        (Tetromino::L, Orientation::W) => 0b_0000_0100_0100_1100,

        // J-Piece
        (Tetromino::J, Orientation::N) => 0b_0000_0111_0001_0000,
        (Tetromino::J, Orientation::E) => 0b_0000_0011_0010_0010,
        (Tetromino::J, Orientation::S) => 0b_0000_0100_0111_0000,
        (Tetromino::J, Orientation::W) => 0b_0000_0010_0010_0110,

        // S-Piece
        (Tetromino::S, Orientation::N) => 0b_0000_0000_1100_0110,
        (Tetromino::S, Orientation::E) => 0b_0000_0100_1100_1000,
        (Tetromino::S, Orientation::S) => 0b_0000_0000_1100_0110,
        (Tetromino::S, Orientation::W) => 0b_0000_0100_1100_1000,

        // Z-Piece
        (Tetromino::Z, Orientation::N) => 0b_0000_0000_0110_1100,
        (Tetromino::Z, Orientation::E) => 0b_0000_1000_1100_0100,
        (Tetromino::Z, Orientation::S) => 0b_0000_0000_0110_1100,
        (Tetromino::Z, Orientation::W) => 0b_0000_1000_1100_0100,
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
        let piece_row = (piece >> i * 4) & 0xf;
        if piece_row > 0 {
            result += 1;
        }
    }
    result
}

fn get_positioned_piece_row(piece: &u16, i: &u8, x: &u8) -> u16 {
    // Extracts the i-th row from a piece and position it into
    // a 10 bit wide row in accordance with the specified (x, y) position.

    // Extracting i-th the row
    let mut piece_row = (piece >> (i * 4)) & 0xf;
    // Removing the extra zeroes from the right
    // let width = get_piece_width(&piece);
    // piece_row >>= 4 - width;
    // // Shifting to the leftmost position
    // piece_row <<= 10 - width;
    // // Shifting to the action x position
    // piece_row >>= x;
    piece_row <<= 6; // Move it to the X=0 coordinate
    piece_row >>= x;
    piece_row
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

// DTO which is used to transfer the data into the renderer.
pub struct GameState {
    pub playfield: [u16; 20],
    pub piece_position: [u8; 2],
    pub active_piece: u16,
    pub score: usize,
}

pub struct TetrisEngine {
    playfield: [u16; 20],
    piece_position: [u8; 2],
    piece_orientation: Orientation,
    active_piece: Tetromino,
    pub changed: bool,
    last_update: f64,
    score: usize,
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
            score: 0,
        };
    }

    pub fn generate_random_piece(&mut self) {
        let idx: usize = rand::random_range(0..7);
        self.active_piece = match idx {
            0 => Tetromino::I,
            1 => Tetromino::J,
            2 => Tetromino::L,
            3 => Tetromino::O,
            4 => Tetromino::S,
            5 => Tetromino::T,
            6 => Tetromino::Z,
            _ => panic!("No such type of tetromino found"),
        }
    }

    pub fn move_current_shape(&mut self, dx: isize, dy: isize) {
        if let Some(new_x) = (self.piece_position[0] as isize + dx).try_into().ok() {
            let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
            let valid_move = get_piece_width(&piece) + new_x <= 10;
            let valid_move =
                valid_move && !self.overlaps_locked_pieces(&new_x, &self.piece_position[1]);
            if valid_move {
                self.piece_position[0] = new_x;
            }
        }

        if let Some(new_y) = (self.piece_position[1] as isize + dy).try_into().ok() {
            if self.can_move_down() {
                self.piece_position[1] = new_y;
            }
        }

        self.changed = true;
    }

    fn can_move_down(&self) -> bool {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        let piece_height = get_piece_height(&piece);
        if (self.piece_position[1] + piece_height) > 19 {
            return false;
        }
        if self.overlaps_locked_pieces(&self.piece_position[0], &(self.piece_position[1] + 1)) {
            return false;
        };
        true
    }

    fn overlaps_locked_pieces(&self, x: &u8, y: &u8) -> bool {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        for i in 0..4 {
            let piece_row = get_positioned_piece_row(&piece, &i, x);
            let target_y = (y + i) as usize;
            if target_y > 19 {
                break;
            };
            let playfield_row = self.playfield[target_y];
            if (piece_row & playfield_row) != 0 {
                return true;
            }
        }
        false
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
            self.piece_position = [4, 0];
            self.generate_random_piece();
        }

        self.apply_gravity();
        self.last_update = current_time;
        self.changed = true;
    }

    fn lock_active_piece(&mut self) {
        let piece = get_tetromino_representation(&self.active_piece, &self.piece_orientation);
        for i in 0..4 {
            let piece_row = get_positioned_piece_row(&piece, &i, &self.piece_position[0]);
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

    fn clear_line(&mut self, i: usize) {
        for j in (1..i + 1).rev() {
            self.playfield[j] = self.playfield[j - 1];
        }
        self.playfield[0] = 0;
    }

    fn apply_gravity(&mut self) {
        for i in 0..20 {
            if self.playfield[i] == 0b1111111111 {
                self.clear_line(i);
            }
        }
    }

    pub fn get_state(&self) -> GameState {
        GameState {
            playfield: self.playfield,
            piece_position: self.piece_position,
            active_piece: get_tetromino_representation(&self.active_piece, &self.piece_orientation),
            score: self.score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let row = get_positioned_piece_row(&piece, &0, &2);
        assert_eq!(row, 0b0011100000);
        let row = get_positioned_piece_row(&piece, &1, &2);
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

    #[test]
    fn piece_should_not_be_moved_down_if_not_possible() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░█░░░░░░
        // 15 ░░░█░░░░░░
        // 16 ░░░██░░░░░
        // 17 ░░░▓░░░░░░
        // 18 ░░░▓░░░░░░
        // 19 ░░░▓▓░░░░░
        // The upper piece is about to be locked in the next update.
        // The engine should ignore soft drop.
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [3, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [3, 14];
        tetris.move_current_shape(0, 1); // Soft drop
        assert_eq!(tetris.piece_position[1], 14) // Y-position of the piece shouldn't change
    }

    #[test]
    fn piece_coudnt_move_right_into_locked_pieces() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░░░░░░░░
        // 15 ░█░░░░░░░░ The upper L-shappe
        // 16 ░█░░░░░░░░
        // 17 ░██▓░░░░░░ ..is about to move into the locked tiles
        // 18 ░░░▓░░░░░░
        // 19 ░░░▓▓░░░░░
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [3, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [1, 15];
        tetris.move_current_shape(1, 0); // Right move
        assert_eq!(tetris.piece_position[0], 1); // The move doesn't affect the position
    }

    #[test]
    fn piece_coudnt_move_left_into_locked_pieces() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░░░░░░░░
        // 15 ░░░░█░░░░░ The upper L-shappe
        // 16 ░░░░█░░░░░
        // 17 ░░░▓██░░░░ ..is about to move into the locked tiles
        // 18 ░░░▓░░░░░░
        // 19 ░░░▓▓░░░░░
        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [3, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [4, 15];
        tetris.move_current_shape(-1, 0); // Left move
        assert_eq!(tetris.piece_position[0], 4); // The move doesn't affect the position
    }

    #[test]
    fn update_makes_the_filled_rows_disapear() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░░░░░░░░
        // 15 ░░░░░░░░░░
        // 16 ░░░░░░░░░░
        // 17 ▓░▓░▓░▓░█░
        // 18 ▓░▓░▓░▓░█░
        // 19 ▓▓▓▓▓▓▓▓██ <- The last piece adding up to the row

        let mut tetris = TetrisEngine::new();
        tetris.rotate();
        tetris.piece_position = [0, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [2, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [4, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [6, 17];
        tetris.lock_active_piece();
        tetris.piece_position = [8, 17];
        tetris.last_update -= 1.0;
        tetris.update();
        assert_eq!(tetris.playfield[19], 0b1010101010);
        assert_eq!(tetris.playfield[18], 0b1010101010);
        assert_eq!(tetris.playfield[17], 0b0000000000);
    }

    #[test]
    fn update_removes_rows_tricky_1() {
        // The case we are handling:
        //    0123456789
        // 14 ░░░░░░░░░░
        // 15 ░░░░░░░░░░
        // 16 ░░░░░░░░░░
        // 17 ███░░░░░░░
        // 18 █▓▓▓▓▓▓▓▓▓ <- Only line 18 must be removed
        // 19 ░░▓▓▓▓▓▓▓▓

        // And that what should left after all:
        // 17 ░░░░░░░░░░
        // 18 ▓▓▓░░░░░░░ <- The part from the line 17
        // 19 ░░▓▓▓▓▓▓▓▓ <- The bottom line is untouched

        let mut tetris = TetrisEngine::new();
        for i in 1..10 {
            tetris.lock_tile(i, 18);
        }
        for i in 2..10 {
            tetris.lock_tile(i, 19);
        }
        tetris.piece_position = [0, 17];
        tetris.last_update -= 1.0;
        tetris.update();
        assert_eq!(tetris.playfield[18], 0b1110000000);
        assert_eq!(tetris.playfield[19], 0b0011111111)
    }

    #[test]
    fn ipiece_is_able_to_appear_on_the_playfield() {
        let mut tetris = TetrisEngine::new();
        tetris.active_piece = Tetromino::I;
        tetris.last_update -= 1.0;
        tetris.update();
    }
}
