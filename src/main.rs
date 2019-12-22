use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod components;
mod resources;
mod states;
mod systems;
mod utility;

use crate::states::TitleState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let binding_path = resources.join("bindings_config.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config).with_clear([0., 0., 0., 1.0]),
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
        )
        .with(systems::PawnRunSystem, "pawn_run", &["pawn_spawn"])
        .with(systems::RookRunSystem, "rook_run", &["rook_spawn"])
        .with(
            systems::BishopAnimationSystem,
            "bishop_animation",
            &["bishop_spawn", "bishop_shoot"],
        )
        .with(
            systems::PlayerAnimationSystem,
            "player_animation",
            &["player_move"],
        )
        .with(
            systems::PlayerProjectileAnimationSystem,
            "player_projectile_animation",
            &["player_shoot"],
        )
        .with(
            systems::BishopProjectileAnimationSystem,
            "bishop_projectile_animation",
            &["bishop_shoot"],
        )
        .with(systems::PotionPickupSystem, "potion_pickup", &[])
        .with(systems::KnightRunSystem, "knight_run", &["knight_spawn"]);

    let mut game = Application::new(app_root, TitleState {}, game_data)?;
    game.run();

    Ok(())
}
