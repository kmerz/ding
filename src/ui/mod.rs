use chess::{Board, Game, GameResult};
use std::io::{stdin,stdout,Write};
use std::process::exit;
 
#[derive(PartialEq)]
pub enum Command {
    Success,
    Unknown,
}

pub fn print_board(board: &Board) {
    //  A B C D E F G H
    //8 ♚ . . . . . ♖ .
    //7 . . . . ♖ . . .
    //6 . . . . . . . .
    //5 . . . . . . . .
    //4 . . . . . . . .
    //3 . . . . . . . .
    //2 . . . . . . . .
    //1 . . . . . . . .

    print!("  A B C D E F G H\n8 ");

    const ROWS: i32 = 8;
    let mut counter = 1; 
    let board_fen_string = board.to_string();
    let positions = board_fen_string.split(' ').next().unwrap();
    for position in positions.chars() {
        if position.is_numeric() {
            let empty_fields = (position.to_string()).parse::<i16>().unwrap();
            for _ in 0..empty_fields {
                print!(". ");
            }
        } else if position == '/' {
            let row = ROWS - counter;
            print!("\n{} ", row); 
            counter += 1;
        } else {
            let fig = lookup_figure(position);
            print!("{} ", fig);
        }
    }

    println!();
}

fn lookup_figure(current: char) -> &'static str {
    match current {
        'K' => "♔",
        'Q' => "♕",
        'R' => "♖",
        'B' => "♗",
        'N' => "♘",
        'P' => "♙",
        'k' => "♚",
        'q' => "♛",
        'r' => "♜",
        'b' => "♝",
        'n' => "♞",
        'p' => "♟︎",
        _ => "."
    }
}

pub fn read_str() -> String {
    let mut input = String::new();
    let _=stdout().flush();

    stdin().read_line(&mut input).expect("Did not enter a correct string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    input
}

pub fn print_result(game: &Game) {
    if game.result().is_some() {
        match game.result().unwrap() {
            GameResult::WhiteCheckmates => println!("White checkmates Black"),
            GameResult::WhiteResigns => println!("White resigns"),
            GameResult::BlackCheckmates => println!("Black checkmates White"),
            GameResult::BlackResigns => println!("Black resigns"),
            GameResult::Stalemate => println!("Stalemate"),
            GameResult::DrawAccepted => println!("Draw accepted"),
            GameResult::DrawDeclared => println!("Draw declared"),
        }
    }
}



pub fn parse_command(input: &str, game: &mut Game) -> Command {
    match input {
        "quit!" => exit(0),
        "print" => print_fen(game),
        "help"  => print_help(),
        "new"   => start_new_game(game),
        _ => Command::Unknown,
    }
}

fn print_fen(game: &Game) -> Command {
    println!("FEN: {}", game.current_position());
    Command::Success
}

fn start_new_game(game: &mut Game) -> Command {
    *game = Game::new();
    Command::Success
}

fn print_help() -> Command {
    println!("Following Commands are avaiable:\n");
    println!("quit! - To quit the game immediately");
    println!("print - Print the current board in fen notation");
    println!("help - Print this help message");
    println!();
    println!("For a move use SAN notation:");
    println!("e4 Qh5xf7 or 0-0");
    Command::Success
}
