use amethyst::assets::Loader;
use amethyst::core::transform::Transform;
use amethyst::ecs::Join;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::renderer::camera::Projection;
use amethyst::renderer::Camera;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::utils::application_root_dir;

use crate::components::TitleItem;
use crate::resources::CurrentState;
use crate::states::GameplayState;
use crate::utility::{GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

pub struct TitleState;

impl SimpleState for TitleState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        world.register::<TitleItem>();
        initialize_camera(world);
        initialize_title_text(world);
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                let mut game_state = _data.world.write_resource::<CurrentState>();
                game_state.resume();
                return Trans::Switch(Box::new(GameplayState));
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let title_state_items = world.read_storage::<TitleItem>();
        let entities = world.entities();

        for (entity, _) in (&*entities, &title_state_items).join() {
            entities
                .delete(entity)
                .expect("Unable to delete title menu entitiy");
        }
    }
}

fn initialize_title_text(world: &mut World) {
    let app_root = application_root_dir().unwrap();
    let font_path = app_root.join("textures").join("square.ttf");

    let font = world.read_resource::<Loader>().load(
        font_path.to_str().unwrap(),
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let title_transform = UiTransform::new(
        "TITLE".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        0.,
        1.,
        1000.,
        80.,
    );

    let start_transform = UiTransform::new(
        "START".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.,
        -100.,
        1.,
        1000.,
        50.,
    );

    world
        .create_entity()
        .with(title_transform)
        .with(UiText::new(
            font.clone(),
            "INCANTATION CATASTROPHE".to_string(),
            [1., 1., 1., 1.],
            75.,
        ))
        .with(TitleItem)
        .build();

    world
        .create_entity()
        .with(start_transform)
        .with(UiText::new(
            font,
            "PRESS SPACE TO START".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .with(TitleItem)
        .build();
}

fn initialize_camera(_world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    _world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.,
            GAMEPLAY_AREA_WIDTH,
            0.,
            GAMEPLAY_AREA_HEIGHT,
            0.,
            20.,
        )))
        .with(transform)
        .build();
}
