use std::process::exit;

use chess::{Game, ChessMove};

mod ui;
mod engine;

use crate::engine::Engine;
use engine::RandomEng;

fn main() {
    let mut game = Game::new();
    let eng = RandomEng {};

    while game.result().is_none() {
        ui::print_board(&game.current_position());

        print!("> ");
        let input = ui::read_str();
        if input.eq("quit!") {
            exit(0);
        }
        let next_move = ChessMove::from_san(&game.current_position(), &input);
        if let Ok(next_move) = next_move{
            game.make_move(next_move);
            game.make_move(eng.next_move(&game));
        } else {
            println!("Not a legal move!");
        }
    }
    println!("Game finished");
}
