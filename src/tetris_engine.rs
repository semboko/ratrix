
use primitive_types::U256;

pub struct TetrisEngine {
    board: [u16; 20],
    pos: [u8; 2],
}

impl TetrisEngine {
    pub fn new() -> Self {
        return Self {
            board: [0; 20],
            pos: [5, 0],
        };
    }

    fn to_string(&self, line: &u16) -> String {
        let mut result = String::from("");
        for i in (0..10).rev() {
            if (line >> i & 1) == 1 {
                result.push_str("#");
            } else {
                result.push_str(" ");
            }
        }
        result
    }

    pub fn blit_tile(&mut self, x: usize, y: usize) {
        self.board[y] = (1 << (9 - x)) | self.board[y];
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for row in 0..20 {
            let row = self.board[row];
            result.push(self.to_string(&row))
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_str() {
        let tetris = TetrisEngine::new();
        let board_str = tetris.get_lines();
        for i in 0..20 {
            assert_eq!(board_str[i], "          ");
        }
    }

    #[test]
    fn board_blit_tile() {
        let mut tetris = TetrisEngine::new();
        tetris.blit_tile(0, 0);
        let board_str = tetris.get_lines();
        assert_eq!(board_str[0].as_bytes()[0], b'#');
    }
}
