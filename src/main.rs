use nightmaregl::events::{Event, EventLoop, Key, KeyState, LoopAction, Modifiers};
use nightmaregl::text::Text;
use nightmaregl::texture::{Filter, Texture, Wrap};
use nightmaregl::{
    Color, Context, Pixel, Position, Renderer, Result, Size, Sprite, VertexData, Viewport,
};

mod canvas;
mod cursor;

use canvas::Canvas;
use cursor::Cursor;

// -----------------------------------------------------------------------------
//     - Thoughts -
//     Fill pixels in a buffer and drain the buffer
//     on each draw call.
//
//     How do we do undo?
//     Q: How do we represent deleted pixels?
//     A: Zero the pixel
// -----------------------------------------------------------------------------

fn run() -> Result<()> {
    let (eventloop, mut context) = Context::builder("Mixel: the modal pixel editor")
        .vsync(true)
        .build()?;
    let mut window_size = context.window_size::<i32>();
    let mut viewport = Viewport::new(
        Position::zero(),
        Size::new(window_size.width, window_size.height),
    );
    let mut viewport_text = Viewport::new(
        Position::zero(),
        Size::new(window_size.width as f32, window_size.height as f32),
    );

    let mut editor_renderer = Renderer::<VertexData>::default(&mut context)?;
    editor_renderer.pixel_size = 32.0;

    let red = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    // -----------------------------------------------------------------------------
    //     - Canvas -
    // -----------------------------------------------------------------------------
    let canvas_size = Size::new(32, 32);
    let mut canvas = Canvas::new(
        canvas_size,
        Color {
            b: 0.5,
            ..Default::default()
        },
    );
    canvas.move_by(-canvas.sprite.size.to_vector() / 2);

    // -----------------------------------------------------------------------------
    //     - Cursor -
    // -----------------------------------------------------------------------------
    let mut cursor = Cursor::new(red);

    // -----------------------------------------------------------------------------
    //     - Text / Commands -
    // -----------------------------------------------------------------------------
    let mut text_renderer = Renderer::default_font(&mut context)?;
    text_renderer.pixel_size = 1.0;

    let font_size = 18.0;
    // let mut text = Text::new("/usr/share/fonts/TTF/Hack-Regular.ttf", font_size, &context, window_size.width as u32)?;
    let mut text = Text::new("/usr/share/fonts/TTF/Hack-Regular.ttf", font_size, &context, 200)?;
    let mut buf = format!("Hello, world");
    text.set_text(&buf)?;
    // text.set_position(-Position::new(window_size.width as f32 / 2.0, window_size.height as f32 / 2.0 - font_size * 2.0));

    eventloop.run(move |event| {
        match event {
            Event::Char(c) => {
                if c.is_control() {
                    return LoopAction::Continue;
                }
                buf.push(c);
                text.set_text(&buf);
            }
            Event::KeyInput {
                key,
                state: KeyState::Pressed,
            } => match key {
                Key::Back => {
                    buf.pop();
                    text.set_text(&buf);
                }
                Key::Escape => return LoopAction::Quit,
                Key::H => {
                    cursor.sprite.position += Position::new(-1, 0);
                }
                Key::J => {
                    cursor.sprite.position += Position::new(0, -1);
                }
                Key::K => {
                    cursor.sprite.position += Position::new(0, 1);
                }
                Key::L => {
                    cursor.sprite.position += Position::new(1, 0);
                }
                Key::Colon => {}
                Key::Key1 => editor_renderer.pixel_size += 2.0,
                Key::Key2 => editor_renderer.pixel_size -= 2.0,
                Key::Space => canvas.draw(cursor.position()),
                _ => { }
            },
            Event::Draw => {
                context.clear(Color::grey());

                editor_renderer.render(
                    &canvas.texture,
                    &[canvas.sprite.vertex_data()],
                    &viewport,
                    &mut context,
                );

                editor_renderer.render(
                    &cursor.texture,
                    &[cursor.offset_sprite(canvas.sprite.size.to_vector()).vertex_data()],
                    &viewport,
                    &mut context,
                );

                text_renderer.render(
                    &text.texture(),
                    &text.vertex_data(),
                    &viewport_text,
                    &mut context,
                );

                context.swap_buffers();
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
