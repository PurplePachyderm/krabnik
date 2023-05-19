/*
 * The board_representations module contains the definition of the boards, Position and
 * Coord structs, as well as various functions and methods to interact with them. In this
 * module, we only consider static positions representations, boards are not movable yet
 * (see the move_generation module for that).
 */

#![allow(dead_code)]

use super::datatypes::*;
use super::misc::*;

/******************
* STATICBOARD TRAIT
*******************/

/// This trait contains a collection of utlity methods to manipulate a position
pub trait StaticBoard {
    /// Reset the board to the starting position
    fn reset(&mut self);

    /// Return the piece code of the corresponding square
    fn get_square(&self, coord: Coord) -> PieceCode;
    /// Overwrites a square with a piece code
    fn set_square(&mut self, piece_code: PieceCode, coord: Coord);

    /// Return an ASCII (Unicode) representation of the board
    fn ascii(&self) -> String {
        let mut board_string: String = String::new();

        // Iterate over ranks in reverse order
        for i in (0..8u8).rev() {
            // Print the rank number
            board_string.push((i + 49) as char);
            board_string.push(' ');

            // Iterate over files
            for j in 0..8u8 {
                board_string.push(' ');
                let piece_symbol: char = get_unicode_piece(self.get_square(Coord::new(j, i)));
                board_string.push(piece_symbol);
            }
            board_string.push('\n');
        }
        board_string.push_str("   a b c d e f g h\n");

        board_string
    }
}

impl StaticBoard for BitBoard {
    fn reset(&mut self) {
        self.main_boards = [
            0b11111111 << 48, // White pawns
            0b01000010 << 56, // White knights
            0b00100100 << 56, // White bishops
            0b10000001 << 56, // White rooks
            0b00010000 << 56, // White queen
            0b00001000 << 56, // White king
            0b11111111 << 8,  // Black pawns
            0b01000010,       // Black knights
            0b00100100,       // Black bishops
            0b10000001,       // Black rooks
            0b00010000,       // Black queen
            0b00001000,       // Black king
        ];
        self.en_passant_board = 0;
    }

    fn get_square(&self, coord: Coord) -> PieceCode {
        let mask: u64 = 0b10000000 << ((7 - coord.r) << 3) >> coord.f;

        for i in 0..12 {
            let piece: u64 = self.main_boards[i] & mask;
            if piece != 0 {
                return PieceCode::from_usize(i + 1);
            }
        }
        PieceCode::ES
    }

    fn set_square(&mut self, piece_code: PieceCode, coord: Coord) {
        let mask: u64 = 0b10000000 << ((7 - coord.r) << 3) >> coord.f;

        if piece_code != PieceCode::ES {
            self.main_boards[piece_code as usize - 1] |= mask;
        } else {
            // We're emptying a square, this is a special case, as we will need to find
            // the piece board we will be overwriting
            for i in 0..12 {
                let piece: u64 = self.main_boards[i] & mask;
                if piece != 0 {
                    // We found the correct piece board
                    self.main_boards[i] &= !mask;
                    return;
                }
            }
        }
    }
}

impl StaticBoard for Zerox88Board {
    #[rustfmt::skip]
    fn reset(&mut self) {
        self.main_board = [
            PieceCode::WR, PieceCode::WN, PieceCode::WB, PieceCode::WQ, PieceCode::WK, PieceCode::WB, PieceCode::WN, PieceCode::WR,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::WP, PieceCode::WP, PieceCode::WP, PieceCode::WP, PieceCode::WP, PieceCode::WP, PieceCode::WP, PieceCode::WP,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,

            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,

            PieceCode::BP, PieceCode::BP, PieceCode::BP, PieceCode::BP, PieceCode::BP, PieceCode::BP, PieceCode::BP, PieceCode::BP,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
            PieceCode::BR, PieceCode::BN, PieceCode::BB, PieceCode::BQ, PieceCode::BK, PieceCode::BB, PieceCode::BN, PieceCode::BR,
            PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES, PieceCode::ES,
        ];

        self.en_passant_board = [PieceCode::ES; 128];
    }

    fn get_square(&self, coord: Coord) -> PieceCode {
        self.main_board[((coord.r << 4) + coord.f) as usize]
    }

    fn set_square(&mut self, piece_code: PieceCode, coord: Coord) {
        self.main_board[((coord.r << 4) + coord.f) as usize] = piece_code;
    }
}

impl StaticBoard for Position {
    fn reset(&mut self) {
        // Reset the two boards
        self.piece_centric_board.reset();
        self.square_centric_board.reset();

        // Reset the position "metadata"
        self.current_turn = Player::White;
        self.can_castle_kingside = [true, true];
        self.can_castle_queenside = [true, true];
        self.plys_without_capture = 0;
    }

    fn get_square(&self, coord: Coord) -> PieceCode {
        // More efficient in square centric representation
        self.square_centric_board.get_square(coord)
    }

    fn set_square(&mut self, piece_code: PieceCode, coord: Coord) {
        self.piece_centric_board.set_square(piece_code, coord);
        self.square_centric_board.set_square(piece_code, coord);
    }

    fn ascii(&self) -> String {
        self.square_centric_board.ascii() // More efficient in square centric representation
    }
}
