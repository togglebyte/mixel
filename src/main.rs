use std::time::Instant;

// TODO: Rename Key in Input to something less generic
use nightmaregl::events::{Event, Key as WinitKey, KeyState, LoopAction};
use nightmaregl::{Color, Context, Position, Result, Size, Animation, Sprite, Renderer, Viewport, Rotation};
use nightmaregl::texture::{Wrap, Texture};

// mod canvas;
mod commands;
mod input;
mod lark;

use input::{Input, Key};
// use canvas::Canvas;
use commands::CommandInput;

fn run() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(true)
        .resizable(false)
        .with_size(Size::new(901, 733))
        .build()?;

    let window_size = context.window_size::<i32>();

    // // -----------------------------------------------------------------------------
    // //     - Canvas -
    // // -----------------------------------------------------------------------------
    // let mut canvas = Canvas::new(
    //     window_size,
    //     Size::new(32, 32),
    //     Color {
    //         b: 0.5,
    //         ..Default::default()
    //     },
    //     &mut context,
    // )?;

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
    let mut viewport = Viewport::new(Position::zero(), context.window_size());
    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size = 4;

    let texture = Texture::<f32>::from_disk("src/horrible.png")?;
    texture.wrap_x(Wrap::NoWrap);
    texture.wrap_y(Wrap::NoWrap);
    let mut sprite = {
        let mut s = Sprite::<f32>::new(texture.size());
        s.size = Size::new(16.0, 12.0);
        let position = Position::new(window_size.width as f32 / renderer.pixel_size as f32, window_size.height as f32 / renderer.pixel_size as f32) / 2.0;
        s.position = Position::new(position.x as i32 as f32, position.y as i32 as f32);

        s.size = Size::new(13.0, 7.0);
        s.texture_offset= Position::new(18.0, 4.0);
        s
    };

    // let mut animation = Animation::new(sprite, 2, 2, 32);
    // animation.should_loop = false;
    // animation.fps = 0.0;

    let mut counter = 0;

    // -----------------------------------------------------------------------------
    //     - Event loop -
    // -----------------------------------------------------------------------------
    let mut now = Instant::now();
    eventloop.run(move |event| {
        match event {
            Event::Char(c) => {
                input.update(Key::Char(c));
                commands.input(&mut input);
            }
            Event::Key {
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
                commands.render(&mut context);

                // -----------------------------------------------------------------------------
                //     - Nonsense -
                // -----------------------------------------------------------------------------
                let t = now.elapsed().as_secs_f32();
                sprite.position.x += t.sin() * 0.3;
                sprite.position.y += t.cos() * 0.3;
                sprite.rotation = Rotation::radians(t);

                let res = renderer.render(
                    &texture,
                    &vec![sprite.vertex_data()],
                    &viewport,
                    &mut context
                );

                if let Err(e) = res {
                    eprintln!("failed to render: {:?}", e);
                }

                context.swap_buffers();
                // animation.update(dt);
            }

            Event::Resize(new_size) => viewport.resize(new_size),
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
