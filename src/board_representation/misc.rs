#![allow(dead_code)]

use super::datatypes::*;
use super::static_board::*;

/*************************
* MISC METHODS & FUNCTIONS
**************************/

/// Macro to return a bitboard on the a1 square. This bitboard can later be shifted to point
/// to another square.
macro_rules! a1_bitboard {
    () => {
        0x8000000000000000
    };
}
pub(crate) use a1_bitboard;

/// From a Coord, get a bitboard corresponding to the square
pub fn get_square_bitboard(coord: Coord) -> u64 {
    a1_bitboard!() >> (coord.r << 3) >> coord.f
}

pub fn invert_player(player: &Player) -> Player {
    match player {
        Player::White => Player::Black,
        Player::Black => Player::White,
    }
}

impl BitBoard {
    /// Copy content of a 0x88 board onto the bitboard
    pub fn apply_0x88_board(&mut self, zerox88_board: &Zerox88Board) {
        // Clean all bitboards
        for i in 0..12 {
            self.main_boards[i] = 0;
        }
        self.en_passant_board = 0;

        // Copy pieces
        let mut mask: u64 = a1_bitboard!();
        for i in 0..8 {
            for j in 0..8 {
                // Fill main boards
                let piece_code = zerox88_board.get_square(Coord::new(j, i));
                if piece_code != PieceCode::ES {
                    self.main_boards[piece_code as usize - 1] |= mask;
                }

                // Fill en passant board
                let en_passant_piece_code = zerox88_board.en_passant_board[((i << 4) + j) as usize];
                if en_passant_piece_code != PieceCode::ES {
                    self.en_passant_board |= mask;
                }

                mask >>= 1;
            }
        }
    }
}

impl Zerox88Board {
    /// Copy content of a bitboard onto the 0x88 board
    pub fn apply_bitboard(&mut self, bitboard: &BitBoard) {
        // Clean boards
        for i in 0..128 {
            self.main_board[i] = PieceCode::ES;
            self.en_passant_board[i] = PieceCode::ES;
        }

        // Copy pieces
        let mut mask: u64 = a1_bitboard!();
        for i in 0..8 {
            for j in 0..8 {
                // Fill main board
                for k in 0..12 {
                    let filtered_board = bitboard.main_boards[k] & mask;
                    if filtered_board != 0 {
                        self.main_board[(i << 4) + j] = PieceCode::from_usize(k + 1);
                    }
                }

                // Fill en passant board
                let filtered_en_passant_board = bitboard.en_passant_board & mask;
                if filtered_en_passant_board != 0 {
                    // We'll put a WP by default, but any PieceCode (!= ES) is fine
                    self.en_passant_board[(i << 4) + j] = PieceCode::WP;
                }

                mask = mask >> 1;
            }
        }
    }
}

impl Position {
    /// Copy content of a 0x88 board onto the Position
    pub fn apply_0x88_board(&mut self, zerox88_board: &Zerox88Board) {
        self.piece_centric_board.apply_0x88_board(zerox88_board);
    }

    /// Copy content of a bitboard onto the Position
    pub fn apply_bitboard(&mut self, bitboard: &BitBoard) {
        self.square_centric_board.apply_bitboard(bitboard);
    }

    /// Copy content of the Position's own 0x88 board onto its own bitboard
    pub fn apply_own_0x88_board(&mut self) {
        self.piece_centric_board
            .apply_0x88_board(&self.square_centric_board);
    }

    /// Copy content of the Position's own bitboard onto its own 0x88 board
    pub fn apply_own_bitboard(&mut self) {
        self.square_centric_board
            .apply_bitboard(&self.piece_centric_board);
    }
}

/// Shorthand to generate integer to PieceCode cast functions
macro_rules! gen_piece_code_cast {
    ($name : ident, $type : ty) => {
        pub fn $name(piece_code: $type) -> PieceCode {
            gen_piece_code_cast_body!(piece_code)
        }
    };
}

/// Shorthand to generate integer to PieceCode cast functions (function body)
macro_rules! gen_piece_code_cast_body {
    ($piece_code : ident) => {
        match $piece_code {
            0 => PieceCode::ES,
            1 => PieceCode::WP,
            2 => PieceCode::WN,
            3 => PieceCode::WB,
            4 => PieceCode::WR,
            5 => PieceCode::WQ,
            6 => PieceCode::WK,
            7 => PieceCode::BP,
            8 => PieceCode::BN,
            9 => PieceCode::BB,
            10 => PieceCode::BR,
            11 => PieceCode::BQ,
            12 => PieceCode::BK,
            _ => panic!("Unknown PieceCode: {}", $piece_code),
        }
    };
}

impl PieceCode {
    gen_piece_code_cast!(from_u8, u8);
    gen_piece_code_cast!(from_u16, u16);
    gen_piece_code_cast!(from_u32, u32);
    gen_piece_code_cast!(from_u64, u64);
    gen_piece_code_cast!(from_usize, usize);
}

/// From a piece ID, return its Unicode character
pub fn get_unicode_piece(piece_code: PieceCode) -> char {
    match piece_code {
        PieceCode::ES => '.',

        PieceCode::WP => '♙',
        PieceCode::WN => '♘',
        PieceCode::WB => '♗',
        PieceCode::WR => '♖',
        PieceCode::WQ => '♕',
        PieceCode::WK => '♔',

        PieceCode::BP => '♟',
        PieceCode::BN => '♞',
        PieceCode::BB => '♝',
        PieceCode::BR => '♜',
        PieceCode::BQ => '♛',
        PieceCode::BK => '♚',
    }
}
