use nightmaregl::{Position, Color, Pixel, Pixels, Size, Texture, Sprite};

pub struct Canvas {
    pub texture: Texture<i32>,
    pub sprite: Sprite<i32>,
    pix_buf: Pixels,
}

impl Canvas {
    pub fn new(size: Size<usize>, color: Color) -> Self {
        let pixels = Pixels::from_pixel(color.into(), size);
        let texture = Texture::default_with_data(size.cast::<i32>(), pixels.as_bytes());
        let mut sprite = Sprite::new(texture.size());
        sprite.z_index = 10;

        Self {
            texture,
            sprite,
            pix_buf: Pixels::from_size(Size::new(1, 1)),
        }
    }

    pub fn move_by(&mut self, position: Position<i32>) {
        self.sprite.position += position;
    }

    pub fn draw(&mut self, draw_at: Position<i32>) {
        let draw_at = Position::new(draw_at.x, 31 - draw_at.y);
        let pixel = Color::white().into();
        self.pix_buf.push(pixel);
        self.texture.write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();

    }
    
}
