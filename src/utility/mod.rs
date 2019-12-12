use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    formats::texture::ImageFormat, sprite::SpriteSheetHandle, SpriteSheet, SpriteSheetFormat,
    Texture,
};

use crate::components::Size;

pub const GAMEPLAY_AREA_WIDTH: f32 = 1280.;
pub const GAMEPLAY_AREA_HEIGHT: f32 = 960.;

pub const BISHOP_SPRITE_NUMBER: usize = 2;
pub const BACKGROUND_SPRITE_NUMBER: usize = 1;
pub const CIRCLE_SPRITE_NUMBER: usize = 0;
pub const HEART_SPRITE_NUMBER: usize = 7;
pub const PLAYER_SHOT_SPRITE_NUMBER: usize = 8;
pub const BISHOP_SHOT_SPRITE_NUMBER: usize = 9;
pub const PLAYER_SPRITE_NUMBER: usize = 5;

pub const RADIUS: f32 = 422.;

pub fn load_sprite_sheet(
    _world: &mut World,
    _sprite_sheet: &str,
    _sprite_sheet_ron: &str,
) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = _world.read_resource::<Loader>();
        let texture_storage = _world.read_resource::<AssetStorage<Texture>>();
        loader.load(_sprite_sheet, ImageFormat::default(), (), &texture_storage)
    };

    let loader = _world.read_resource::<Loader>();
    let sprite_sheet_store = _world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        _sprite_sheet_ron,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

pub fn did_hit(object_1: (&Size, &Transform), object_2: (&Size, &Transform)) -> bool {
    let object_1_width = object_1.0.width;
    let object_1_height = object_1.0.height;
    let object_1_x = object_1.1.translation().x;
    let object_1_y = object_1.1.translation().y;

    let object_2_width = object_2.0.width;
    let object_2_height = object_2.0.height;
    let object_2_x = object_2.1.translation().x;
    let object_2_y = object_2.1.translation().y;

    let l1 = (object_1_x - object_1_width, object_1_y - object_1_height);

    let r1 = (object_1_x + object_1_width, object_1_y + object_1_height);

    let l2 = (object_2_x - object_2_width, object_2_y - object_2_height);

    let r2 = (object_2_x + object_2_width, object_2_y + object_2_height);

    if l1.0 > r2.0 || l2.0 > r1.0 {
        return false;
    }

    if l1.1 > r2.1 || l2.1 > r1.1 {
        return false;
    }

    true
}
