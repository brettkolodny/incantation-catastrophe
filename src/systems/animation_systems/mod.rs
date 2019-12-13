mod bishop_animation;
mod knight_run;
mod pawn_run;
mod player_animation;
mod rook_run;

pub use self::{
    bishop_animation::BishopAnimationSystem, knight_run::KnightRunSystem, pawn_run::PawnRunSystem,
    player_animation::PlayerAnimationSystem, rook_run::RookRunSystem,
};
