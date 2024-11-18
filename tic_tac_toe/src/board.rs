pub struct Board {
    cells: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[' '; 3]; 3],
        }
    }

    pub fn display(&self) {
        for row in &self.cells {
            println!(" {} | {} | {} ", row[0], row[1], row[2]);
            println!("---|---|---");
        }
    }

    pub fn make_move(&mut self, row: usize, col: usize, player: char) -> bool {
        if self.cells[row][col] == ' ' {
            self.cells[row][col] = player;
            true
        } else {
            false
        }
    }

    pub fn check_winner(&self) -> Option<char> {
        for i in 0..3 {
            // check rows and columns
            if self.cells[i][0] != ' ' && self.cells[i][0] == self.cells[i][1] && self.cells[i][1] == self.cells[i][2] {
                return Some(self.cells[i][0]);
            }
            // this is the column check
            if self.cells[0][i] != ' ' && self.cells[0][i] == self.cells[1][i] && self.cells[1][i] == self.cells[2][i] {
                return Some(self.cells[0][i]);
            }
        }
        // check diagonals
        if self.cells[0][0] != ' ' && self.cells[0][0] == self.cells[1][1] && self.cells[1][1] == self.cells[2][2] {
            return Some(self.cells[0][0])
        }
        if self.cells[0][2] != ' ' && self.cells[0][2] == self.cells[1][1] && self.cells[1][1] == self.cells[2][0] {
            return Some(self.cells[0][2])
        }
        None
    }
}