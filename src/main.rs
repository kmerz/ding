use chess::{Board};

fn main() {
    let board = Board::default();
    print_board(&board);
}

fn print_board(board: &Board) {
    //  A B C D E F G H
    //8 ♚ . . . . . ♖ .
    //7 . . . . ♖ . . .
    //6 . . . . . . . .
    //5 . . . . . . . .
    //4 . . . . . . . .
    //3 . . . . . . . .
    //2 . . . . . . . .
    //1 . . . . . . . .
    
    print!("  A B C D E F G H\n8 ");

    const ROWS: i32 = 8;
    let mut counter = 1; 
    let board_fen_string = board.to_string();
    let positions = board_fen_string.split(" ").next().unwrap();
    for position in positions.chars() {
        if position.is_numeric() {
            let empty_fields = (position.to_string()).parse::<i16>().unwrap();
            for _ in 0..empty_fields {
                print!(". ")
            }
        } else if position == '/' {
            let row = ROWS - counter;
            print!("\n{} ", row); 
            counter = counter + 1;
        } else {
            let fig = lookup_figure(position);
            print!("{} ", fig);
        }
    }

    print!("\n");
}

fn lookup_figure(current: char) -> &'static str {
    match current {
      'K' => "♔",
      'Q' => "♕",
      'R' => "♖",
      'B' => "♗",
      'N' => "♘",
      'P' => "♙",
      'k' => "♚",
      'q' => "♛",
      'r' => "♜",
      'b' => "♝",
      'n' => "♞",
      'p' => "♟︎",
      _ => "."
    }
}
