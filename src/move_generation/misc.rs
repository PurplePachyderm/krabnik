#![allow(dead_code)]

use crate::board_representation::*;

/***************
 * MOVE ENCODING
 ***************/

/// Get the start file of a move
#[inline(always)]
pub fn get_move_start_file(mov: u32) -> u8 {
    (mov >> 29) as u8
}

/// Get the start rank of a move
#[inline(always)]
pub fn get_move_start_rank(mov: u32) -> u8 {
    ((0b111 << 26) & mov) as u8
}

/// Get the start Coord of a move
#[inline(always)]
pub fn get_move_start_coords(mov: u32) -> Coord {
    Coord::new(get_move_start_file(mov), get_move_start_rank(mov))
}

/// Get the arrival file of a move
#[inline(always)]
pub fn get_move_arrival_file(mov: u32) -> u8 {
    ((0b111 << 23) & mov) as u8
}

/// Get the arrival rank of a move
#[inline(always)]
pub fn get_move_arrival_rank(mov: u32) -> u8 {
    ((0b111 << 20) & mov) as u8
}

/// Get the arrival Coord of a move
#[inline(always)]
pub fn get_move_arrival_coords(mov: u32) -> Coord {
    Coord::new(get_move_start_file(mov), get_move_start_rank(mov))
}

/// Get moved PieceCode from a move
#[inline(always)]
pub fn get_move_piece_code(mov: u32) -> PieceCode {
    PieceCode::from_u32((0b1111 << 16) & mov)
}

/// Get arrival square PieceCode of a move
#[inline(always)]
pub fn get_move_arrival_square(mov: u32) -> PieceCode {
    PieceCode::from_u32((0b1111 << 12) & mov)
}

/// Get capture bit of a move
#[inline(always)]
pub fn get_move_capture(mov: u32) -> bool {
    ((0b1 << 15) & mov) != 0
}

/// Get en-passant capture bit of a move
#[inline(always)]
pub fn get_move_en_passant_capture(mov: u32) -> bool {
    ((0b1 << 14) & mov) != 0
}

/// Get double pawn push bit of a move
#[inline(always)]
pub fn get_move_double_pawn_push(mov: u32) -> bool {
    ((0b1 << 13) & mov) != 0
}

/// Get promotion bit of a move
#[inline(always)]
pub fn get_move_promotion(mov: u32) -> bool {
    ((0b1 << 12) & mov) != 0
}

/// Get kingside castling bit of a move
#[inline(always)]
pub fn get_move_kingside_castling(mov: u32) -> bool {
    ((0b1 << 11) & mov) != 0
}

/// Get queenside castling bit of a move
#[inline(always)]
pub fn get_move_queenside_castling(mov: u32) -> bool {
    ((0b1 << 10) & mov) != 0
}

/// Get check castling bit of a move
#[inline(always)]
pub fn get_move_check(mov: u32) -> bool {
    ((0b1 << 9) & mov) != 0
}

/// Get checkmate castling bit of a move
#[inline(always)]
pub fn get_move_checkmate(mov: u32) -> bool {
    ((0b1 << 8) & mov) != 0
}

/************************
 * MISC UTILITY FUNCTIONS
 ************************/

/// Pop (set to 0) the MSB of a bitboard (u64) and return its index
fn pop_msb(bitboard: &mut u64) {}

fn get_player_bitboard(bitboard: &BitBoard, player: Player) -> u64 {
    if player == Player::White {
        bitboard.main_boards[0]
            & bitboard.main_boards[1]
            & bitboard.main_boards[2]
            & bitboard.main_boards[3]
            & bitboard.main_boards[4]
            & bitboard.main_boards[5]
    } else {
        bitboard.main_boards[6]
            & bitboard.main_boards[7]
            & bitboard.main_boards[8]
            & bitboard.main_boards[9]
            & bitboard.main_boards[10]
            & bitboard.main_boards[11]
    }
}

fn get_all_pieces_bitboard(bitboard: &BitBoard, player: Player) -> u64 {
    bitboard.main_boards[0]
        & bitboard.main_boards[1]
        & bitboard.main_boards[2]
        & bitboard.main_boards[3]
        & bitboard.main_boards[4]
        & bitboard.main_boards[5]
        & bitboard.main_boards[6]
        & bitboard.main_boards[7]
        & bitboard.main_boards[8]
        & bitboard.main_boards[9]
        & bitboard.main_boards[10]
        & bitboard.main_boards[11]
}

// Generate a bitboard of all our own pinned pieces (by checking which squares are attacked
// by the opponent's long range pieces)
// TODO
