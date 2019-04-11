use amethyst::renderer::SpriteSheetHandle;

pub struct SpriteSheet {
    pub sprite_sheet: Option<SpriteSheetHandle>,
}

impl Default for SpriteSheet {
    fn default() -> Self {
        SpriteSheet { sprite_sheet: None }
    }
}
