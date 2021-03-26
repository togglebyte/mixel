use nightmaregl::{Size, Sprite, Texture, Position, Pixels, Pixel, Color};

use crate::canvas::Canvas;

pub struct Cursor {
    pub dirty: bool,
    pub texture: Texture<i32>,
    pub sprite: Sprite<i32>,
}

impl Cursor {
    pub fn new(color: Color) -> Self {
        let size = Size::new(1, 1);
        let pixels  = Pixels::from_pixel(color.into(), size);
        let texture = Texture::default_with_data(size.cast::<i32>(), pixels.as_bytes());
        let mut sprite = Sprite::new(texture.size());
        sprite.z_index = 9;

        Self {
            dirty: false,
            texture,
            sprite,
        }
    }

    pub fn offset(&self, offset: Position<i32>) -> Position<i32> {
        self.sprite.position - offset / 2
    }

    pub fn offset_sprite(&self, offset: Position<i32>) -> Sprite<i32> {
        let mut sprite = self.sprite;
        sprite.position = self.offset(offset);
        sprite
    }

    pub fn position(&self) -> Position<i32> {
        self.sprite.position
    }
}
