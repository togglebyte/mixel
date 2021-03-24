use nightmaregl::events::{Event, EventLoop, Key, KeyState, Modifiers};
use nightmaregl::texture::{Filter, PixelType, Texture, Wrap};
use nightmaregl::{Color, Context, Pixel, Position, Renderer, Result, Size, Sprite, Viewport};

mod cursor;
use cursor::Cursor;

// -----------------------------------------------------------------------------
//     - Thoughts -
//     Fill pixels in a buffer and drain the buffer
//     on each draw call.
//
//     How do we do undo?
//     Q: How do we represent delted pixels?
//     A: Zero the pixel
// -----------------------------------------------------------------------------

const CANVAS_SIZE: Size<i32> = Size::new(16, 16);

fn run() -> Result<()> {
    let (event_loop, mut context) = Context::builder("Mixel").build()?;
    let window_size = context.window_size();
    let window_size = Size::new(window_size.width, window_size.height);
    let mut viewport = Viewport::new(Position::zero(), window_size);
    let mut renderer = Renderer::default(&mut context)?;
    renderer.pixel_size = 64.0;

    // -----------------------------------------------------------------------------
    //     - Canvas -
    // -----------------------------------------------------------------------------
    let pixel_data = [Pixel::transparent(); (CANVAS_SIZE.width * CANVAS_SIZE.height) as usize];
    let mut empty_tex = Texture::<i32>::new().with_data(bytemuck::cast_slice(&pixel_data), CANVAS_SIZE);

    let mut draw_sprite = Sprite::<i32>::new(empty_tex.size());
    // draw_sprite.z_index = 10;
    draw_sprite.position = Position::new(1, 1);

    // -----------------------------------------------------------------------------
    //     - Cursor -
    // -----------------------------------------------------------------------------
    let mut data = vec![Pixel::transparent(); (CANVAS_SIZE.width * CANVAS_SIZE.height) as usize];
    data[0].r = 255;
    data[0].a = 255;

    let cursor_texture = Texture::<i32>::new().with_data(bytemuck::cast_slice(&data), CANVAS_SIZE);
    let mut cursor_sprite = Sprite::new(cursor_texture.size());
    let mut cursor = Cursor::new();
    cursor_sprite.position = draw_sprite.position;
    cursor.position = draw_sprite.position;

    // -----------------------------------------------------------------------------
    //     - Border -
    // -----------------------------------------------------------------------------
    let border_tex = Texture::<i32>::new().with_data(
        bytemuck::cast_slice(
            &[Pixel {
                r: 20,
                g: 20,
                b: 40,
                a: 255,
            }; 34 * 34],
        ),
        [34, 34],
    );
    let mut border_sprite = Sprite::new(border_tex.size());
    // border_sprite.z_index = 11;
    border_sprite.position = (draw_sprite.position / 2) - Position::new(1, 1);

    event_loop.run(move |event| {
        match event {
            Event::KeyInput {
                key,
                state: KeyState::Pressed,
                modifiers,
            } => {
                if modifiers != Modifiers::empty() {
                    return false;
                }
                match key {
                    Key::H => { cursor.position.x -= 1; cursor.dirty = true }
                    Key::J => { cursor.position.y += 1; cursor.dirty = true }
                    Key::K => { cursor.position.y -= 1; cursor.dirty = true }
                    Key::L => { cursor.position.x += 1; cursor.dirty = true }
                    Key::Escape => { return true; }
                    _ => {}
                }

                cursor_sprite.position = cursor.position;// - Position::new(16, 16);
                cursor_sprite.position.y = -cursor_sprite.position.y;
            }
            Event::Resize(new_size) => {
                viewport.resize(new_size.cast());
            }
            Event::Draw => {
                if cursor.dirty {
                    empty_tex.write_region(
                        cursor.position,
                        Size::new(1, 1),
                        &[255; 4],
                        PixelType::Rgba,
                    );
                    cursor.dirty = false;
                }

                context.clear(Color::grey());
                // renderer.render(
                //     &border_tex,
                //     &[border_sprite.vertex_data()],
                //     &viewport,
                //     &mut context,
                // );
                renderer.render(
                    &cursor_texture,
                    &[cursor_sprite.vertex_data()],
                    &viewport,
                    &mut context,
                );
                renderer.render(
                    &empty_tex,
                    &[draw_sprite.vertex_data()],
                    &viewport,
                    &mut context,
                );
                context.swap_buffers();
            }
            _ => {}
        }

        false
    });

    Ok(())
}

fn main() {
    let result = run();
    if let Err(e) = result {
        eprintln!("{:?}", e);
    }
}
