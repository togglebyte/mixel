use std::path::Path;

use nightmaregl::texture::{Format, Texture};
use nightmaregl::{
    Color, Context, Framebuffer, Pixel, Pixels, Position, Renderer, Result, Size, Sprite,
    VertexData, Viewport,
};

use crate::commands::Command;
use crate::input::{Action, InputHandler};
use crate::Mode;

struct SaveBuffer {
    fb: Framebuffer,
    texture: Texture<i32>,
    sprite: Sprite<i32>,
    renderer: Renderer<VertexData>,
}

impl SaveBuffer {
    fn new(size: Size<i32>, context: &mut Context) -> Result<Self> {
        let fb = Framebuffer::new();
        let texture = Texture::<i32>::new()
            .with_format(Format::Rgba)
            .with_no_data(size);

        let sprite = Sprite::new(texture.size());
        fb.attach_texture(&texture);

        let renderer = Renderer::default(context)?;

        let inst = Self {
            fb,
            texture,
            sprite,
            renderer,
        };

        Ok(inst)
    }

    fn resize(&mut self, size: Size<i32>) {
        self.sprite.size = size;
        self.texture = Texture::<i32>::new()
            .with_format(Format::Rgba)
            .with_no_data(size);
    }

    fn save(&self, sprite: &Sprite<i32>, textures: &[Texture<i32>], viewport: &Viewport, path: impl AsRef<Path>, context: &mut Context) {
        self.fb.bind();
        textures.into_iter().for_each(|t| {
            self.renderer.render(
                t,
                &[sprite.vertex_data()],
                viewport,
                context,
            );
        });
        self.fb.unbind();
    }
}

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
    mode: Mode,
    fb: Option<(Framebuffer, Texture<i32>, Sprite<i32>)>,
    should_save: bool,
}

impl Canvas {
    pub fn new(window_size: Size<i32>, size: Size<i32>, context: &mut Context) -> Result<Self> {
        let viewport = Viewport::new(Position::zero(), window_size);

        let background_color: Color = Pixel {
            r: 12,
            g: 34,
            b: 56,
            a: 255,
        }
        .into();
        eprintln!("{:?}", background_color);

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

        let mut renderer = Renderer::<VertexData>::default(context)?;
        renderer.pixel_size = 16;

        let mut sprite = Sprite::new(texture.size());
        sprite.z_index = 10;
        sprite.position =
            window_size.to_vector() / 2 / renderer.pixel_size as i32 - sprite.size.to_vector() / 2;

        let mut cursor_sprite = sprite;
        cursor_sprite.z_index = 9;

        let mut inst = Self {
            texture,
            cursor_texture,
            sprite,
            cursor_sprite,
            pix_buf: Pixels::from_size(Size::new(1, 1)),
            viewport,
            renderer,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            fb: None,
            should_save: false,
        };

        // Position the cursor or it won't be
        // drawn until it moves fors the first time.
        inst.move_cursor(Position::zero());

        Ok(inst)
    }

    pub fn render(&mut self, context: &mut Context) {
        let vertex_data = [self.sprite.vertex_data()];
        let cursor_vertex_data = [self.cursor_sprite.vertex_data()];

        // Horrible saving code
        if let Some(ref mut fb) = self.fb {
            let (fb, fb_texture, fb_sprite) = fb;
            fb.bind();

            // Draw the pixels
            self.renderer
                .render(&self.texture, &vertex_data, &self.viewport, context);

            fb.unbind();

            // Draw the cursor
            self.renderer.render(
                &self.cursor_texture,
                &cursor_vertex_data,
                &self.viewport,
                context,
            );

            // Draw the framebuffer texture
            self.renderer.render(
                fb_texture,
                &[fb_sprite.vertex_data()],
                &self.viewport,
                context,
            );

            if self.should_save {
                self.should_save = false;
                fb_texture.write_to_disk("test.png");
            }
        }

        // self.renderer
        //     .render(&self.texture, &vertex_data, &self.viewport, context);

        // let vertex_data = [self.cursor_sprite.vertex_data()];
        // self.renderer
        //     .render(&self.cursor_texture, &vertex_data, &self.viewport, context);
    }

    pub fn move_cursor(&mut self, move_by: Position<i32>) {
        // Put the old pixel back
        let draw_at = self.cursor.position;
        self.pix_buf.push(Pixel::transparent());
        self.cursor_texture
            .write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();

        // Move the cursor
        self.cursor.position += move_by;

        // Render cursor
        let draw_at = self.cursor.position;
        let pixel = self.cursor.color;
        self.pix_buf.push(pixel);
        self.cursor_texture
            .write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
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

    pub fn input(&mut self, c: char, mode: Mode, input: &InputHandler) {
        match mode {
            Mode::Command => return,
            Mode::Normal | Mode::Visual | Mode::Insert => {
                let action = input.to_action(c, mode);
                match action {
                    Some(Action::Left) => self.move_cursor(Position::new(-1, 0)),
                    Some(Action::Right) => self.move_cursor(Position::new(1, 0)),
                    Some(Action::Up) => self.move_cursor(Position::new(0, -1)),
                    Some(Action::Down) => self.move_cursor(Position::new(0, 1)),
                    _ => {}
                }
            }
        }

        if let Mode::Insert = mode {
            self.draw();
        }
    }

    // -----------------------------------------------------------------------------
    //     - TODO -
    //     * On save use the framebuffer rendere for all renders
    //     * Set the framebuffer texture to the same size as the textures to save
    //     * Set the framebuffer sprite to the same size as the textures to save
    // -----------------------------------------------------------------------------
    pub fn exec(&mut self, command: Command) {
        let pixels = Pixels::from_pixel(
            Pixel {
                r: 0,
                ..Default::default()
            },
            self.texture.size().cast(),
        );

        let fb = Framebuffer::new();
        let fb_texture = Texture::<i32>::new()
            .with_format(Format::Rgba)
            .with_data(pixels.as_bytes(), self.texture.size());

        let sprite = Sprite::new(fb_texture.size());
        fb.attach_texture(&fb_texture);

        self.fb = Some((fb, fb_texture, sprite));
        self.should_save = true;
    }
}
