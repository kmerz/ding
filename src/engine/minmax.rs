use chess::{ChessMove, Color, Game, MoveGen};
use log::{debug, info};
use std::collections::HashMap;

use crate::engine::inspect_move;
use crate::engine::Player;

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

            let result_op_move = valued_opp_moves.iter().max_by_key(|entry| entry.1);
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
        let next_move = *valued_moves.iter().max_by_key(|entry| entry.1).unwrap().0;
        info!("next_move: {}", next_move);
        Some(next_move)
    }
}
