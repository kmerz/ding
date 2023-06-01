use chess::{
    get_bishop_moves, get_knight_moves, get_rook_moves, BitBoard, Board, BoardStatus, ChessMove,
    Color, Game, Piece, Square, ALL_SQUARES,
};

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

fn get_knight_moves_blockers(square: Square, _blockers: BitBoard) -> BitBoard {
    get_knight_moves(square)
}

fn is_check(board: &Board, color_to_move: &Color) -> bool {
    let color_to_check = get_other_color(color_to_move);
    let king_square = board.king_square(color_to_check);

    let opp_pieces = board.color_combined(*color_to_move);
    let mut is_check: bool = false;
    for square in *opp_pieces {
        is_check = match board.piece_on(square) {
            Some(Piece::Rook) => is_check_with_fn(get_rook_moves, board, &square, &king_square),
            Some(Piece::Bishop) => is_check_with_fn(get_bishop_moves, board, &square, &king_square),
            Some(Piece::Knight) => {
                is_check_with_fn(get_knight_moves_blockers, board, &square, &king_square)
            }
            Some(Piece::Queen) => {
                is_check_with_fn(get_rook_moves, board, &square, &king_square)
                    || is_check_with_fn(get_bishop_moves, board, &square, &king_square)
            }
            None => false,
            _ => false,
        };
        if is_check {
            break;
        }
    }
    is_check
}

fn is_check_with_fn(
    func: fn(Square, BitBoard) -> BitBoard,
    board: &Board,
    square: &Square,
    king_square: &Square,
) -> bool {
    let rook_moves = func(*square, *board.combined());
    for rook_move in rook_moves {
        let dest = rook_move;
        if dest == *king_square {
            return true;
        }
    }
    false
}

fn inspect_move(board: &Board, my_color: &Color) -> i32 {
    let opp_color = get_other_color(my_color);
    let value_opp = count_pieces(board, &opp_color);
    let value_mine = count_pieces(board, my_color);
    let is_check = is_check(board, my_color);

    let checkmate = match board.status() {
        BoardStatus::Ongoing => 0,
        BoardStatus::Stalemate => 0,
        BoardStatus::Checkmate => 10000,
    };

    let check = if is_check { 1000 } else { 0 };

    value_mine - value_opp + checkmate + check
}

fn color_to_str(color: Color) -> &'static str {
    match color {
        Color::White => "White",
        Color::Black => "Black",
    }
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
