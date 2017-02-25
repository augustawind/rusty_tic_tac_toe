//! A simple, 2-player Tic Tac Toe game for the console.

#[macro_use] extern crate text_io;

use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum State {
    Blank,
    X,
    O,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Blank => write!(f, "."),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

type Board = Vec<Vec<State>>;

struct Game {
    board: Board,
    size: usize,
    next_move: State,
    move_count: u32,
}

/// Draw the game board.
fn draw_board(board: &Board) {
    println!("");
    for row in board {
        for state in row {
            print!("{}  ", state);
        }
        print!("\n\n");
    }
    println!("");
}

/// Execute a turn of the game, prompting for input and placing an X or O.
/// Returns the coordinates of the move.
fn turn(game: &mut Game) -> (usize, usize) {
    // Announce turn.
    let round = ((game.move_count + 1) as f32 / 2.0).ceil() as u32;
    print!("\n\n");
    println!("      Round {} - {}'s    ", round, game.next_move);
    println!("  ----------------------");

    loop {
        // Draw board.
        draw_board(&game.board);
        // Prompt for move.
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
        if x >= game.size || y >= game.size {
            println!("\nCoordinates out of bounds. Options are 0, 1, 2.");
            continue;
        }

        // Ensure space is empty for move.
        if game.board[y][x] != State::Blank {
            println!("\nSomeone has already moved there!");
            continue;
        }

        // Place board piece and iterate turn.
        game.move_count += 1;

        match game.next_move {
            State::X => {
                game.board[y][x] = State::X;
                game.next_move = State::O;
            },
            State::O => {
                game.board[y][x] = State::O;
                game.next_move = State::X
            },
            State::Blank => panic!("This should never happen."),
        };

        return (x, y);
    }
}

/// Return whether the game is over, plus the winner, or Blank if its a draw.
fn is_game_over(game: &Game, state: State, x: usize, y: usize) -> (bool, State) {
    let n = game.size;

    // Check columns
    for i in 0..n {
        if game.board[i][x] != state {
            break;
        }
        if i == n - 1 {
            return (true, state);
        }
    }
    // Check rows
    for i in 0..n {
        if game.board[y][i] != state {
            break;
        }
        if i == n - 1{
            return (true, state);
        }
    }
    // Check diagonal
    if x == y {
        for i in 0..n {
            if game.board[i][i] != state {
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
            if game.board[i][n - 1 - i] != state {
                break;
            }
            if i == n - 1 {
                return (true, state);
            }
        }
    }

    // Check draw
    if game.move_count == (n * n) as u32 {
        return (true, State::Blank);
    }

    // Game continues
    (false, State::Blank)
}

/// Display the game header.
fn header() {
    println!("");
    println!("===========================");
    println!("=                         =");
    println!("= Welcome to Tic Tac Toe! =");
    println!("=                         =");
    println!("===========================");
    println!("");
}


/// Prompt for game size.
fn prompt_game_size() -> usize {
    let mut size: usize;

    loop {
        println!("\nHow big should the game be?");
        let input: String = read!("{}\n");

        size = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Must be at least 3.");
                continue;
            },
        };

        if size < 3 {
            println!("Must be at least 3.");
            continue;
        }

        return size;
    }
}

/// Prompt for who goes first.
fn prompt_first_move() -> State {
    let first_move: State;

    loop {
        println!("\nWho should go first? X or O");
        let input: String = read!("{}\n");

        first_move = match input.to_lowercase().trim() {
            "x" => State::X,
            "o" => State::O,
            _         => {
                println!("\nPlease enter one of X or O.");
                continue;
            },
        };
        return first_move;
    }
}

/// Draw the final board and announce the winner or draw.
fn conclusion(board: &Board, winner: State) {
    // Display final board
    println!("");
    draw_board(board);
    println!("");
    println!("Game over!");

    // Announce winner or draw
    match winner {
        State::Blank => println!("It's a draw!"),
        _            => println!("{}'s win!", winner),
    }
}

/// Play a game from start to finish.
fn play() {
    header();

    let size = prompt_game_size();
    let first_move = prompt_first_move();

    let board = vec![vec![State::Blank; size]; size];

    let mut game = Game {
        board: board,
        size: size,
        next_move: first_move,
        move_count: 0,
    };

    let (mut game_over, mut winner) = (false, State::Blank);

    // Loop until game over
    while !game_over {
        let last_state = game.next_move;
        let (x, y) = turn(&mut game);
        let result = is_game_over(&game, last_state, x, y);
        game_over = result.0;
        winner = result.1;
    }

    conclusion(&game.board, winner);
}

fn main() {
    play();
}
