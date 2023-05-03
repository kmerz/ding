use std::process::exit;

use chess::{Game, Square, ChessMove, Board};

mod ui;

fn main() {
    let mut game = Game::new();

    while game.result().is_none() {
        ui::print_board(&game.current_position());

        print!("> ");
        let mut input = ui::read_str();
        if input.eq("quit!") {
            exit(0);
        }
        let nextMove = ChessMove::from_san(&game.current_position(), &input);
        if nextMove.is_ok() {
            game.make_move(nextMove.unwrap());
        } else {
            println!("Not a legal move!");
        }
    }
}


