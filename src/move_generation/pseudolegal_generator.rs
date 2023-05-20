#![allow(dead_code)]

use crate::board_representation::*;

/*************************************************
 * PSEUDOLEGAL MOVE GENERATOR (non sliding pieces)
 *************************************************/

/// Generate a bitboard of all pseudo-legal moves for a pawn located on the coord square
/// (and controlled by the specified player). Used during pseudolegal moves precomputation.
fn gen_pl_p_moves(player: Player, coord: Coord) -> u64 {
    // Assert that pawn is not behind his start rank, or on his promotion rank
    assert!(
        coord.r != 0 && coord.r != 7,
        "Invalid position: pawn ({}, {}) behind its start rank, or on its promotion rank",
        coord.f,
        coord.r
    );

    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // Create a bitboard with the pawn position, used to generate possible moves
    let pawn_bitboard: u64 = a1_bitboard!() >> (8 * coord.r) >> coord.f;

    if player == Player::White {
        /* Generate forward pawn pushes */
        // Generate simple and double pawn push
        if coord.r == 1 {
            moves_bitboard |= 0b10000001 << 55 >> (16 + coord.f);
        }
        // Generate only a single pawn push
        else {
            moves_bitboard |= pawn_bitboard >> 8;
        }

        /* Generate potential captures */
        // We can capture on the left everywhere but on the a-file
        if coord.f != 0 {
            moves_bitboard |= pawn_bitboard >> 7;
        }
        if coord.f != 7 {
            // We can capture on the right everywhere but on the h-file
            moves_bitboard |= pawn_bitboard >> 9;
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
            moves_bitboard |= pawn_bitboard << 8;
        }

        /* Generate potential captures */
        // We can capture on the left everywhere but on the a-file
        if coord.f != 0 {
            moves_bitboard |= pawn_bitboard << 9;
        }
        // We can capture on the right everywhere but on the h-file
        if coord.f != 7 {
            moves_bitboard |= pawn_bitboard << 7;
        }
    }

    moves_bitboard
}

/// Generate a bitboard of all pseudo-legal moves for a knight located on the coord square.
/// Used during pseudolegal moves precomputation.
fn gen_pl_n_moves(coord: Coord) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // Create a bitboard with the knight position, used to generate possible moves
    let knight_bitboard: u64 = a1_bitboard!() >> (8 * coord.r) >> coord.f;

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

/// Generate a bitboard of all pseudo-legal moves for a king located on the coord square.
/// Used during pseudolegal moves precomputation.
fn gen_pl_k_moves(coord: Coord) -> u64 {
    // Zero initialize the moves bitboard
    let mut moves_bitboard: u64 = 0;

    // Create a bitboard with the king position, used to generate possible moves
    let king_bitboard: u64 = a1_bitboard!() >> (8 * coord.r) >> coord.f;

    // Generate the 3 moves to the left
    if coord.f != 0 {
        // Move left
        moves_bitboard |= king_bitboard << 1;

        // Move top left
        if coord.r != 7 {
            moves_bitboard |= king_bitboard >> 7;
        }

        // Move bottom left
        if coord.r != 0 {
            moves_bitboard |= king_bitboard << 9;
        }
    }

    // Generate the 3 moves to the right
    if coord.f != 7 {
        // Move right
        moves_bitboard |= king_bitboard >> 1;

        // Move top right
        if coord.r != 7 {
            moves_bitboard |= king_bitboard >> 9;
        }

        // Move bottom right
        if coord.r != 0 {
            moves_bitboard |= king_bitboard << 7;
        }
    }

    // Move up
    if coord.f != 7 {
        moves_bitboard |= king_bitboard >> 8;
    }
    // Move down
    if coord.f != 0 {
        moves_bitboard |= king_bitboard << 8;
    }

    moves_bitboard
}

/// Precompute pseudolegal pawn moves for all squares, and return them in an array of
/// bitboards of size 64
pub fn precompute_pl_p_moves(player: Player) -> [u64; 64] {
    let mut pseudolegal_moves: [u64; 64] = [0; 64];
    for i in 0..8 {
        for j in 0..8 {
            pseudolegal_moves[i * 8 + j] = gen_pl_p_moves(player, Coord::new(j as u8, i as u8));
        }
    }
    pseudolegal_moves
}

/// Precompute pseudolegal knight moves for all squares, and return them in an array of
/// bitboards of size 64
pub fn precompute_pl_n_moves() -> [u64; 64] {
    let mut pseudolegal_moves: [u64; 64] = [0; 64];
    for i in 0..8 {
        for j in 0..8 {
            pseudolegal_moves[i * 8 + j] = gen_pl_n_moves(Coord::new(j as u8, i as u8));
        }
    }
    pseudolegal_moves
}

/// Precompute pseudolegal king moves for all squares, and return them in an array of
/// bitboards of size 64
pub fn precompute_pl_k_moves() -> [u64; 64] {
    let mut pseudolegal_moves: [u64; 64] = [0; 64];
    for i in 0..8 {
        for j in 0..8 {
            pseudolegal_moves[i * 8 + j] = gen_pl_k_moves(Coord::new(j as u8, i as u8));
        }
    }
    pseudolegal_moves
}

/*********************************************
 * PSEUDOLEGAL MOVE GENERATOR (sliding pieces)
 *********************************************/

// NOTE Because sliding/long-range pieces can get their line of sight blocked by other
// pieces, efficiently generating moves for them is a bit trickier. For that, we use
// "magic bitboards". See this SO answer for a good explanation of the following code :
// <https://stackoverflow.com/a/30862064>

// TODO

/**************************
 * PSEUDOLEGAL LOOKUP TABLE
 **************************/

#[derive(Debug)]
pub struct PLMoveLUT {
    pub white_p_lut: [u64; 64],
    pub black_p_lut: [u64; 64],
    pub n_lut: [u64; 64],
    pub k_lut: [u64; 64],
}

impl Default for PLMoveLUT {
    fn default() -> PLMoveLUT {
        PLMoveLUT {
            white_p_lut: precompute_pl_p_moves(Player::White),
            black_p_lut: precompute_pl_p_moves(Player::Black),
            n_lut: precompute_pl_n_moves(),
            k_lut: precompute_pl_k_moves(),
        }
    }
}
