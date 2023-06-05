use chess::{ChessMove, Color, Game, MoveGen};
use log::{debug, info};
use std::collections::HashMap;

use crate::engine::Player;
use crate::engine::{color_to_str, get_other_color, inspect_move};

#[derive(Default)]
pub struct TreeEng {}

impl Player for TreeEng {
    fn next_move(&self, game: &Game, my_color: Color) -> Option<ChessMove> {
        let board = game.current_position();
        let iterable = MoveGen::new_legal(&board);
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let default_value: i32 = 0;

        let mut valued_moves = HashMap::new();
        for next_move in iterable { 
            valued_moves.insert(next_move, default_value);
            let new_board = &board.make_move_new(next_move);
            let opp_iterable = MoveGen::new_legal(new_board);
            for next_opp_move in opp_iterable {
                let new_my_board = &new_board.make_move_new(next_opp_move);
                let new_my_iterable = MoveGen::new_legal(new_my_board);
                for my_snd_move in new_my_iterable {
                    let new_my_op_board = &new_my_board.make_move_new(my_snd_move);
                    let new_my_iterable = MoveGen::new_legal(new_my_op_board);
                    for my_last_move in new_my_iterable {
                        let board_to_inspect = &new_my_op_board.make_move_new(my_last_move);
                        let value = inspect_move(&board_to_inspect, &my_color);
                        let current_value = valued_moves.get(&next_move).unwrap();
                        if value >= *current_value {
                            valued_moves.insert(next_move, value);
                        }
                    } 
                } 
            }
        }

        let next_value = *valued_moves.iter().max_by_key(|entry| entry.1).unwrap().1;
        let next_move = *valued_moves.iter().max_by_key(|entry| entry.1).unwrap().0;
        info!(
            "color: {}, next_move: {}: value: {}",
            color_to_str(my_color),
            next_move,
            next_value
        );

        Some(next_move)
    }
}
