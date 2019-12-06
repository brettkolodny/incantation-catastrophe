mod bishop_spawn;
mod knight_spawn;
mod pawn_spawn;
mod rook_spawn;

pub use self::{
    bishop_spawn::BishopSpawnSystem, knight_spawn::KnightSpawnSystem, pawn_spawn::PawnSpawnSystem,
    rook_spawn::RookSpawnSystem,
};
