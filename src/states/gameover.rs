use amethyst::assets::Loader;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::resources::ScoreResource;

pub struct GameOverState;

impl SimpleState for GameOverState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let score = world.read_resource::<ScoreResource>().score;

        initialize_score_text(world, score);
    }

}

fn initialize_score_text(world: &mut World, score: u32) {
    let score_string = format!("SCORE:{}", score);
    println!("Your score was: {}!", score);

    let font = world.read_resource::<Loader>().load(
        "textures/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "SCORE".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        200.,
        50.,
    );

    world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font,
            score_string.to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .build();
}
