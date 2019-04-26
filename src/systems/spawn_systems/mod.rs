mod bishop_spawn;
mod pawn_spawn;
mod rook_spawn;

pub use self::{
    bishop_spawn::BishopSpawnSystem, pawn_spawn::PawnSpawnSystem, rook_spawn::RookSpawnSystem,
};
