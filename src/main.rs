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
use piston::input::{ButtonArgs, ButtonState, Button, ButtonEvent};

pub struct Character {
    position: Vec2D,
    speed: Vec2D,
    is_target_set: bool,
    target_position: Vec2D
}

impl Character {

    fn update_position(&mut self, dt: &f64){
        self.position.x += self.speed.x * dt;
        self.position.y += self.speed.y * dt;

        if self.is_target_set && (self.position.x - self.target_position.x).abs() < 0.25 && (self.position.y - self.target_position.y).abs() < 0.25 {
            self.unset_target();
        }
    }

    fn unset_target(&mut self){
        self.is_target_set = false;
        self.speed.x = 0.0;
        self.speed.y = 0.0;
    }

    fn set_target(&mut self, target_position: &Vec2D){
        self.is_target_set = true;
        self.target_position.x = target_position.x;
        self.target_position.y = target_position.y;

        let length = ((self.target_position.x - self.position.x).powi(2) + (self.target_position.y - self.position.y).powi(2)).sqrt();
        self.speed.x = (self.target_position.x - self.position.x) / length * SPEED;
        self.speed.y = (self.target_position.y - self.position.y) / length * SPEED;
    }
}

pub struct Vec2D {
    x: f64,
    y: f64
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: Character,
    cursor_position: Vec2D,
    textures: GameTextures,
    camera_transform: Vec2D
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
                .trans(self.player.position.x, self.player.position.y)
                .trans(self.textures.player.get_width() as f64 * - 0.5, self.textures.player.get_height() as f64 * - 0.5)
                .trans(self.camera_transform.x, self.camera_transform.y);

            let size_factor = 0.25;

            let map_transform = c
                .transform
                .trans(0.0, 0.0)
                .trans(self.textures.map.get_width() as f64 * - 0.5, self.textures.map.get_height() as f64 * - 0.5)
                .trans(self.camera_transform.x, self.camera_transform.y);

            image(&self.textures.map, map_transform, gl);

            if self.player.is_target_set {
                let target_transform = c
                .transform
                .trans(self.player.target_position.x, self.player.target_position.y)
                .trans(self.textures.target.get_width() as f64 * - 0.5 * size_factor, self.textures.target.get_height() as f64 * - 0.5 * size_factor)
                .trans(self.camera_transform.x, self.camera_transform.y)
                .scale(size_factor, size_factor);

                image(&self.textures.target, target_transform, gl);
            }

            image(&self.textures.player, player_transform, gl);
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

        self.player.update_position(&args.dt);
        
    }

    fn change_direction(&mut self, args: &ButtonArgs){

        if args.state == ButtonState::Press {
            match args.button {
                Button::Mouse(MouseButton::Left) => self.set_player_target(),
                _ => ()
            }
        }
    }

    fn set_player_target(&mut self){

        let cursor_world_position = Vec2D {
            x: self.cursor_position.x - self.camera_transform.x,
            y: self.cursor_position.y - self.camera_transform.y
        };

        self.player.set_target(&cursor_world_position);
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
        player: Character {
            position: Vec2D { x: 0.0, y: 0.0 },
            speed: Vec2D { x: 0.0, y: 0.0 },
            is_target_set: false,
            target_position: Vec2D { x: 0.0, y: 0.0 }
        },
        cursor_position: Vec2D { x: 0.0, y: 0.0 },
        textures: load_game_textures(),
        camera_transform: Vec2D { x: 0.0, y: 0.0 }
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