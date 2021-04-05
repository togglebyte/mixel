use anyhow::Result;
use nightmaregl::events::{Key, KeyState};
use nightmaregl::{Context, Size};

use crate::canvas::Canvas;
use crate::commands::CommandInput;
use crate::input::{Input, InputHandler};

#[derive(Debug, Copy, Clone)]
pub enum Mode {
    Insert,
    Normal,
    Visual,
    Command,
}

impl Mode {
    pub fn command_mode(&self) -> bool {
        match self {
            Mode::Command => true,
            _ => false,
        }
    }
}

pub struct App {
    pub mode: Mode,
    input: InputHandler,
    command_input: CommandInput,
    canvas: Canvas,
}

impl App {
    pub fn new(context: &mut Context) -> Result<Self> {
        let window_size = context.window_size::<i32>();

        // -----------------------------------------------------------------------------
        //     - Input handler -
        // -----------------------------------------------------------------------------
        let mut input = InputHandler::new()?;

        // -----------------------------------------------------------------------------
        //     - Command input -
        // -----------------------------------------------------------------------------
        let mut command_input = CommandInput::new(context)?;

        // -----------------------------------------------------------------------------
        //     - Canvas -
        // -----------------------------------------------------------------------------
        let mut canvas = Canvas::new(window_size, Size::new(32, 32), context)?;

        let inst = Self {
            mode: Mode::Normal,
            input,
            command_input,
            canvas,
        };

        Ok(inst)
    }

    pub fn resize(&mut self, new_size: Size<u32>) {}

    pub fn update_input(&mut self, c: char) {
        self.input.update(c);

        match self.mode {
            Mode::Normal => {
                // Possibly enter insert mode
                // Possibly enter visual mode
                match c {
                    'i' => self.mode = Mode::Insert,
                    ':' => self.mode = Mode::Command,
                    _ => {}
                }
            }
            Mode::Visual => {
                // Possibly enter insert mode
                // Possibly back to normal mode
                match c {
                    ':' => self.mode = Mode::Command,
                    // Esc
                    '\u{1b}' => {
                        self.canvas.input(c, self.mode, &self.input);
                        self.mode = Mode::Normal;
                    }
                    _ => {}
                }
            }
            Mode::Insert => {
                // Esc will change back to normal
                match c {
                    // Esc
                    '\u{1b}' => {
                        self.canvas.input(c, self.mode, &self.input);
                        self.mode = Mode::Normal;
                    }
                    _ => {}
                }
            }
            Mode::Command => {
                // Esc will change back to normal
                // Submit command needs to go back to normal
                match c {
                    // Enter
                    '\r' => {
                        self.command_input.input(c, self.mode, &self.input);
                        self.mode = Mode::Normal;
                    }
                    // Esc
                    '\u{1b}' => {
                        self.command_input.input(c, self.mode, &self.input);
                        self.mode = Mode::Normal;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render(&mut self, context: &mut Context) {
        self.canvas.render(context);
        self.command_input.render(context, self.mode);
    }

    pub fn update_modifier(&mut self, key: Key, state: KeyState) {
        self.input.update_modifier(key, state);
    }

    pub fn input(&mut self, c: char) {
        self.command_input.input(c, self.mode, &self.input);
        self.canvas.input(c, self.mode, &self.input);
    }
}
