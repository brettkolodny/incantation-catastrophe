mod bishop_move;
mod pawn_move;
mod player_move;
mod rook_move;

pub use self::{
    bishop_move::BishopMoveSystem, pawn_move::PawnMoveSystem, player_move::PlayerMoveSystem, rook_move::RookMoveSystem,
};
