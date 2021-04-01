use nightmaregl::{
    Color, Context, Pixel, Pixels, Position, Renderer, Result, Size, Sprite, Texture, VertexData, Viewport,
};

struct Cursor {
    position: Position<i32>,
    color: Pixel,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: Position::new(16, 16),
            color: Pixel {
                r: 255,
                ..Pixel::default()
            },
        }
    }
}

pub struct Canvas {
    pub texture: Texture<i32>,
    pub renderer: Renderer<VertexData>,
    sprite: Sprite<i32>,
    cursor_sprite: Sprite<i32>,
    cursor_texture: Texture<i32>,
    pix_buf: Pixels,
    viewport: Viewport,
    cursor: Cursor,
}

impl Canvas {
    pub fn new(
        window_size: Size<i32>,
        size: Size<i32>,
        background_color: Color,
        context: &mut Context,
    ) -> Result<Self> {
        let viewport = Viewport::new(
            Position::zero(),
            Size::new(window_size.width, window_size.height),
        );

        // Main canvas texture
        let texture = {
            let pixels = Pixels::from_pixel(background_color.into(), size.cast());
            Texture::default_with_data(size, pixels.as_bytes())
        };

        // Cursor texture
        let cursor_texture = {
            let pixels = Pixels::from_pixel(Pixel::transparent(), size.cast());
            Texture::default_with_data(size, pixels.as_bytes())
        };

        let mut sprite = Sprite::new(texture.size());
        sprite.z_index = 10;
        sprite.position -= size.to_vector() / 2;

        let mut cursor_sprite = sprite;
        cursor_sprite.z_index = 9;

        let mut renderer = Renderer::<VertexData>::default(context)?;
        renderer.pixel_size = 32.0;

        let inst = Self {
            texture,
            cursor_texture,
            sprite,
            cursor_sprite,
            pix_buf: Pixels::from_size(Size::new(1, 1)),
            viewport,
            renderer,
            cursor: Cursor::new(),
        };

        Ok(inst)
    }

    pub fn render(&mut self, context: &mut Context) {
        let vertex_data = [self.sprite.vertex_data()];
        self.renderer.render(
            &self.texture,
            &vertex_data,
            &self.viewport,
            context,
        );

        let vertex_data = [self.cursor_sprite.vertex_data()];
        self.renderer.render(
            &self.cursor_texture,
            &vertex_data,
            &self.viewport,
            context,
        );

    }

    // pub fn move_by(&mut self, position: Position<i32>) {
    //     self.sprite.position += position;
    // }

    pub fn move_cursor(&mut self, pos: Position<i32>) {
        // // Put the old pixel back
        let draw_at = self.cursor.position;
        self.pix_buf.push(Pixel::transparent());
        self.cursor_texture.write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();

        // Move the cursor
        self.cursor.position += pos;

        // Render cursor
        let draw_at = self.cursor.position;
        let pixel = self.cursor.color;
        self.pix_buf.push(pixel);
        self.cursor_texture.write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();
    }

    pub fn draw(&mut self) {
        // Put a new pixel in place
        let draw_at = self.cursor.position;
        let pixel = Color::white().into();
        self.pix_buf.push(pixel);
        self.texture
            .write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();
    }

}
