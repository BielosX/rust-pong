extern crate sdl2;

use std::time::Instant;

use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

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
    let mut y: f64 = 10.0;
    let mut rect = Rect::new(10, y as i32, 100, 100);
    let mut delta: f64 = 0.0;
    let mut velocity: f64 = 0.0;
    while !quit {
        let now = Instant::now();
        context.canvas.set_draw_color(Color::RGB(100,100,0));
        context.canvas.clear();
        for event in context.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => velocity = -300.0,
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => velocity = 300.0,
                Event::KeyUp { keycode: Some(Keycode::Up), ..} => velocity = 0.0,
                Event::KeyUp { keycode: Some(Keycode::Down), ..} => velocity = 0.0,
                _ => {}
            }
        }
        if rect.y() == 0 && velocity < 0.0 {
            y = y;
        }
        else if (rect.y() + rect.height() as i32) == 600 && velocity > 0.0 {
            y = y; 
        }
        else {
            y += delta * velocity;
        }
        rect.set_y(y as i32);
        context.canvas.set_draw_color(Color::RGB(0, 0, 255));
        context.canvas.fill_rect(rect).unwrap();
        context.canvas.present();
        delta = (now.elapsed().as_nanos() as f64) / 1000000000.0;
    }
}

fn main() -> () {
    let init_result = create_context();
    match init_result {
        Ok(mut context) => draw(&mut context),
        Err(err) => println!("Error occured during context init: {}", err),
    }
}