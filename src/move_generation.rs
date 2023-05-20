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
 *
 * The move generator itself is split into two main parts :
 * - the pseudo-legal generator, which can precompute all potential moves for a given piece
 * on all the 64 squares
 * - the legal move generator, which retrieves the precomputed pseudolegal moves, and
 * filters out the one that are illegal in a given position. All the legal moves in a
 * position can later be evaluated in a negamax algorithm.
 *
 * Finally, the MovableBoard trait contains utility methods to make/unmake moves on a
 * Position.
 */

pub mod legal_generator;
pub mod misc;
pub mod movable_board;
pub mod pseudolegal_generator;

pub use legal_generator::*;
pub use misc::*;
pub use movable_board::*;
pub use pseudolegal_generator::*;
