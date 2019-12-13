use amethyst::assets::Loader;
use amethyst::ecs::Join;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::utils::application_root_dir;

use crate::components::PauseItem;
use crate::resources::CurrentState;

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        world.register::<PauseItem>();
        initialize_pause_text(world);
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut game_state = _data.world.write_resource::<CurrentState>();
                game_state.resume();
                return Trans::Pop;
            }
        }
        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let pause_state_items = world.read_storage::<PauseItem>();
        let entities = world.entities();

        for (entity, _) in (&*entities, &pause_state_items).join() {
            entities
                .delete(entity)
                .expect("Unable to delete pause menu entitiy");
        }
    }
}

fn initialize_pause_text(world: &mut World) {
    let app_root = application_root_dir().unwrap();
    let font_path = app_root.join("textures").join("square.ttf");

    let font = world.read_resource::<Loader>().load(
        font_path.to_str().unwrap(),
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "PAUSE".to_string(),
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
            "PAUSE".to_string(),
            [1., 1., 1., 1.],
            50.,
        ))
        .with(PauseItem)
        .build();
}
