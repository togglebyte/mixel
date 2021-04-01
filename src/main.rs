use nightmaregl::events::{Event, Key as WinitKey, KeyState, LoopAction};
use nightmaregl::{Color, Context, Position, Result, Size, Animation, Sprite, Renderer, Viewport};
use nightmaregl::texture::{Wrap, Texture};

mod canvas;
mod commands;
mod input;

use input::{Input, Key};
use canvas::Canvas;
use commands::CommandInput;

fn run() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(true)
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
    //     - Animation -
    // -----------------------------------------------------------------------------
    let viewport = Viewport::new(Position::zero(), context.window_size());
    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size *= 4.0;

    let texture = Texture::<f32>::from_disk("src/horrid.png")?;
    texture.wrap_x(Wrap::NoWrap);
    texture.wrap_y(Wrap::NoWrap);
    let sprite = {
        let mut s = Sprite::<f32>::new(texture.size());
        s.size = Size::new(17.0, 13.0);
        s
    };

    let mut animation = Animation::new(sprite, 2, 2, 32);
    animation.should_loop = false;
    animation.fps = 0.0;

    let mut counter = 0;

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    eventloop.run(move |event| {
        match event {
            Event::Char(c) => {
                input.update(Key::Char(c));
                commands.input(&mut input);
            }
            Event::KeyInput {
                key,
                state: KeyState::Pressed,
            } => {
                input.update(Key::Key(key));
                // canvas.input(&input);
                commands.input(&mut input);
            }

            Event::Draw(dt) => {
                context.clear(Color::grey());
                // canvas.render(&mut context);
                // commands.render(&mut context);

                renderer.render(
                    &texture,
                    &vec![animation.sprite.vertex_data()],
                    &viewport,
                    &mut context
                );
                context.swap_buffers();
                animation.update(dt);
            }

            Event::Resize(_new_size) => {}
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
