use std::time::Instant;

use anyhow::Result;
use nightmaregl::events::{Event, LoopAction};
use nightmaregl::{Color, Context, Size};

mod application;
mod canvas;
mod commands;
mod input;

use commands::Command;
pub use application::{App, Mode};

fn run() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(false)
        .resizable(false)
        .with_size(Size::new(901, 733))
        .build()?;

    // -----------------------------------------------------------------------------
    //     - App -
    // -----------------------------------------------------------------------------
    let mut app = App::new(&mut context)?;

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let mut now = Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Char(c) => {
                if let Command::Quit = app.update_input(c, &mut context) {
                    return LoopAction::Quit;
                }
                app.input(c);
            }

            Event::Key { key, state } => app.update_modifier(key, state),

            Event::Draw(dt) => {
                context.clear(Color::grey());
                app.render(&mut context);
                context.swap_buffers();
            }

            Event::Resize(new_size) => app.resize(new_size),
            _ => {}
        }

        LoopAction::Continue
    });
}

fn main() {
    let result = run();
    if let Err(e) = result {
        eprintln!("Application error: {:?}", e);
    }
}
