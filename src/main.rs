extern crate sdl2;
extern crate nalgebra;

use std::time::Instant;

use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;

use nalgebra::Vector2;

pub type Vect = Vector2<f32>;

fn reflection(vec: &Vect, normal: &Vect) -> Vect {
    let coef = vec.dot(normal) * 2.0;
    -1.0 * (coef * normal - vec)
}

struct Context {
    canvas: WindowCanvas,
    event_pump: EventPump
}

struct Rectangle {
    x: f32,
    y: f32,
    width: u32,
    height: u32
}

impl Rectangle {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        let rect = Rect::new(self.x as i32, self.y as i32, self.width, self.height);
        canvas.set_draw_color(Color::WHITE);
        canvas.fill_rect(rect).unwrap()
    }
}

struct Player {
    rect: Rectangle,
    norm: Vect
}

fn approx_equal (a: f32, b: f32) -> bool {
    let p = 0.000001;
    if (a - b).abs() < p {
        true
    }
    else {
        false
    }
}

impl Player {
    fn top_collision(&self) -> bool {
        approx_equal(self.rect.y, 0.0) || self.rect.y < 0.0
    }

    fn bottom_collision(&self) -> bool {
        let bottom = self.rect.y + self.rect.height as f32;
        approx_equal(bottom, 600.0) || bottom > 600.0
    }

    pub fn move_down(&mut self, delta_time: f32) {
        let velocity: f32 = 300.0;
        if !self.bottom_collision() {
            self.rect.y += delta_time * velocity;
        }
    }

    pub fn move_up(&mut self, delta_time: f32) {
        let velocity: f32 = -300.0;
        if !self.top_collision() {
            self.rect.y += delta_time * velocity;
        }
    }
    
    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        self.rect.draw(canvas)
    }

    pub fn right_x(&self) -> f32 {
        self.rect.x + self.rect.width as f32
    }
}

struct Ball {
    rect: Rectangle,
    velocity: Vect
}

#[derive(Debug)]
struct Pair {
    first: f32,
    second: u32
}

impl Pair {
    pub fn new(f: f32, s: u32) -> Pair {
        Pair {first: f, second: s}
    }
}

impl Ball {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.rect.draw(canvas)
    }
    
    pub fn move_ball(&mut self, delta_time: f32) {
        self.rect.x += delta_time * self.velocity.x;
        self.rect.y += delta_time * self.velocity.y;
    }

    pub fn collision(&self, player: &Player) -> bool {
        let mut horizontal = [Pair::new(self.rect.x, 0),
            Pair::new(self.rect.x + self.rect.width as f32, 0),
            Pair::new(player.rect.x, 1),
            Pair::new(player.rect.x + player.rect.width as f32, 1)];
        let mut vertical = [Pair::new(self.rect.y, 0),
            Pair::new(self.rect.y + self.rect.height as f32, 0),
            Pair::new(player.rect.y, 1),
            Pair::new(player.rect.y + player.rect.height as f32, 1)];
        horizontal.sort_by(|a,b| (a.first as i32).cmp(&(b.first as i32)));
        vertical.sort_by(|a,b| (a.first as i32).cmp(&(b.first as i32)));
        let horizontal_ok;
        let vertical_ok;
        match horizontal {
            [Pair {first: _, second: 0}, Pair {first: _, second: 0}, _, _] => horizontal_ok = true,
            [Pair {first: _, second: 1}, Pair {first: _, second: 1}, _, _] => horizontal_ok = true,
            _ => horizontal_ok = false,
        }
        match vertical {
            [Pair {first: _, second: 0}, Pair {first: _, second: 0}, _, _] => vertical_ok = true,
            [Pair {first: _, second: 1}, Pair {first: _, second: 1}, _, _] => vertical_ok = true,
            _ => vertical_ok = false,
        }
        !horizontal_ok && !vertical_ok
    }

    pub fn calc_velocity(&mut self, first_player: &Player, second_player: &Player) {
        if self.collision(first_player) {
            self.velocity = reflection(&self.velocity, &first_player.norm)
        }
        if self.collision(second_player) {
            self.velocity = reflection(&self.velocity, &second_player.norm)
        }
    }
}

enum Border {
    Upper { norm: Vect, width: i32 },
    Lower { norm: Vect, bottom: i32, width: i32 }
}

impl Border {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::WHITE);
        match self {
            Border::Upper {width, ..} => {
                let start = Point::new(0, 0);
                let end = Point::new(*width, 0);
                canvas.draw_line(start, end).unwrap();
            },
            Border::Lower {bottom, width, ..} => {
                let start = Point::new(0, *bottom);
                let end = Point::new(*width, *bottom);
                canvas.draw_line(start, end).unwrap();
            }
        }
    }
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
    let mut first_player = Player {rect: Rectangle {x: 10.0, y: 10.0, width: 25, height: 150}, norm: Vect::new(1.0, 0.0) };
    let mut second_player = Player {rect: Rectangle {x: 750.0, y: 10.0, width: 25, height: 150}, norm: Vect::new(-1.0, 0.0) };
    let mut ball = Ball {rect: Rectangle{x: 200.0, y: 200.0, width: 25, height: 25}, velocity: Vect::new(200.0, 0.0) };
    let upper = Border::Upper {norm: Vect::new(0.0, -1.0), width: 800};
    let lower = Border::Lower {norm: Vect::new(0.0, 1.0), width: 800, bottom: 599};
    let mut delta: f32 = 0.0;
    while !quit {
        let now = Instant::now();
        context.canvas.set_draw_color(Color::BLACK);
        context.canvas.clear();
        ball.calc_velocity(&first_player, &second_player);
        for event in context.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => second_player.move_up(delta),
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => second_player.move_down(delta),
                Event::KeyDown { keycode: Some(Keycode::W), ..} => first_player.move_up(delta),
                Event::KeyDown { keycode: Some(Keycode::S), ..} => first_player.move_down(delta),
                _ => {}
            }
        }
        ball.move_ball(delta);
        first_player.draw(&mut context.canvas);
        second_player.draw(&mut context.canvas);
        ball.draw(&mut context.canvas);
        upper.draw(&mut context.canvas);
        lower.draw(&mut context.canvas);
        context.canvas.present();
        std::thread::sleep_ms(50);
        delta = (now.elapsed().as_micros() as f32) / 1000000.0;
    }
}

fn main() -> () {
    let init_result = create_context();
    match init_result {
        Ok(mut context) => draw(&mut context),
        Err(err) => println!("Error occured during context init: {}", err),
    }
}