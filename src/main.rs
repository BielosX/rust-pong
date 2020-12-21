extern crate sdl2;

use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

struct Context {
    canvas: WindowCanvas,
    event_pump: EventPump
}

fn create_canvas(sdl_context: &Sdl) -> Result<WindowCanvas, String> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust-pong", 800, 600)
        .position_centered()
        .build().map_err(|x| -> String {x.to_string()})?;
    window.into_canvas().build().map_err(|x| -> String {x.to_string()})
}

fn create_context() -> Result<Context, String> {
    let sdl_context = sdl2::init()?;
    let canvas = create_canvas(&sdl_context)?;
    let event_pump = sdl_context.event_pump()?;
    Ok(Context {canvas, event_pump})
}

fn draw(context: &mut Context) {
    let mut quit = false;
    while !quit {
        context.canvas.set_draw_color(Color::RGB(100,100,0));
        context.canvas.clear();
        for event in context.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
                _ => {}
            }
        }
        context.canvas.present();
    }
}

fn main() -> () {
    let init_result = create_context();
    match init_result {
        Ok(mut context) => draw(&mut context),
        Err(err) => println!("Error occured during context init: {}", err),
    }
}