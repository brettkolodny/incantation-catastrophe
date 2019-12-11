use amethyst::core::{math::Vector3, transform::Transform};
use amethyst::ecs::{Entity, Join};
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::renderer::{camera::Projection, sprite::SpriteSheetHandle, Camera, SpriteRender};

use crate::components::{Background, GameplayItem, Player, Size};
use crate::resources::{CurrentState, Hearts, PlayerResource, ScoreResource, SpriteSheet};
use crate::states::{GameOverState, PauseState};
use crate::utility::{
    load_sprite_sheet, BACKGROUND_SPRITE_NUMBER, CIRCLE_SPRITE_NUMBER, GAMEPLAY_AREA_HEIGHT,
    GAMEPLAY_AREA_WIDTH, HEART_SPRITE_NUMBER,
};

pub struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let spritesheet_handle = Some(load_sprite_sheet(
            world,
            "textures/incantation_catastrophe.png",
            "textures/incantation_catastrophe.ron",
        ));

        world.write_resource::<SpriteSheet>().sprite_sheet = Some(spritesheet_handle.unwrap());
        world.insert(ScoreResource { score: 0 });

        let spritesheet = world
            .read_resource::<SpriteSheet>()
            .sprite_sheet
            .clone()
            .unwrap();

        initialize_hearts(world, spritesheet.clone());
        initialize_arena(world, spritesheet.clone());
        initialize_circle(world, spritesheet.clone());
        initialize_camera(world);
        Player::initialize(world, spritesheet.clone());
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                let mut game_state = data.world.write_resource::<CurrentState>();
                game_state.pause();
                return Trans::Push(Box::new(PauseState));
            }
        }
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let player = data.world.read_resource::<PlayerResource>();
        let mut state = data.world.write_resource::<CurrentState>();
        if let None = player.player {
            state.pause();
            return Trans::Switch(Box::new(GameOverState));
        }
        Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        let world = _data.world;
        let gameplay_state_items = world.read_storage::<GameplayItem>();
        let entities = world.entities();

        for (entity, _) in (&*entities, &gameplay_state_items).join() {
            entities
                .delete(entity)
                .expect("Unable to delete gameplay menu entitiy");
        }
    }
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

pub fn initialize_arena(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        GAMEPLAY_AREA_WIDTH / 2.0,
        GAMEPLAY_AREA_HEIGHT / -2.0,
        -2.,
    );
    local_transform.set_scale(Vector3::new(2.0, 2.0, 1.));

    let sprite_render = {
        SpriteRender {
            sprite_sheet: _sprite_sheet_handle,
            sprite_number: BACKGROUND_SPRITE_NUMBER,
        }
    };

    _world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(GameplayItem)
        .with(Background)
        .with(Size::new(1000., 1000.))
        .build();
}

pub fn initialize_circle(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        GAMEPLAY_AREA_WIDTH / 2.0,
        GAMEPLAY_AREA_HEIGHT / -2.0,
        -1.,
    );
    local_transform.set_scale(Vector3::new(1.15, 1.15, 1.));

    let sprite_render = {
        SpriteRender {
            sprite_sheet: _sprite_sheet_handle,
            sprite_number: CIRCLE_SPRITE_NUMBER,
        }
    };

    _world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(GameplayItem)
        .with(Background)
        .with(Size::new(1000., 1000.))
        .build();
}

pub fn initialize_hearts(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(100., -50., 0.);
    local_transform.set_scale(Vector3::new(2.0, 2.0, 1.0));

    let sprite_render = {
        SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: HEART_SPRITE_NUMBER,
        }
    };

    let mut hearts: Vec<Entity> = Vec::new();

    for _ in 0..10 {
        let transform = local_transform.clone();
        hearts.push(
            world
                .create_entity()
                .with(sprite_render.clone())
                .with(transform)
                .with(Background)
                .with(GameplayItem)
                .build(),
        );

        local_transform.set_translation_x(local_transform.translation().x + 50.);
    }

    world.insert(Hearts { hearts });
}
