use chess::{Game, ChessMove, MoveGen, Board, ALL_SQUARES, Color, Piece};
use std::collections::HashMap;
use rand::Rng;
use log::{info, debug};

pub trait Player {
    fn next_move(&self, game: &Game, my_color: Color) -> Option<ChessMove>;
}

#[derive(Default)]
pub struct RandomEng { }

impl Player for RandomEng {
    fn next_move(&self, game: &Game, _my_color: Color) -> Option<ChessMove> {
        let iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let choosen_move_idx = rand::thread_rng().gen_range(0..move_count);
        Some(iterable.enumerate().find(|c| c.0 == choosen_move_idx).unwrap().1)
    }
}

#[derive(Default)]
pub struct MinMaxEng {}

impl Player for MinMaxEng {
    fn next_move(&self, game: &Game, my_color: Color) -> Option<ChessMove> {
        let board = game.current_position();
        let iterable = MoveGen::new_legal(&board);
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let mut valued_moves = HashMap::new();
        for next_move in iterable {
            let new_board = &board.make_move_new(next_move);
            let result_my_move = inspect_move(&board, &my_color, false);

            let opp_iterable = MoveGen::new_legal(new_board);
            let mut valued_opp_moves: HashMap<ChessMove, i32> = HashMap::new();
            for next_opp_move in opp_iterable {
                let new_opp_board = new_board.make_move_new(next_opp_move);
                let result_opp_move = inspect_move(&new_opp_board, &my_color, true);
                valued_opp_moves.insert(next_move, result_opp_move);
            }

            let result_op_move = valued_opp_moves.iter()
                .max_by_key(|entry| entry.1);
            let result_op = match result_op_move {
                Some(result) => result.1,
                None => &0,
            };
            
            let result = result_my_move - result_op;
            valued_moves.insert(next_move, result);
        }
        for (next_move, value) in valued_moves.iter() {
            debug!("move: {}, value: {}", next_move, value);
        }
        let next_move = *valued_moves.iter().max_by_key(|entry| entry.1)
            .unwrap().0;
        info!("next_move: {}", next_move);
        Some(next_move)
    }
}

#[derive(Default)]
pub struct CountingEng {}

impl Player for CountingEng {
    fn next_move(&self, game: &Game, my_color: Color) -> Option<ChessMove> {
        let board = game.current_position();
        let iterable = MoveGen::new_legal(&board);
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let mut valued_moves = HashMap::new();
        for next_move in iterable {
            let new_board = &board.make_move_new(next_move);
            let result = inspect_move(new_board, &my_color, false);
            valued_moves.insert(next_move, result);
        }
        for (next_move, value) in valued_moves.iter() {
            debug!("move: {}, value: {}", next_move, value);
        }
        let next_move = *valued_moves.iter().max_by_key(|entry| entry.1)
            .unwrap().0;
        info!("next_move: {}", next_move);
        Some(next_move)
    }
}

fn get_other_color(color: &Color) -> Color {
    match color {
        Color::Black => Color::White,
        Color::White => Color::Black
    }
}

fn inspect_move(board: &Board, my_color: &Color, invert: bool) -> i32 {
    let opp_color = get_other_color(&my_color);
    let value_opp = count_pieces(board, &opp_color);
    let value_mine = count_pieces(board, &my_color);
    let result: i32 = if invert {
        value_opp - value_mine
    } else {
        value_mine - value_opp
    };
    result
}

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
