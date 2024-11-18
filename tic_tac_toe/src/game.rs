use crate::board::Board;
// crate: Refers to the root module of the current package (i.e., the Rust project
// you’re working on). crate refers to the same package, starting at the root
use crate::player::Player;
use std::io;

pub struct Game {
    board: Board,
    players: [Player; 2],
    current_turn: usize
    // usize is a primitive type in Rust used for indexing and size-related operations.
    // usize is an unsigned integer (cannot be negative).
    // It adjusts its size to match the platform’s architecture:
    // On a 64-bit machine: 64 bits.
    // On a 32-bit machine: 32 bits
    // usize is commonly used for indexing collections like arrays or vectors because it matches
    // the size of the machine's memory pointers
}

impl Game {
    pub fn new() -> Self {
        let player1 = Player::new(String::from("Player 1"), 'X');
        let player2 = Player::new(String::from("Player 2"), 'O');

        Self {
            board: Board::new(),
            players: [player1, player2],
            current_turn: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            // loop is an infinite loop in Rust. It is similar to a while loop but does not require
            // a condition.
            self.board.display();

            // The & creates a reference to the value, meaning:
            //
            // current_player becomes a reference to self.players[self.current_turn] rather than a
            // copy of it. This avoids copying the value (efficient for large data structures)
            // and allows you to read the value without taking ownership.
            let current_player = &self.players[self.current_turn];
            println!("{}'s turn ({}):", current_player.name, current_player.marker);

            let (row, col) = self.get_move();

            if self.board.make_move(row, col, current_player.marker) {
                if let Some(winner) = self.board.check_winner() {
                    self.board.display();
                    println!("{} ({}) wins!", current_player.name, winner);
                    break;
                }
                self.current_turn = 1 - self.current_turn; // Toggle between 1 and 0
            } else {
                println!("Invalid move. Try again!");
            }
        }
    }

    // The &self means the method borrows the struct (self) as an immutable reference.
    // The method can read self’s fields but cannot modify them.
    // Why? You want to access the game state to get a move but don’t need to change anything.
    fn get_move(&self) -> (usize, usize) {
        loop {
            println!("Enter row and column. Enter two numbers with a space in between: ");
            let mut input = String::new();
            // unwrap() is used to extract the value from a Result or Option type.
            // Rust enforces that you can have only one mutable reference at a time to prevent
            // data races.
            io::stdin().read_line(&mut input).unwrap();
            // Yes, (row, col) is a tuple. Tuples are ordered collections of values
            // with potentially different types.
            // Why the inner parentheses ((row, col))? We are Destructuring the Option into its
            // contained tuple value.
            if let Some((row, col)) = Self::parse_input(&input) {
                println!("Player entered row: {}, col: {}", row, col);
                if row < 3 && col < 3 {
                    return (row, col);
                }
            }
            else {
                println!("Invalid input. Try again!");
            }

        }
    }

    fn parse_input(input: &str) -> Option<(usize, usize)> {
        // A Vec is a growable array in Rust.
        // &str represents a string slice, which is a reference to part of a string.
        let nums: Vec<&str> = input.trim().split_whitespace().collect();
        if nums.len() == 2 {
            if let (Ok(row), Ok(col)) = (nums[0].parse(), nums[1].parse()) {
                return Some((row, col));
            }
        }
        None
    }
}