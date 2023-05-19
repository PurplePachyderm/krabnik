/*
 * The move_generation module contains functions to generate an make/unmake moves on a
 * Position.
 *
 * Moves are stored in u32 integers using the following patern (in big-endian) :
 * - 6 bits for starting square(3 bits for file + 3 bits for rank)
 * - 6 bits for arrival square (3 bits for file + 3 bits for rank)
 * - 4 bits for the PieceCode of the moved piece
 * - 4 bits for the PieceCode of the arrival square
 * - 8 bits for special moves encoding
 * - 4 bits for optional promotion piece code
 *
 * The special moves encoding byte is encoded as follows :
 * - Capture bit
 * - En-passant capture bit
 * - Double pawn push bit
 * - Promotion bit
 * - Kingside castling bit
 * - Queenside castling bit
 * - Check bit
 * - Checkmate bit
 */

pub mod misc;

pub use misc::*;
