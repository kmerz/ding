use chess::{Game, ChessMove, MoveGen};
use rand::Rng;


pub trait Engine {
    fn next_move(&self, game: &Game) -> ChessMove;
}


pub struct RandomEng { }

impl Engine for RandomEng {
    fn next_move(&self, game: &Game) -> ChessMove {
        let iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        let choosen_move_idx = rand::thread_rng().gen_range(0..move_count);
        iterable.enumerate().find(|c| c.0 == choosen_move_idx).unwrap().1
    }
}
