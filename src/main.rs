use chess::{Game, Square, ChessMove, Board};

mod ui;

fn main() {
    let mut game = Game::new();

    while game.result().is_none() {
        ui::print_board(&game.current_position());

        print!("> ");
        let mut input = ui::read_str();
        println!("You typed: {}", input);

    }
}


