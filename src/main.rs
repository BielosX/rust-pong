extern crate sdl2;
extern crate nalgebra;

use std::time::Instant;
use std::cmp::Ordering;

use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::pixels::Color;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::ttf;
use sdl2::ttf::Font;
use sdl2::surface::Surface;

use nalgebra::Vector2;

pub type Vect = Vector2<f32>;

trait Obstacle {
    fn collision(&self, ball: &Ball) -> bool;
    fn normal(&self, ball: &Ball) -> Vect;
    fn bounce_ball(&self, ball: &mut Ball) {
        ball.velocity = reflection(&ball.velocity, &self.normal(ball))
    }
}

fn reflection(vec: &Vect, normal: &Vect) -> Vect {
    let coef = vec.dot(normal) * 2.0;
    -1.0 * (coef * normal - vec)
}

struct Context {
    canvas: WindowCanvas,
    event_pump: EventPump,
    ttf_context: ttf::Sdl2TtfContext
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

    pub fn right_x(&self) -> f32 {
        self.x + self.width as f32
    }
    
    pub fn bottom_y(&self) -> f32 {
        self.y + self.height as f32
    }
}

struct Player {
    rect: Rectangle,
    norm: Vect,
    velocity: f32
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
    fn new(rect: Rectangle, norm: Vect) -> Player {
        Player {rect: rect, norm: norm, velocity: 0.0}
    }

    fn top_collision(&self) -> bool {
        approx_equal(self.rect.y, 0.0) || self.rect.y < 0.0
    }

    fn bottom_collision(&self) -> bool {
        let bottom = self.rect.y + self.rect.height as f32;
        approx_equal(bottom, 600.0) || bottom > 600.0
    }

    pub fn move_player(&mut self, delta_time: f32) {
        if !self.top_collision() && self.velocity < 0.0 {
            self.rect.y += delta_time * self.velocity;
        }
        if !self.bottom_collision() && self.velocity > 0.0 {
            self.rect.y += delta_time * self.velocity;
        }
    }
    
    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        self.rect.draw(canvas)
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity
    }
}

fn compare(first: (i32, i32), second: (i32, i32)) -> Ordering {
    let (first_position, _first_index) = first;
    let (second_position, _second_index) = second;
    first_position.cmp(&second_position)
}

impl Obstacle for Player {

    fn collision(&self, ball: &Ball) -> bool {
        let mut horizontal = [
            (self.rect.x as i32, 0), (self.rect.right_x() as i32, 0),
            (ball.rect.x as i32, 1), (ball.rect.right_x() as i32, 1)
        ];
        let mut vertical = [
            (self.rect.y as i32, 0), (self.rect.bottom_y() as i32, 0),
            (ball.rect.y as i32, 1), (ball.rect.bottom_y() as i32, 1)
        ];
        horizontal.sort_by(|a,b| compare(*a, *b));
        vertical.sort_by(|a,b| compare(*a, *b));
        let horizontal_ok;
        let vertical_ok;
        match horizontal {
            [(_, 0), (_, 0), _, _] => horizontal_ok = true,
            [(_, 1), (_, 1), _, _] => horizontal_ok = true,
            _ => horizontal_ok = false,
        }
        match vertical {
            [(_, 0), (_, 0), _, _] => vertical_ok = true,
            [(_, 1), (_, 1), _, _] => vertical_ok = true,
            _ => vertical_ok = false,
        }
        !horizontal_ok && !vertical_ok
    }

    fn normal(&self, _ball: &Ball) -> Vect {
        self.norm
    }

    fn bounce_ball(&self, ball: &mut Ball) {
        ball.velocity = reflection(&ball.velocity, &self.normal(ball)) + Vect::new(0.0, self.velocity)
    }
}

struct Ball {
    rect: Rectangle,
    velocity: Vect
}

impl Ball {
    pub fn draw(&self, canvas: &mut WindowCanvas) {
        self.rect.draw(canvas)
    }
    
    pub fn move_ball(&mut self, delta_time: f32) {
        self.rect.x += delta_time * self.velocity.x;
        self.rect.y += delta_time * self.velocity.y;
    }

    pub fn calc_velocity(&mut self, obstacles: &[&dyn Obstacle]) {
        for obstacle in obstacles {
            if obstacle.collision(self) {
                obstacle.bounce_ball(self)
            }
        }
    }

    pub fn new() -> Ball {
        Ball {rect: Rectangle{x: 200.0, y: 200.0, width: 25, height: 25}, velocity: Vect::new(20.0, 0.0) }
    }

