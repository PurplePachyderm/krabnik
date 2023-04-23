mod board_representation;
use board_representation::*;

fn main() {
    println!("Hello, world!");

    let bit: BitBoard = BitBoard::new();
    print!("Bit board :\n{}", bit.ascii());
    print!(
        "Bit c8 : {}\n\n",
        get_unicode_piece(bit.get_square(Coord::new(2, 7)))
    );

    let zerox88: Zerox88Board = Zerox88Board::new();
    print!("Zerox88 board :\n{}", zerox88.ascii());
    print!(
        "Zerox88 c8 : {}\n\n",
        get_unicode_piece(zerox88.get_square(Coord::new(2, 7)))
    );

    let pos: Position = Position::new();
    print!("Pos board :\n{}", pos.ascii());
    print!(
        "Pos c8 : {}\n\n",
        get_unicode_piece(pos.get_square(Coord::new(2, 7)))
    );
}
