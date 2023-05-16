use std::str::FromStr;

use clap::{arg, command};
use env_logger;

use chess::{Game, ChessMove};

mod ui;
mod engine;

use crate::engine::Engine;
use engine::CountingEng;
use engine::RandomEng;

fn main() {
    env_logger::init();

    let matches = command!()
        .arg(arg!(-f --fen <String> "Start the game from a fen string")
             .required(false))
        .arg(arg!(-e --engine <String> "Choose the Random Engine: random or counting. Counting is default")
             .required(false))
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

    let default_engine = "counting".to_string();
    let engine = matches.get_one::<String>("engine").unwrap_or(&default_engine).as_str();

    let eng: Box::<dyn Engine> = match engine {
        "random" => Box::<RandomEng>::default(),
        _ => Box::<CountingEng>::default(),
    };

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
