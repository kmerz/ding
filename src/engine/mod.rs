use chess::{Game, ChessMove, MoveGen};
use rand::Rng;

pub struct Random { }

trait Engine {
    fn next_move(&self, game: &Game) -> ChessMove;
}

impl Engine for Random {
    fn next_move(&self, game: &Game) -> ChessMove {
        let mut iterable = MoveGen::new_legal(&game.current_position());
        let move_count = iterable.len();
        let choosen_move_idx = rand::thread_rng().gen_range(0..100);
        return iterable.next().unwrap();
    }
}
