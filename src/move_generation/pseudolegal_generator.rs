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
fn precompute_pl_p_moves(player: Player) -> [u64; 64] {
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
fn precompute_pl_n_moves() -> [u64; 64] {
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
fn precompute_pl_k_moves() -> [u64; 64] {
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
// pieces, efficiently generating moves for them is a bit trickier. To that effect, we use
// "magic bitboards". See this blog post for a good explanation of the following code :
// <https://rhysre.net/fast-chess-move-generation-with-magic-bitboards.html>

fn gen_pl_r_moves(coord : Coord) -> u64 {
    let mut pseudolegal_moves : u64 = 0;

    let file_mask : u64 = 0x80_80_80_80_80_80_80_80;
    let rank_mask : u64 = 0xff_00_00_00_00_00_00_00;

    // Shift masks and apply them on rook mask
    pseudolegal_moves |= file_mask >> coord.f;
    pseudolegal_moves |= rank_mask >> coord.r;

    // Substract the square on which the rook itself is located
    pseudolegal_moves &= !(a1_bitboard!() >> (8*coord.r) >> coord.f);

    pseudolegal_moves
}

fn gen_pl_b_moves(coord : Coord) -> u64 {
    let mut pseudolegal_moves : u64 = 0;

    let bishop_bitboard: u64 = a1_bitboard!() >> (8 * coord.r) >> coord.f;

    // Precompute all SW-NE diagonals, and see on which one (if any) the piece is located
    let sw_ne_diags : [u64; 15] = [
        0x00_00_00_00_00_00_00_80, // a8
        0x00_00_00_00_00_00_80_40, // a7-b8
        0x00_00_00_00_00_80_40_20, // a6-c8
        0x00_00_00_00_80_40_20_10, // a5-d8
        0x00_00_00_80_40_20_10_08, // a4-e8
        0x00_00_80_40_20_10_08_04, // a3-f8
        0x00_80_40_20_10_08_04_02, // a2-g8
        0x80_40_20_10_08_04_02_01, // a1-h8 (long one)
        0x40_20_10_08_04_02_01_00, // b1-h7
        0x20_10_08_04_02_01_00_00, // c1-h6
        0x10_08_04_02_01_00_00_00, // d1-h5
        0x08_04_02_01_00_00_00_00, // e1-h3
        0x04_02_01_00_00_00_00_00, // f1-h3
        0x02_01_00_00_00_00_00_00, // g1-h2
        0x01_00_00_00_00_00_00_00, // h1
    ];

    for i in 0..15 {
        if bishop_bitboard & sw_ne_diags[i] != 0 {
            // We found our SW-NE diagonal, add it to the pseudolegal_moves
            pseudolegal_moves |= sw_ne_diags[i];
            break;
        }
    }

    // Same for NW-SE diagonals
    let nw_se_diags : [u64; 15] = [
        0x00_00_00_00_00_00_00_01, // h8
        0x00_00_00_00_00_00_01_02, // h7-g8
        0x00_00_00_00_00_01_02_04, // h6-f8
        0x00_00_00_00_01_02_04_08, // h5-e8
        0x00_00_00_01_02_04_08_10, // h4-d8
        0x00_00_01_02_04_08_10_20, // h3-c8
        0x00_01_02_04_08_10_20_40, // h2-b8
        0x01_02_04_08_10_20_40_80, // h1-a8 (long one)
        0x02_04_08_10_20_40_80_00, // g1-a7
        0x04_08_10_20_40_80_00_00, // f1-a6
        0x08_10_20_40_80_00_00_00, // e1-a5
        0x10_20_40_80_00_00_00_00, // d1-a3
        0x20_40_80_00_00_00_00_00, // c1-a3
        0x40_80_00_00_00_00_00_00, // b1-a2
        0x80_00_00_00_00_00_00_00, // a1
    ];

    for i in 0..15 {
        if bishop_bitboard & nw_se_diags[i] != 0 {
            // We found our NW-SE diagonal, add it to the pseudolegal_moves
            pseudolegal_moves |= nw_se_diags[i];
            break;
        }
    }


    // Substract the square on which the bishop itself is located
    pseudolegal_moves &= !bishop_bitboard;

    pseudolegal_moves
}


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
