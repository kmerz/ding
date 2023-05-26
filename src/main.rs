use std::str::FromStr;
use std::{thread, time};

use clap::{arg, command};

use chess::{Game, Color};

mod ui;
mod engine;

use crate::engine::Player;
use crate::ui::Human;
use engine::CountingEng;
use engine::RandomEng;
use engine::MinMaxEng;

fn main() {
    env_logger::init();

    let matches = command!()
        .arg(arg!(-f --fen <String> "Start the game from a fen string")
             .required(false))
        .arg(arg!(-w --white <String> "Choose between human, random, counting, minmax. Default is human")
             .required(false))
        .arg(arg!(-b --black <String> "Choose between human, random, counting, minmax. Default is minmax")
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

    let default_white = "human".to_string();
    let default_black = "minmax".to_string();

    let white = matches.get_one::<String>("white").unwrap_or(&default_white).as_str();
    let black = matches.get_one::<String>("black").unwrap_or(&default_black).as_str();

    let player_white: Box::<dyn Player> = match white {
        "random" => Box::<RandomEng>::default(),
        "minmax" => Box::<MinMaxEng>::default(),
        "counting" => Box::<CountingEng>::default(),
        _ => Box::<Human>::default(),
    };

    let player_black: Box::<dyn Player> = match black {
        "random" => Box::<RandomEng>::default(),
        "minmax" => Box::<MinMaxEng>::default(),
        "human" => Box::<Human>::default(),
        _ => Box::<CountingEng>::default(),
    };

    while game.result().is_none() {
        println!();
        ui::print_board(&game.current_position());

        let next_move = player_white.next_move(&game, Color::White);
        if let Some(next_move) = next_move {
            game.make_move(next_move);
            if let Some(next_move) = player_black.next_move(&game, Color::Black) {
              game.make_move(next_move);
            } else {
                break;
            }
        } 
        let fivehundred_millis = time::Duration::from_millis(500);
        thread::sleep(fivehundred_millis);
    }
    println!();
    ui::print_board(&game.current_position());
    ui::print_result(&game);
    println!("Game Over!");
}
