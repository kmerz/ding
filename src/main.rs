use std::process::exit;

use chess::{Game, ChessMove, MoveGen};
use rand::Rng;

pub struct Random { }

trait Engine {
    fn next_move(&self, game: &Game) -> ChessMove;
}

impl Engine for Random {
    fn next_move(&self, game: &Game) -> ChessMove {
        let iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        let choosen_move_idx = rand::thread_rng().gen_range(0..move_count);
        iterable.enumerate().find(|c| c.0 == choosen_move_idx).unwrap().1
    }
}

mod ui;

fn main() {
    let mut game = Game::new();
    let eng = Random {};

    while game.result().is_none() {
        ui::print_board(&game.current_position());

        print!("> ");
        let input = ui::read_str();
        if input.eq("quit!") {
            exit(0);
        }
        let next_move = ChessMove::from_san(&game.current_position(), &input);
        if next_move.is_ok() {
            game.make_move(next_move.unwrap());
            game.make_move(eng.next_move(&game));
        } else {
            println!("Not a legal move!");
        }
    }
    println!("Game finished");
}


