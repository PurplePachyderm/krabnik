#![allow(dead_code)]

/*******************************
 * MOVES GENERATOR (PSEUDOLEGAL)
 *******************************/

/// From a position, return a vector of possible moves for the current ply
pub fn generate_moves(position: &Position) -> Vec<u32> {
    // TODO
    Vec::new()
}

/// Get a bitboard of all our own pinned pieces (by checking which squares are attacked by
/// the opponent's long range pieces)
// TODO

/// Get a bitboard of all pseudo-legal moves for a pawn located on the coord square
/// (and controlled by the specified player)
fn get_pseudolegal_pawn_moves(coord: Coord, player: Player) -> u64 {
    // Assert that pawn is not behind his start rank, or on his promotion rank
    assert!(
        coord.r != 0 && coord.r != 7,
        "Invalid position: pawn ({}, {}) behind its start rank, or on its promotion rank",
        coord.f,
        coord.r
    );

    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    if player == Player::White {
        /* Generate forward pawn pushes */
        // Generate simple and double pawn push
        if coord.r == 1 {
            moves_bitboard |= 0b10000001 << 55 >> (16 + coord.f);
        }
        // Generate only a single pawn push
        else {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r + 1) + coord.f);
        }

        /* Generate potential captures (will be filtered out later if none is possible) */
        // We can capture on the left everywhere but on the a-file
        if coord.f != 0 {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r + 1) + coord.f - 1);
        }
        // We can capture on the right everywhere but on the h-file
        if coord.f != 7 {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r + 1) + coord.f + 1);
        }
    }
    // Move gen for black player is the same, but moves are in the other direction
    else {
        /* Generate forward pawn pushes */
        // Generate simple and double pawn push
        if coord.r == 6 {
            moves_bitboard |= 0b10000001 << (31 + coord.f);
        }
        // Generate only a single pawn push
        else {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r - 1) + coord.f);
        }

        /* Generate potential captures (will be filtered out later if none is possible) */
        // We can capture on the left everywhere but on the a-file
        if coord.f != 0 {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r - 1) + coord.f - 1);
        }
        // We can capture on the right everywhere but on the h-file
        if coord.f != 7 {
            moves_bitboard |= a1_bitboard!() >> (8 * (coord.r - 1) + coord.f + 1);
        }
    }

    moves_bitboard
}

/// Get a bitboard of all pseudo-legal moves for a knight located on the coord square
fn get_pseudolegal_knight_moves(coord: Coord, player: Player) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // Create bitboard with the knight position, used to generate possible moves
    let knight_bitboard: u64 = a1_bitboard!() >> 8 * coord.r >> coord.f;

    // Generate moves two ranks down
    if coord.r >= 2 {
        // One file left
        if coord.f >= 1 {
            moves_bitboard |= knight_bitboard << 17;
        }
        // One file right
        if coord.f <= 6 {
            moves_bitboard |= knight_bitboard << 15;
        }
    }

    // Generate moves one rank down
    if coord.r >= 1 {
        // Two files left
        if coord.f >= 1 {
            moves_bitboard |= knight_bitboard << 10;
        }
        // Two files right
        if coord.f <= 6 {
            moves_bitboard |= knight_bitboard << 6;
        }
    }

    // Generate moves one rank up
    if coord.r <= 5 {
        // Two files left
        if coord.f >= 1 {
            moves_bitboard |= knight_bitboard >> 6;
        }
        // Two files right
        if coord.f <= 6 {
            moves_bitboard |= knight_bitboard >> 10;
        }
    }

    // Generate moves two ranks up
    if coord.r <= 6 {
        // One file left
        if coord.f >= 1 {
            moves_bitboard |= knight_bitboard >> 15;
        }
        // One file right
        if coord.f <= 6 {
            moves_bitboard |= knight_bitboard >> 17;
        }
    }

    moves_bitboard
}

/// Get a bitboard of all pseudo-legal moves for a bishop located on the coord square
fn get_pseudolegal_bishop_moves(coord: Coord, player: Player) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // TODO

    moves_bitboard
}

/// Get a bitboard of all pseudo-legal moves for a rook located on the coord square
fn get_pseudolegal_rook_moves(coord: Coord, player: Player) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // TODO

    moves_bitboard
}

/// Get a bitboard of all pseudo-legal moves for a queen located on the coord square
fn get_pseudolegal_queen_moves(coord: Coord, player: Player) -> u64 {
    get_pseudolegal_bishop_moves(coord, player) | get_pseudolegal_rook_moves(coord, player)
}

/// Get a bitboard of all pseudo-legal moves for a king located on the coord square
fn get_pseudolegal_king_moves(coord: Coord, player: Player) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // TODO

    moves_bitboard
}
