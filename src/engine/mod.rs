use chess::{Game, ChessMove, MoveGen, Board, ALL_SQUARES, Color, Piece};
use std::collections::HashMap;
use rand::Rng;


pub trait Engine {
    fn next_move(&self, game: &Game) -> Option<ChessMove>;
}

pub struct RandomEng { }

impl Engine for RandomEng {
    fn next_move(&self, game: &Game) -> Option<ChessMove> {
        let iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let choosen_move_idx = rand::thread_rng().gen_range(0..move_count);
        Some(iterable.enumerate().find(|c| c.0 == choosen_move_idx).unwrap().1)
    }
}

pub struct CountingEng {}


impl Engine for CountingEng {
    fn next_move(&self, game: &Game) -> Option<ChessMove> {
        let board = game.current_position();
        let iterable = MoveGen::new_legal(&board);
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let mut valued_moves = HashMap::new();
        for next_move in iterable {
            let value_black = count_pieces(&board.make_move_new(next_move),
                &Color::Black);
            let value_white = count_pieces(&board.make_move_new(next_move),
                &Color::White);
            let result = value_black - value_white;
            valued_moves.insert(next_move, result);
        }
        // TODO: Turn this into debug logging
        for (next_move, value) in valued_moves.iter() {
            println!("move: {}, value: {}", next_move, value);
        }
        let next_move = *valued_moves.iter().max_by_key(|entry| entry.1)
            .unwrap().0;
        // TODO: Turn this into debug logging
        println!("next_move: {}", next_move);
        Some(next_move)
    }
}

// TODO: Add tests
fn count_pieces(board: &Board, color: &Color) -> i32 {
    let mut sum: i32 = 0;
    for sq in ALL_SQUARES.iter() {
        if let Some(found_color) = board.color_on(*sq) {
            if found_color == *color {
                if let Some(piece) = board.piece_on(*sq) {
                    sum += lookup_value(&piece); 
                }
            }
        }
    }
    sum
}

fn lookup_value(piece: &Piece) -> i32 {
    match piece {
        Piece::Pawn => 1,
        Piece::Knight => 3,
        Piece::Bishop => 3,
        Piece::Rook => 5,
        Piece::Queen => 9,
        Piece::King => 99,
    }
}
