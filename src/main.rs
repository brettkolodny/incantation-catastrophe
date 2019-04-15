extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ALPHA,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::application_root_dir;

mod components;
mod resources;
mod states;
mod systems;
mod utility;

use crate::states::GameplayState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new().with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()),
    );

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PlayerDeathSystem, "player_death", &[])
        .with(systems::PlayerMoveSystem, "player_move", &["player_death"])
        .with(
            systems::BoundarySystem,
            "boundary",
            &["player_move", "player_death"],
        )
        .with(
            systems::PlayerShootSystem { is_shooting: false },
            "player_shoot",
            &["player_move", "player_death"],
        )
        .with(
            systems::ProjectileMoveSystem,
            "projectile_move",
            &["player_shoot", "player_death"],
        )
        .with(
            systems::PawnSpawnSystem {
                spawn_timer: 1.5,
                time_since_spawn: 0.,
            },
            "pawn_spawn",
            &["player_death"],
        )
        .with(
            systems::BishopSpawnSystem {
                spawn_timer: 8.,
                time_since_spawn: 0.,
            },
            "bishop_spawn",
            &["player_death"],
        )
        .with(systems::PawnMoveSystem, "pawn_move", &["pawn_spawn"])
        .with(
            systems::BishopMoveSystem { move_timer: 4. },
            "bishop_move",
            &["bishop_spawn", "player_death"],
        )
        .with(
            systems::BishopShootSystem,
            "bishop_shoot",
            &["player_death"],
        )
        .with(systems::EnemyHitSystem, "enemy_hit", &["player_shoot"])
        .with(
            systems::PlayerHitSystem::default(),
            "player_hit",
            &["pawn_move", "bishop_shoot", "player_death"],
        );
    let mut game = Application::new("./", GameplayState {}, game_data)?;

    game.run();

    Ok(())
}
