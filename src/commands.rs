use nightmaregl::{Position, VertexData, Result, Context, Renderer, Viewport};
use nightmaregl::text::{WordWrap, Text};
use nightmaregl::events::Key as WinitKey;

use crate::input::{Input, Action, Key};

const FONT_SIZE: f32 = 24.0;

pub struct CommandInput {
    text: Text,
    renderer: Renderer<VertexData>,
    viewport: Viewport,
    enabled: bool,
    visible_text: String,
    text_buffer: String,
}

impl CommandInput {
    pub fn new(context: &mut Context) -> Result<Self> {
        let font_path = "/usr/share/fonts/nerd-fonts-complete/TTF/Hack Regular Nerd Font Complete Mono.ttf";
        let win_size = context.window_size();

        let viewport = Viewport::new(Position::zero(), win_size);

        let text = {
            let mut t = Text::from_path(font_path, FONT_SIZE, WordWrap::NoWrap, context)?;

            let x = -win_size.width / 2;
            let y = -win_size.height / 2 + FONT_SIZE as i32;

            let pos = Position::new(x, y);

            t.position(pos.cast());

            t
        };

        let mut renderer = Renderer::default_font(context)?;
        renderer.pixel_size = 1.0;

        let mut inst = Self {
            text,
            renderer,
            viewport,
            enabled: false,
            visible_text: String::new(),
            text_buffer: String::new(),
        };

        Ok(inst)
    }

    pub fn input(&mut self, input: &mut Input) {
        match input.action() {
            None => {},
            Some(Action::CommandInput) if !self.enabled => self.enabled = true,
            None | Some(_) if !self.enabled => return,
            Some(Action::CloseCommandInput) => {
                self.enabled = false;
                self.text_buffer.clear();
                self.visible_text.clear();
                return;
            }
            Some(_) => { }
        }

        if !self.enabled {
            return
        }

        let key = input.take();
        match key {
            Key::Char(c) => {
                if c.is_control() {
                    return;
                }

                if !self.text.fits(c, self.viewport.size().width as u32) {
                    self.visible_text.drain(..1);
                }

                self.visible_text.push(c);
                self.text_buffer.push(c);
                self.text.set_text(&self.visible_text);
            }
            Key::Key(WinitKey::Back) => {
                self.text_buffer.pop();
                self.visible_text = self.text_buffer.clone();
                self.text.set_text(&self.visible_text);
            }
            Key::Key(WinitKey::Return) => {
                self.visible_text.clear();
                self.text_buffer.clear();
                self.text.set_text(String::new());
            }
            Key::Empty => return,
            _ => {}
        }
    }

    pub fn render(&self, context: &mut Context) {
        if !self.enabled {
            return;
        }

        self.renderer.render(
            self.text.texture(),
            &self.text.vertex_data(),
            &self.viewport,
            context
        );
    }
}
