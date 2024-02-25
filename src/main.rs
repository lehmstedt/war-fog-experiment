extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{window::*, MouseCursorEvent};
use piston::input::{ButtonArgs, ButtonState, Button, Key, ButtonEvent};

pub struct Position {
    x: f64,
    y: f64
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    position: Position,
    speed: Position,
    map: Texture,
    character: Texture
}

const SPEED: f64 = 50.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {

            clear([1.0, 1.0, 1.0, 1.0], gl);

            let transform = c
                .transform
                .trans(self.position.x, self.position.y)
                .scale(0.5, 0.5);

            image(&self.map, c.transform, gl);

            image(&self.character, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.position.x += self.speed.x * args.dt;
        self.position.y += self.speed.y * args.dt;
    }

    fn change_direction(&mut self, args: &ButtonArgs){
        self.speed.x = 0.0;
        self.speed.y = 0.0;

        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(Key::Up) => self.speed.y -= SPEED,
                Button::Keyboard(Key::Down) => self.speed.y += SPEED,
                Button::Keyboard(Key::Left) => self.speed.x -= SPEED,
                Button::Keyboard(Key::Right) => self.speed.x += SPEED,
                _ => ()
            }
        }
    }

    fn change_move_goal(&mut self, args: &[f64]){

        let length = ((args[0] - self.position.x).powi(2) + (args[1] - self.position.y).powi(2)).sqrt();
        self.speed.x = (args[0] - self.position.x) / length * SPEED;
        self.speed.y = (args[1] - self.position.y) / length * SPEED;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("War Fog Experiment", [1280, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        position: Position { x: 0.0, y: 0.0 },
        speed: Position { x: 0.0, y: 0.0 },
        map: load_map(),
        character: load_character()
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args(){
            app.change_direction(&args);
        }
        if let Some(args) = e.mouse_cursor_args(){
            app.change_move_goal(&args);
        }
    }
}

fn load_map() -> Texture{
    let path = Path::new("./assets/map.jpg");
    Texture::from_path(path, &TextureSettings::new()).unwrap()   
}

fn load_character() -> Texture {
    let path = Path::new("./assets/rust.png");
    Texture::from_path(path, &TextureSettings::new()).unwrap()
}