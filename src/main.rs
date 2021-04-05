use std::time::Instant;

use nightmaregl::events::{Event, Key as WinitKey, KeyState, LoopAction};
use nightmaregl::{Color, Context, Position, Result, Size, Animation, Sprite, Renderer, Viewport, Rotation};
use nightmaregl::texture::{Wrap, Texture};

mod canvas;
mod commands;
mod input;

use input::Input;
use canvas::Canvas;
use commands::CommandInput;

fn run() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(false)
        .resizable(false)
        .with_size(Size::new(901, 733))
        .build()?;

    let window_size = context.window_size::<i32>();

    // -----------------------------------------------------------------------------
    //     - Canvas -
    // -----------------------------------------------------------------------------
    let mut canvas = Canvas::new(
        window_size,
        Size::new(32, 32),
        Color {
            b: 0.5,
            ..Default::default()
        },
        &mut context,
    )?;

    // -----------------------------------------------------------------------------
    //     - Command input -
    // -----------------------------------------------------------------------------
    let mut commands = CommandInput::new(&mut context)?;

    // -----------------------------------------------------------------------------
    //     - Input -
    // -----------------------------------------------------------------------------
    let mut input = Input::new();

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let mut now = Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Char(c) => {
                input.update(c);
                commands.input(&mut input);
                canvas.input(&mut input);
            }
            Event::Key {
                key,
                state,
            } => {
                input.update_modifier(key, state);
            }

            Event::Draw(dt) => {
                context.clear(Color::grey());
                canvas.render(&mut context);
                commands.render(&mut context);
                context.swap_buffers();
            }

            Event::Resize(new_size) => {
                // TODO: resize canvas and input
            }
            _ => {}
        }

        LoopAction::Continue
    });
}

fn main() {
    let result = run();
    if let Err(e) = result {
        eprintln!("{:?}", e);
    }
}
