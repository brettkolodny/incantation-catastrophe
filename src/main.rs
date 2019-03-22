extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::renderer::{DrawFlat, PosNormTex};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::application_root_dir;

mod incantation_catastrophe;
mod states;
mod systems;

use crate::states::GameplayState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PlayerMoveSystem, "player_move", &[]);
    let mut game = Application::new("./", GameplayState, game_data)?;

    game.run();

    Ok(())
}
