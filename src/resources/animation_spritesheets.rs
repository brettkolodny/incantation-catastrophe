use amethyst::renderer::sprite::SpriteSheetHandle;
use std::collections::HashMap;

pub struct AnimationSpriteSheets {
    pub sprite_sheets: HashMap<String, SpriteSheetHandle>,
}

impl Default for AnimationSpriteSheets {
    fn default() -> Self {
        AnimationSpriteSheets {
            sprite_sheets: HashMap::new(),
        }
    }
}
