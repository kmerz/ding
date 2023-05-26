use chess::{ChessMove, Color, Game, MoveGen};
use rand::Rng;

use crate::engine::Player;

#[derive(Default)]
pub struct RandomEng {}

impl Player for RandomEng {
    fn next_move(&self, game: &Game, _my_color: Color) -> Option<ChessMove> {
        let iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        if move_count == 0 {
            return None;
        }
        let choosen_move_idx = rand::thread_rng().gen_range(0..move_count);
        Some(
            iterable
                .enumerate()
                .find(|c| c.0 == choosen_move_idx)
                .unwrap()
                .1,
        )
    }
}
