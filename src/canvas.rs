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

    fn save(
        &self,
        sprite: &Sprite<i32>,
        textures: &[Texture<i32>],
        viewport: &Viewport,
        path: impl AsRef<Path>,
        context: &mut Context,
    ) -> Result<()> {
        self.fb.bind();
        textures.into_iter().for_each(|t| {
            self.renderer
                .render(t, &[self.sprite.vertex_data()], viewport, context);
        });

        let res = self.texture.write_to_disk(path.as_ref());

        self.fb.unbind();

        res
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
    textures: Vec<Texture<i32>>,
    layer: usize,
    renderer: Renderer<VertexData>,
    sprite: Sprite<i32>,
    cursor_sprite: Sprite<i32>,
    cursor_texture: Texture<i32>,
    pix_buf: Pixels,
    viewport: Viewport,
    cursor: Cursor,
    mode: Mode,
    save_buffer: SaveBuffer,
}

impl Canvas {
    pub fn new(window_size: Size<i32>, size: Size<i32>, context: &mut Context) -> Result<Self> {
        let viewport = Viewport::new(Position::zero(), window_size);

        let background_color: Pixel = Pixel {
            r: 12,
            g: 34,
            b: 56,
            a: 255,
        };

        // Main canvas texture
        let texture = {
            eprintln!("{:?}", background_color);
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

        let save_buffer = SaveBuffer::new(size, context)?;

        let mut inst = Self {
            textures: vec![texture],
            layer: 0,
            cursor_texture,
            sprite,
            cursor_sprite,
            pix_buf: Pixels::from_size(Size::new(1, 1)),
            viewport,
            renderer,
            cursor: Cursor::new(),
            mode: Mode::Normal,
            save_buffer,
        };

        // Position the cursor or it won't be
        // drawn until it moves fors the first time.
        inst.move_cursor(Position::zero());

        Ok(inst)
    }

    // -----------------------------------------------------------------------------
    //     - Render -
    // -----------------------------------------------------------------------------
    pub fn render(&mut self, context: &mut Context) {
        let vertex_data = [self.sprite.vertex_data()];

        self.textures.iter().for_each(|t| {
            let res = self
                .renderer
                .render(t, &vertex_data, &self.viewport, context);

            if let Err(e) = res {
                eprintln!("canvas render: {:?}", e);
            }
        });

        let res = self.renderer.render(
            &self.cursor_texture,
            &[self.cursor_sprite.vertex_data()],
            &self.viewport,
            context,
        );

        if let Err(e) = res {
            eprintln!("cursor render: {:?}", e);
        }
    }

    // -----------------------------------------------------------------------------
    //     - Move cursor -
    // -----------------------------------------------------------------------------
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
        self.cursor_texture.write_region(
            draw_at,
            Size::new(1, 1),
            self.pix_buf.as_bytes()
        );
        self.pix_buf.clear();
    }

    // -----------------------------------------------------------------------------
    //     - Draw cursor -
    // -----------------------------------------------------------------------------
    pub fn draw(&mut self) {
        // Put a new pixel in place
        let draw_at = self.cursor.position;
        let pixel = Color::white().into();
        self.pix_buf.push(pixel);
        let texture = &self.textures[self.layer];
        texture.write_region(draw_at, Size::new(1, 1), self.pix_buf.as_bytes());
        self.pix_buf.clear();
    }

    // -----------------------------------------------------------------------------
    //     - Input handling -
    // -----------------------------------------------------------------------------
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
    pub fn exec(&mut self, command: Command, context: &mut Context) {
        match command {
            Command::Write(path) => {
                let res = self.save_buffer.save(
                    &self.sprite,
                    &self.textures,
                    &self.viewport,
                    path,
                    context,
                );

                eprintln!("{:?}", res);
            }
            _ => {}
        }
    }
}
