type Board = [[u32; 3]; 3];

enum Player {
    X,
    O,
}

enum GameState {
    TurnX,
    TurnO,
    GameOver,
}

struct Game {
    board: Board,
    player_x: Player,
    player_o: Player,
    state: GameState,
}

fn draw_board(board: &Board) {
    for row in board {
        for code in row {
            match *code {
                0 => print!("."),
                1 => print!("X"),
                2 => print!("O"),
                _ => panic!("Invalid value for board piece: {}", code),
            }
        }
        println!("");
    }
}

fn main() {
    let mut board = [
        [0, 0, 0],
        [0, 0, 0],
        [0, 0, 0],
    ];
    draw_board(&board);
}
