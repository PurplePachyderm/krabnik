/*
 * Contains the definition of the boards, Position and Coord structs, as well as various
 * functionsand methods to interact with them. In this module, we only consider static
 * positions representations, boards are not movable yet (see move_generation.rs for that).
 */

/**********
* DATATYPES
***********/

/// Represents either the black or white player
#[derive(Copy, Clone, Debug)]
pub enum Player {
    White,
    Black,
}

/// Represents any piece, or the empty square
#[derive(Copy, Clone, Debug, PartialEq)]
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
    main_boards: [u64; 12],

    /// Only one en passant bitboard is needed, as it changes every ply
    en_passant_board: u64,
}

/// Square centric 0x88 board representation. Its values correspond to the PieceCode values.
/// See : <https://www.chessprogramming.org/0x88>
#[derive(Debug)]
pub struct Zerox88Board {
    main_board: [PieceCode; 128],
    en_passant_board: [PieceCode; 128],
}

/// This structure contains all the required data to represent a chess position.
/// This includes pieces positions (in redundant piece and square centric board
/// representations), as well as additional game state informations such as the current turn,
/// castling possibilities, ...
#[derive(Debug)]
pub struct Position {
    /* Pieces positions */
    /// Piece-centric bitboard representation
    piece_centric_board: BitBoard,

    /// Square-centric 0x88 representation
    square_centric_board: Zerox88Board,

    /* Other game state informations */
    current_turn: Player,

    /// Specify if each player can castle on each side.
    /// Should be indexed using the Player enum for clarity.
    can_castle_kingside: [bool; 2],
    can_castle_queenside: [bool; 2],

    /// Used for the 50 moves rule
    plys_without_capture: u8,
    /* TODO Add a way to check for threefold repetitions. This will likely involve
     * transposition tables. However, a linked list containing all the previous boards could
     * work at the beginning, albeit quite inefficient. */
}

/// Stores a coordinate in algebraic notation. Files are indexed from 0 to 7 instead of a-h.
/// Because value checking is done at the structure creation, Coord is safe to use and does
/// not require any additional checking.
#[derive(Copy, Clone, Debug)]
pub struct Coord {
    file: u8,
    rank: u8,
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
        Coord { file: 0, rank: 0 }
    }
}

impl Coord {
    /// Initialize from a couple of u8 with value checking
    pub fn new(f: u8, r: u8) -> Coord {
        assert!(f < 8 && r < 8);
        Coord { file: f, rank: r }
    }
}

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
        let mask: u64 = 0b10000000 << ((7 - coord.rank) << 3) >> coord.file;

        for i in 0..12 {
            let piece: u64 = self.main_boards[i] & mask;
            if piece != 0 {
                return PieceCode::from_usize(i + 1);
            }
        }
        PieceCode::ES
    }

    fn set_square(&mut self, piece_code: PieceCode, coord: Coord) {
        let mask: u64 = 0b10000000 << ((7 - coord.rank) << 3) >> coord.file;

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
        self.main_board[((coord.rank << 4) + coord.file) as usize]
    }

    fn set_square(&mut self, piece_code: PieceCode, coord: Coord) {
        self.main_board[((coord.rank << 4) + coord.file) as usize] = piece_code;
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

/***************
* MISC FUNCTIONS
****************/

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

    assert!(bit_c8 == PieceCode::BB);
    assert!(bit_e2 == PieceCode::WP);
    assert!(bit_e4 == PieceCode::ES);

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

    assert!(bit_e2 == PieceCode::ES);
    assert!(bit_e4 == PieceCode::WP);

    // Zerox88Board

    let mut zerox: Zerox88Board = Zerox88Board::new();
    let zerox_c8: PieceCode = zerox.get_square(Coord::new(2, 7));
    let mut zerox_e2: PieceCode = zerox.get_square(Coord::new(4, 1));
    let mut zerox_e4: PieceCode = zerox.get_square(Coord::new(4, 3));

    print!("0x88 board :\n{}", zerox.ascii());
    println!("0x88 c8 : {}", get_unicode_piece(zerox_c8));
    println!("0x88 e2 : {}", get_unicode_piece(zerox_e2));
    println!("0x88 e4 : {}\n", get_unicode_piece(zerox_e4));

    assert!(zerox_c8 == PieceCode::BB);
    assert!(zerox_e2 == PieceCode::WP);
    assert!(zerox_e4 == PieceCode::ES);

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

    assert!(zerox_e2 == PieceCode::ES);
    assert!(zerox_e4 == PieceCode::WP);
}
