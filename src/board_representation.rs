/*
 * The board_representations module contains the definition of the boards, Position and
 * Coord structs, as well as various functions and methods to interact with them. In this
 * module, we only consider static positions representations, boards are not movable yet
 * (see the move_generation module for that).
 *
 * The current state of a game is represented by the Position type, which contains redundant
 * piece and square centric representations of the board, as well as some metadata (such
 * as the current turn, castling info, ...).
 *
 * This module is the core of the engine, and contains definitions for all fundamental
 * datatypes.
 */

pub mod datatypes;
pub mod misc;
pub mod static_board;

pub use datatypes::*;
pub use misc::*;
pub use static_board::*;

/******
* TESTS
*******/

#[test]
fn test_static_board() {
    // BitBoard

    let mut bit: BitBoard = BitBoard::new();
    let bit_c8: PieceCode = bit.get_square(Coord::new(2, 7));
    let mut bit_e2: PieceCode = bit.get_square(Coord::new(4, 1));
    let mut bit_e4: PieceCode = bit.get_square(Coord::new(4, 3));

    print!("Bit board :\n{}", bit.ascii());
    println!("Bit c8 : {}", get_unicode_piece(bit_c8));
    println!("Bit e2 : {}", get_unicode_piece(bit_e2));
    println!("Bit e4 : {}\n", get_unicode_piece(bit_e4));

    assert!(bit_c8 == PieceCode::BB, "Failed at assert 0");
    assert!(bit_e2 == PieceCode::WP, "Failed at assert 1");
    assert!(bit_e4 == PieceCode::ES, "Failed at assert 2");

    bit.set_square(PieceCode::ES, Coord::new(4, 1));
    bit.set_square(PieceCode::WP, Coord::new(4, 3));
    bit_e2 = bit.get_square(Coord::new(4, 1));
    bit_e4 = bit.get_square(Coord::new(4, 3));

    print!("Bit board after playing e4 :\n{}", bit.ascii());
    println!("Bit e2 after playing e4 : {}", get_unicode_piece(bit_e2));
    println!(
        "Bit e4 after playing e4 : {}\n\n",
        get_unicode_piece(bit_e4)
    );

    assert!(bit_e2 == PieceCode::ES, "Failed at assert 3");
    assert!(bit_e4 == PieceCode::WP, "Failed at assert 4");

    // Zerox88Board

    let mut zerox: Zerox88Board = Zerox88Board::new();
    let zerox_c8: PieceCode = zerox.get_square(Coord::new(2, 7));
    let mut zerox_e2: PieceCode = zerox.get_square(Coord::new(4, 1));
    let mut zerox_e4: PieceCode = zerox.get_square(Coord::new(4, 3));

    print!("0x88 board :\n{}", zerox.ascii());
    println!("0x88 c8 : {}", get_unicode_piece(zerox_c8));
    println!("0x88 e2 : {}", get_unicode_piece(zerox_e2));
    println!("0x88 e4 : {}\n", get_unicode_piece(zerox_e4));

    assert!(zerox_c8 == PieceCode::BB, "Failed at assert 4");
    assert!(zerox_e2 == PieceCode::WP, "Failed at assert 5");
    assert!(zerox_e4 == PieceCode::ES, "Failed at assert 6");

    zerox.set_square(PieceCode::ES, Coord::new(4, 1));
    zerox.set_square(PieceCode::WP, Coord::new(4, 3));
    zerox_e2 = zerox.get_square(Coord::new(4, 1));
    zerox_e4 = zerox.get_square(Coord::new(4, 3));

    print!("0x88 board after playing e4 :\n{}", zerox.ascii());
    println!("0x88 e2 after playing e4 : {}", get_unicode_piece(zerox_e2));
    println!(
        "0x88 e4 after playing e4 : {}\n",
        get_unicode_piece(zerox_e4)
    );

    assert!(zerox_e2 == PieceCode::ES, "Failed at assert 7");
    assert!(zerox_e4 == PieceCode::WP, "Failed at assert 8");

    // Test board copies
    bit.set_square(PieceCode::ES, Coord::new(4, 6));
    bit.set_square(PieceCode::BP, Coord::new(4, 4));
    zerox.apply_bitboard(&bit);

    zerox.set_square(PieceCode::ES, Coord::new(4, 0));
    zerox.set_square(PieceCode::WK, Coord::new(4, 1));
    bit.apply_0x88_board(&zerox);
    bit_e2 = bit.get_square(Coord::new(4, 1));

    print!("Bit board after copies :\n{}", bit.ascii());
    println!("Bit e2 after copies : {}", get_unicode_piece(bit_e2));

    assert!(bit_e2 == PieceCode::WK, "Failed at assert 9");

    // Test the get_square_bitboard func
    assert!(get_square_bitboard(Coord::new(4, 1)) == 0b1 << 63 >> 12);
}
