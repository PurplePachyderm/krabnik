/*
 * The board_representations module contains the definition of the boards, Position and
 * Coord structs, as well as various functions and methods to interact with them. In this
 * module, we only consider static positions representations, boards are not movable yet
 * (see the move_generation module for that).
 */

#![allow(dead_code)]

/**********
* DATATYPES
***********/

/// Represents either the black or white player
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    White,
    Black,
}

/// Represents any piece, or the empty square
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceCode {
    ES, // Empty square

    WP, // White pawn
    WN, // White knight
    WB, // White bishop
    WR, // White rook
    WQ, // White queen
    WK, // White king

    BP, // Black pawn
    BN, // Black knight
    BB, // Black bishop
    BR, // Black rook
    BQ, // Black queen
    BK, // Black king
}

/// Piece centric bitoard representation. One u64 represents a board in row-major ordering
/// (starting from the a1 square)
/// See : <https://www.chessprogramming.org/Bitboards>
#[derive(Debug)]
pub struct BitBoard {
    /// Main boards for positions of each piece type.
    /// Should be indexed using PieceCode
    pub main_boards: [u64; 12],

    /// Only one en passant bitboard is needed, as it changes every ply
    pub en_passant_board: u64,
}

/// Square centric 0x88 board representation. Its values correspond to the PieceCode values.
/// See : <https://www.chessprogramming.org/0x88>
#[derive(Debug)]
pub struct Zerox88Board {
    pub main_board: [PieceCode; 128],
    pub en_passant_board: [PieceCode; 128],
}

/// This structure contains all the required data to represent a chess position.
/// This includes pieces positions (in redundant piece and square centric board
/// representations), as well as additional game state informations such as the current turn,
/// castling possibilities, ...
#[derive(Debug)]
pub struct Position {
    /* Pieces positions */
    /// Piece-centric bitboard representation
    pub piece_centric_board: BitBoard,

    /// Square-centric 0x88 representation
    pub square_centric_board: Zerox88Board,

    /* Other game state informations */
    pub current_turn: Player,

    /// Specify if each player can castle on each side.
    /// Should be indexed using the Player enum for clarity.
    pub can_castle_kingside: [bool; 2],
    pub can_castle_queenside: [bool; 2],

    /// Used for the 50 moves rule
    pub plys_without_capture: u8,
    /* TODO Add a way to check for threefold repetitions. This will likely involve
     * transposition tables. However, a linked list containing all the previous boards could
     * work at the beginning, albeit quite inefficient. */
}

/// Stores a coordinate in algebraic notation. Files are indexed from 0 to 7 instead of a-h.
/// Because value checking is done at the structure creation, Coord is safe to use and does
/// not require any additional checking.
#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub f: u8,
    pub r: u8,
}

/*************
* INIT METHODS
**************/

/// Default trait for bitboard is the normal starting position
impl Default for BitBoard {
    fn default() -> BitBoard {
        BitBoard {
            // Each binary litteral can be seen as a rank, with the bit shifts specifying
            // which rank we are inserting pieces in.
            main_boards: [
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
            ],
            en_passant_board: 0,
        }
    }
}

impl BitBoard {
    /// Shorthand for default
    pub fn new() -> BitBoard {
        BitBoard::default()
    }
}

/// Default trait for Zerox88Board is the normal starting position
impl Default for Zerox88Board {
    #[rustfmt::skip]
    fn default() -> Zerox88Board {
        Zerox88Board {
            main_board: [
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
            ],

            en_passant_board: [PieceCode::ES; 128],
        }
    }
}

impl Zerox88Board {
    /// Shorthand for default
    pub fn new() -> Zerox88Board {
        Zerox88Board::default()
    }
}

/// Default trait for Position is the normal starting position.
/// Both piece and square centric boards are initialized.
impl Default for Position {
    fn default() -> Position {
        Position {
            piece_centric_board: BitBoard::default(),
            square_centric_board: Zerox88Board::default(),
            current_turn: Player::White,
            can_castle_kingside: [true, true],
            can_castle_queenside: [true, true],
            plys_without_capture: 0,
        }
    }
}

impl Position {
    /// Shorthand for default
    pub fn new() -> Position {
        Position::default()
    }
}

/// Default trait for Coord is the a1 square
impl Default for Coord {
    fn default() -> Coord {
        Coord { f: 0, r: 0 }
    }
}

impl Coord {
    /// Initialize from a couple of u8 with value checking
    pub fn new(file: u8, rank: u8) -> Coord {
        assert!(
            file < 8 && rank < 8,
            "Tried to build out of bounds coordinates ({}, {})",
            file,
            rank
        );
        Coord { f: file, r: rank }
    }
}