    pub fn set_to_origin(&mut self) {
        self.rect.x = 200.0;
        self.rect.y = 200.0;
        self.velocity = Vect::new(20.0, 0.0);
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

impl Obstacle for Border {
    fn collision(&self, ball: &Ball) -> bool {
        match self {
            Border::Upper {..} => ball.rect.y < 0.0,
            Border::Lower {bottom, ..} => ball.rect.bottom_y() > *bottom as f32
        }
    }

    fn normal(&self, _ball: &Ball) -> Vect {
        match self {
            Border::Lower {norm, ..} => *norm,
            Border::Upper {norm, ..} => *norm
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
    let ttf_context = ttf::init().map_err(|x| -> String {x.to_string()})?;
    Ok(Context {canvas, event_pump, ttf_context})
}

fn tick(event_pump: &mut EventPump,
    first_player: &mut Player,
    second_player: &mut Player,
    upper: &Border,
    lower: &Border,
    ball: &mut Ball,
    score_board: &mut ScoreBoard,
    font: &Font) -> bool {
    let delta: f32 = 0.01;
    let mut quit = false;
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => quit = true,
            Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, ..} => second_player.set_velocity(-40.0),
            Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, ..} => second_player.set_velocity(40.0),
            Event::KeyDown { keycode: Some(Keycode::W), repeat: false, ..} => first_player.set_velocity(-40.0),
            Event::KeyDown { keycode: Some(Keycode::S), repeat: false, ..} => first_player.set_velocity(40.0),
            Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, ..} => second_player.set_velocity(0.0),
            Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, ..} => second_player.set_velocity(0.0),
            Event::KeyUp { keycode: Some(Keycode::W), repeat: false, ..} => first_player.set_velocity(0.0),
            Event::KeyUp { keycode: Some(Keycode::S), repeat: false, ..} => first_player.set_velocity(0.0),
            _ => {}
        }
    }
    first_player.move_player(delta);
    second_player.move_player(delta);
    let obstacles: [&dyn Obstacle; 4] = [first_player, second_player, upper, lower];
    ball.calc_velocity(&obstacles);
    if ball.rect.x < 0.0 {
        score_board.inc_second(font);
        ball.set_to_origin();
    }
    else if ball.rect.right_x() > 800.0 {
        score_board.inc_first(font);
        ball.set_to_origin();
    }
    else {
        ball.move_ball(delta);
    }
    quit
}

struct ScoreBoard<'r> {
    first: u32,
    second: u32,
    surface: Surface<'r>,
    x: i32,
    y: i32
}

impl<'r> ScoreBoard<'r> {
    fn new(font: &Font, x: i32, y: i32) -> ScoreBoard<'r> {
        let font_surface = font.render("0:0").solid(Color::WHITE).expect("Unable to render font");
        ScoreBoard {first: 0, second: 0, surface: font_surface, x: x, y: y}
    }

    fn update_surface(&mut self, font: &Font) {
        let text = format!("{}:{}", self.first, self.second);
        let font_surface = font.render(text.as_str()).solid(Color::WHITE).expect("Unable to render font");
        self.surface = font_surface
    }

    fn inc_first(&mut self, font: &Font) {
        self.first += 1;
        self.update_surface(font)
    }

    fn inc_second(&mut self, font: &Font) {
        self.second += 1;
        self.update_surface(font)
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&self.surface).unwrap();
        let width = texture.query().width;
        let height = texture.query().height;
        let dst = Rect::new(self.x, self.y, width, height);
        canvas.copy(&texture, None, dst).unwrap();
    }
}

fn draw(context: &mut Context) {
    let mut quit = false;
    let mut first_player = Player::new(Rectangle {x: 10.0, y: 10.0, width: 25, height: 150}, Vect::new(1.0, 0.0));
    let mut second_player = Player::new(Rectangle {x: 750.0, y: 10.0, width: 25, height: 150}, Vect::new(-1.0, 0.0));
    let mut ball = Ball::new();
    let upper = Border::Upper {norm: Vect::new(0.0, -1.0), width: 800};
    let lower = Border::Lower {norm: Vect::new(0.0, 1.0), width: 800, bottom: 599};
    let mut time: u128 = 0;
    let font = context.ttf_context.load_font("Lato-Black.ttf", 32).expect("Unable to load font");
    let mut score_board = ScoreBoard::new(&font, 400, 0);
    while !quit {
        if time > 10000 {
            quit = tick(&mut context.event_pump,
                &mut first_player,
                &mut second_player,
                &upper,
                &lower,
                &mut ball,
                &mut score_board,
                &font);
            time = 0;
        }
        let now = Instant::now();
        context.canvas.set_draw_color(Color::BLACK);
        context.canvas.clear();
        first_player.draw(&mut context.canvas);
        second_player.draw(&mut context.canvas);
        ball.draw(&mut context.canvas);
        upper.draw(&mut context.canvas);
        lower.draw(&mut context.canvas);
        score_board.draw(&mut context.canvas);
        context.canvas.present();
        time += now.elapsed().as_nanos();
    }
}

fn main() -> () {
    let init_result = create_context();
    match init_result {
        Ok(mut context) => draw(&mut context),
        Err(err) => println!("Error occured during context init: {}", err),
    }
}