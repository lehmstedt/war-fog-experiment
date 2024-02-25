extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{window::*, MouseButton, MouseCursorEvent};
use piston::input::{ButtonArgs, ButtonState, Button, Key, ButtonEvent};

pub struct Position {
    x: f64,
    y: f64
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player_position: Position,
    speed: Position,
    cursor_position: Position,
    target_position: Position,
    textures: GameTextures,
    is_target_set: bool,
    camera_transform: Position
}

pub struct GameTextures {
    player: Texture,
    target: Texture,
    map: Texture
}

const SPEED: f64 = 50.0;

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {

            clear([1.0, 1.0, 1.0, 1.0], gl);

            self.camera_transform.x = args.window_size[0] / 2.0;
            self.camera_transform.y = args.window_size[1] / 2.0;

            let player_transform = c
                .transform
                .trans(self.player_position.x, self.player_position.y)
                .trans(self.textures.player.get_width() as f64 * - 0.5, self.textures.player.get_height() as f64 * - 0.5)
                .trans(self.camera_transform.x, self.camera_transform.y);

            let size_factor = 0.25;

            let map_transform = c
                .transform
                .trans(0.0, 0.0)
                .trans(self.textures.map.get_width() as f64 * - 0.5, self.textures.map.get_height() as f64 * - 0.5)
                .trans(self.camera_transform.x, self.camera_transform.y);

            image(&self.textures.map, map_transform, gl);

            if self.is_target_set {
                let target_transform = c
                .transform
                .trans(self.target_position.x, self.target_position.y)
                .trans(self.textures.target.get_width() as f64 * - 0.5 * size_factor, self.textures.target.get_height() as f64 * - 0.5 * size_factor)
                .trans(self.camera_transform.x, self.camera_transform.y)
                .scale(size_factor, size_factor);

                image(&self.textures.target, target_transform, gl);
            }

            image(&self.textures.player, player_transform, gl);
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.player_position.x += self.speed.x * args.dt;
        self.player_position.y += self.speed.y * args.dt;

        if self.is_target_set && (self.player_position.x - self.target_position.x).abs() < 0.25 && (self.player_position.y - self.target_position.y).abs() < 0.25 {
            self.unset_target();
        }
    }

    fn change_direction(&mut self, args: &ButtonArgs){

        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(Key::Up) => self.speed.y -= SPEED,
                Button::Keyboard(Key::Down) => self.speed.y += SPEED,
                Button::Keyboard(Key::Left) => self.speed.x -= SPEED,
                Button::Keyboard(Key::Right) => self.speed.x += SPEED,
                Button::Mouse(MouseButton::Left) => self.set_target(),
                _ => ()
            }
        }
    }

    fn set_target(&mut self){
        self.is_target_set = true;
        self.target_position.x = self.cursor_position.x - self.camera_transform.x;
        self.target_position.y = self.cursor_position.y - self.camera_transform.y;

        let length = ((self.target_position.x - self.player_position.x).powi(2) + (self.target_position.y - self.player_position.y).powi(2)).sqrt();
        self.speed.x = (self.target_position.x - self.player_position.x) / length * SPEED;
        self.speed.y = (self.target_position.y - self.player_position.y) / length * SPEED;
    }

    fn unset_target(&mut self){
        self.is_target_set = false;
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    fn update_cursor_position(&mut self, args: &[f64]){
        self.cursor_position.x = args[0];
        self.cursor_position.y = args[1];
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
        player_position: Position { x: 0.0, y: 0.0 },
        cursor_position: Position { x: 0.0, y: 0.0 },
        target_position: Position { x: 0.0, y: 0.0 },
        speed: Position { x: 0.0, y: 0.0 },
        textures: load_game_textures(),
        is_target_set: false,
        camera_transform: Position { x: 0.0, y: 0.0 }
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
            app.update_cursor_position(&args);
        }
    }
}

fn load_game_textures() -> GameTextures{
    GameTextures {
        player: load_texture_from_path("./assets/rust.png"),
        target: load_texture_from_path("./assets/target.png"),
        map: load_texture_from_path("./assets/map_2.jpg")
    }
}

fn load_texture_from_path(path: &str) -> Texture{
    Texture::from_path(Path::new(path), &TextureSettings::new()).unwrap()
}