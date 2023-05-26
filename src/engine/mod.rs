use chess::{Board, ChessMove, Color, Game, Piece, ALL_SQUARES};

pub mod counting;
pub mod minmax;
pub mod random;

pub trait Player {
    fn next_move(&self, game: &Game, my_color: Color) -> Option<ChessMove>;
}

fn get_other_color(color: &Color) -> Color {
    match color {
        Color::Black => Color::White,
        Color::White => Color::Black,
    }
}

fn inspect_move(board: &Board, my_color: &Color, invert: bool) -> i32 {
    let opp_color = get_other_color(my_color);
    let value_opp = count_pieces(board, &opp_color);
    let value_mine = count_pieces(board, my_color);
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
