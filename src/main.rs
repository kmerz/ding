use std::str::FromStr;

use chess::{Game, ChessMove};
use clap::{arg, command};

mod ui;
mod engine;

use crate::engine::Engine;
use engine::CountingEng;

fn main() {
    let matches = command!()
        .arg(arg!(-f --fen <String> "Start the game from a fen string") .required(false))
        .get_matches();

    let mut game = Game::new();
    if let Some(fen) = matches.get_one::<String>("fen") {
        if let Ok(game_from_fen) = Game::from_str(fen) {
            game = game_from_fen;
        } else {
            println!("Could not find a valid game from fen, will start a new one!");
            game = Game::new();
        }
    }

    let eng = CountingEng {};

    while game.result().is_none() {
        println!();
        ui::print_board(&game.current_position());

        print!("> ");
        let input = ui::read_str();
        let command = ui::parse_command(input.as_str(), &mut game);
        if command == ui::Command::Success {
            continue;
        }
        let next_move = ChessMove::from_san(&game.current_position(), &input);
        if let Ok(next_move) = next_move{
            game.make_move(next_move);
            if let Some(next_move) = eng.next_move(&game) {
              game.make_move(next_move);
            } else {
                // Game Over - AI has no legal move left
                break;
            }
        } else {
            println!("Not a legal move!");
        }
    }
    println!();
    ui::print_board(&game.current_position());
    ui::print_result(&game);
    println!("Game Over!");
}
