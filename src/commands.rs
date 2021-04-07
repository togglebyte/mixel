use nightmaregl::text::{Text, WordWrap};
use nightmaregl::{
    Context, Pixel, Pixels, Position, Renderer, Result, Size, Sprite, Texture, VertexData, Viewport,
};

use crate::input::InputHandler;
use crate::Mode;

const FONT_SIZE: f32 = 18.0;

// -----------------------------------------------------------------------------
//     - Commands -
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub enum Command {
    Quit,
    Write(String),
    Noop,
}

// -----------------------------------------------------------------------------
//     - Cursor -
// -----------------------------------------------------------------------------
struct Cursor {
    sprite: Sprite<f32>,
    texture: Texture<f32>,
}

impl Cursor {
    fn new(size: Size<usize>) -> Self {
        let pixels = Pixels::from_pixel(Pixel::white(), size);
        let texture = Texture::default_with_data(size.cast(), pixels.as_bytes());
        let sprite = Sprite::new(texture.size());

        Self { sprite, texture }
    }
}

// -----------------------------------------------------------------------------
//     - Command input -
// -----------------------------------------------------------------------------
pub struct CommandInput {
    text: Text,
    text_renderer: Renderer<VertexData>,
    cursor_renderer: Renderer<VertexData>,
    viewport: Viewport,
    enabled: bool,
    visible_text: String,
    text_buffer: String,
    cursor: Cursor,
}

impl CommandInput {
    pub fn new(context: &mut Context) -> Result<Self> {
        let font_path =
            "/usr/share/fonts/nerd-fonts-complete/TTF/Hack Regular Nerd Font Complete Mono.ttf";
        let win_size = context.window_size();

        let viewport = Viewport::new(Position::zero(), win_size);

        let mut cursor_renderer = Renderer::default(context)?;
        cursor_renderer.pixel_size = 1;

        let mut cursor = Cursor::new(Size::new(FONT_SIZE as usize, FONT_SIZE as usize * 2));
        cursor.sprite.position = Position::new(0.0, FONT_SIZE / 1.5);

        let mut text_renderer = Renderer::default_font(context)?;
        text_renderer.pixel_size = 1;

        let mut text = Text::from_path(font_path, FONT_SIZE, WordWrap::NoWrap, context)?;
        text.position(Position::new(0.0, FONT_SIZE / 1.5));

        let mut inst = Self {
            text,
            text_renderer,
            cursor_renderer,
            viewport,
            enabled: false,
            visible_text: String::new(),
            text_buffer: String::new(),
            cursor,
        };

        Ok(inst)
    }

    fn update_text(&mut self) {
        self.text.set_text(&self.visible_text);

        while self.text.caret().x + self.cursor.sprite.size.width
            > self.viewport.size().width as f32
        {
            if self.visible_text.is_empty() {
                return;
            }
            self.visible_text.drain(..1);
            self.text.set_text(&self.visible_text);
        }

        self.cursor.sprite.position = Position::new(self.text.caret().x, FONT_SIZE / 3.0);
    }

    pub fn render(&self, context: &mut Context, mode: Mode) {
        if !mode.command_mode() {
            return;
        }

        let res = self.text_renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            context,
        );

        if let Err(e) = res {
            eprintln!("text renderer: {:?}", e);
        }

        let res = self.cursor_renderer.render(
            &self.cursor.texture,
            &[self.cursor.sprite.vertex_data()],
            &self.viewport,
            context,
        );

        if let Err(e) = res {
            eprintln!("cursor renderer: {:?}", e);
        }

    }

    pub fn input(&mut self, c: char, mode: Mode, input: &InputHandler) -> Command {
        match mode {
            Mode::Command => {}
            Mode::Insert | Mode::Normal | Mode::Visual => return Command::Noop,
        }

        match c {
            // Backspace
            '\u{8}' => {
                self.text_buffer.pop();
                self.visible_text = self.text_buffer.clone();
                self.update_text();
            }
            // Enter
            '\r' => {
                self.visible_text.clear();
                let raw_command = self.text_buffer.drain(..).collect();
                let command = parse_command(raw_command);
                self.text.set_text(String::new());
                self.enabled = false;
                return command;
            }
            // Character input
            c => {
                if !c.is_control() {
                    self.visible_text.push(c);
                    self.text_buffer.push(c);
                    self.update_text();
                }
            }
            _ => {}
        }

        Command::Noop
    }
}

fn parse_command(s: String) -> Command {
    if s == ":q" {
        return Command::Quit;
    }

    if s.starts_with(":w") {
        return Command::Write(
            s.split_whitespace()
                .skip(1)
                .take(1)
                .collect::<String>()
                .trim()
                .to_string()
        );
    }

    Command::Noop
}
