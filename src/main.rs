use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{UiBundle, RenderUi},
    utils::application_root_dir,
};

mod components;
mod resources;
mod states;
mod systems;
mod utility;

use crate::states::GameplayState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let binding_path = format!(
        "{}/resources/bindings_config.ron",
        application_root_dir().unwrap().to_str().unwrap()
    );
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.1, 0.1, 0.1, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
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
        //Spawn systems
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
        .with(
            systems::RookSpawnSystem {
                spawn_timer: 15.,
                time_since_spawn: 0.,
            },
            "rook_spawn",
            &[],
        )
        .with(
            systems::KnightSpawnSystem {
                spawn_timer: 20.,
                time_since_spawn: 0.,
            },
            "knight_spawn",
            &[],
        )
        //Move systems
        .with(systems::KnightMoveSystem, "knight_move", &["knight_spawn"])
        .with(systems::PawnMoveSystem, "pawn_move", &["pawn_spawn"])
        .with(systems::RookMoveSystem, "rook_move", &["rook_spawn"])
        .with(
            systems::BishopMoveSystem { move_timer: 4. },
            "bishop_move",
            &["bishop_spawn", "player_death"],
        )
        //Shoot systems
        .with(
            systems::BishopShootSystem,
            "bishop_shoot",
            &["player_death"],
        )
        .with(systems::EnemyHitSystem, "enemy_hit", &["player_shoot"])
        .with(systems::KnightPushSystem, "knight_push", &["knight_move"])
        .with(
            systems::PlayerHitSystem::default(),
            "player_hit",
            &["pawn_move", "bishop_shoot", "player_death"],
        );

    let mut game = Application::new("./", GameplayState {}, game_data)?;
    game.run();

    Ok(())
}
