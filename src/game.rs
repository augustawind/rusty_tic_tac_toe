//! Core data structures and methods.
use std::fmt;

/// A position on the board.
#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Blank,
    X,
    O,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Blank => write!(f, "-"),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

/// The game board.
type Board = Vec<Vec<State>>;

/// Current game state.
pub struct Game {
    board: Board,
    size: usize,
    next_move: State,
    move_count: u32,
}

impl Game {
    /// Initialize a new Game.
    pub fn new(size: usize, first_move: State) -> Game {
        let board = vec![vec![State::Blank; size]; size];
        Game {
            board: board,
            size: size,
            next_move: first_move,
            move_count: 0,
        }
    }

    /// Run the game, prompting for each move until a winner or draw.
    pub fn run(&mut self) {
        let mut game_over = false;
        let mut winner = State::Blank;

        // Loop until game over
        while !game_over {
            let last_state = self.next_move;
            let (x, y) = self.turn();
            let result = self.is_game_over(last_state, x, y);
            game_over = result.0;
            winner = result.1;
        }

        self.end(winner);
    }

    /// Draw the game board.
    fn draw_board(&self) {
        println!("");
        for row in &self.board {
            for state in row {
                print!("{}  ", state);
            }
            print!("\n\n");
        }
        println!("");
    }

    /// Execute a turn of the game, prompting for input and placing an X or O.
    /// Returns the coordinates of the move.
    fn turn(&mut self) -> (usize, usize) {
        // Announce turn.
        let round = ((self.move_count + 1) as f32 / 2.0).ceil() as u32;
        print!("\n\n");
        println!("      Round {} - {}'s    ", round, self.next_move);
        println!("  ----------------------");

        loop {
            // Prompt for move.
            self.draw_board();
            println!("Where will you move? [x,y]");

            let input: String = read!("{}\n");
            let coords: Vec<&str> = input.split(',').collect();

            // Ensure input is properly formatted.
            if coords.len() != 2 {
                println!("\nPlease enter two digits, separated by a comma.");
                continue;
            }

            // Parse coords as usize.
            let x: usize = match coords[0].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("\nPlease enter two digits, separated by a comma.");
                    continue;
                }
            };
            let y: usize = match coords[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("\nPlease enter two digits, separated by a comma.");
                    continue;
                }
            };

            // Ensure coords are in bounds.
            if x >= self.size || y >= self.size {
                println!("\nCoordinates out of bounds. Options are 0, 1, 2.");
                continue;
            }

            // Ensure space is empty for move.
            if self.board[y][x] != State::Blank {
                println!("\nSomeone has already moved there!");
                continue;
            }

            // Place board piece and iterate turn.
            self.move_count += 1;

            match self.next_move {
                State::X => {
                    self.board[y][x] = State::X;
                    self.next_move = State::O;
                },
                State::O => {
                    self.board[y][x] = State::O;
                    self.next_move = State::X
                },
                State::Blank => panic!("This should never happen."),
            };

            return (x, y);
        }
    }

    /// Return whether the game is over, plus the winner, or Blank if its a draw.
    fn is_game_over(&self, state: State, x: usize, y: usize) -> (bool, State) {
        let n = self.size;

        // Check columns
        for i in 0..n {
            if self.board[i][x] != state {
                break;
            }
            if i == n - 1 {
                return (true, state);
            }
        }
        // Check rows
        for i in 0..n {
            if self.board[y][i] != state {
                break;
            }
            if i == n - 1 {
                return (true, state);
            }
        }
        // Check diagonal
        if x == y {
            for i in 0..n {
                if self.board[i][i] != state {
                    break;
                }
                if i == n - 1 {
                    return (true, state);
                }
            }
        }
        // Check anti-diagonal
        if x + y == n - 1 {
            for i in 0..n {
                if self.board[i][n - 1 - i] != state {
                    break;
                }
                if i == n - 1 {
                    return (true, state);
                }
            }
        }

        // Check draw
        if self.move_count == (n * n) as u32 {
            return (true, State::Blank);
        }

        // Game continues
        (false, State::Blank)
    }

    /// Draw the final board and announce the winner or draw.
    fn end(&self, winner: State) {
        // Display final board
        println!("");
        self.draw_board();
        println!("");
        println!("Game over!");

        // Announce winner or draw
        match winner {
            State::Blank => println!("It's a draw!"),
            _            => println!("{}'s win!", winner),
        }
    }
}
