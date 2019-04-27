use amethyst::core::transform::Transform;
use amethyst::ecs::Entity;
use amethyst::input::is_key_down;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection, SpriteRender, SpriteSheetHandle, VirtualKeyCode};

use crate::components::{Background, GameplayItem, Player, Size};
use crate::resources::{CurrentState, Hearts, PlayerResource, ScoreResource, SpriteSheet};
use crate::states::{GameOverState, PauseState};
use crate::utility::{load_sprite_sheet, GAMEPLAY_AREA_HEIGHT, GAMEPLAY_AREA_WIDTH};

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
        world.add_resource(ScoreResource { score: 0 });

        let spritesheet = world
            .read_resource::<SpriteSheet>()
            .sprite_sheet
            .clone()
            .unwrap();

        initialize_hearts(world, spritesheet.clone());
        initialize_arena(world, spritesheet.clone());
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
            return Trans::Push(Box::new(GameOverState));
        }
        Trans::None
    }
}

fn initialize_camera(_world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    _world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            GAMEPLAY_AREA_WIDTH,
            0.0,
            GAMEPLAY_AREA_HEIGHT,
        )))
        .with(transform)
        .build();
}

pub fn initialize_arena(_world: &mut World, _sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(GAMEPLAY_AREA_WIDTH / 2., GAMEPLAY_AREA_HEIGHT / 2., -100.);
    local_transform.set_scale(1.5, 1.5, 1.);

    let sprite_render = {
        SpriteRender {
            sprite_sheet: _sprite_sheet_handle,
            sprite_number: 0,
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
    local_transform.set_xyz(100., 1000., 0.);
    local_transform.set_scale(0.1, 0.1, 1.);

    let sprite_render = {
        SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 2,
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
                .build(),
        );

        local_transform.set_x(local_transform.translation().x + 50.);
    }

    world.add_resource(Hearts { hearts });
}
