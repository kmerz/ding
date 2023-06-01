use chess::{ChessMove, Color, Game, MoveGen};
use log::{debug, info};
use std::collections::HashMap;

use crate::engine::inspect_move;
use crate::engine::Player;

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
            let result = inspect_move(new_board, &my_color);
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
