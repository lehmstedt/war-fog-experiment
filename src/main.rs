extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use std::path::Path;
use piston::input::{Button, ButtonArgs, ButtonEvent, ButtonState, Key};
use piston::input::{ RenderEvent, UpdateArgs, UpdateEvent};
use piston::{ MouseButton, MouseCursorEvent};
use vec2d::Vec2D;
use graphics::{clear, math, rectangle, Context};
use crate::graphics::{ImageSize, Transformed};
use crate::scout::ScoutStatus;
use piston_window::prelude::*;
use piston_window::*;

mod character;
mod collision;
mod scout;
mod vec2d;
mod game;

use crate::character::CharacterStatus;

const CAMERA_MOVE_SPEED: f64 = 200.0;

pub struct App {
    cursor_position: vec2d::Vec2D,
    camera_transform: vec2d::Vec2D,
    camera_position: vec2d::Vec2D,
    camera_speed: vec2d::Vec2D,
    player_renderable: Renderable,
    player_target_renderable: Renderable,
    scout_renderable: Renderable,
    map_renderable: Renderable,
    enemy_renderable: Renderable,
    god_mode: bool,
    font: Glyphs,
    window: PistonWindow
}

pub struct Renderable {
    position: vec2d::Vec2D,
    texture: G2dTexture,
    size: f64
}

impl App {
    fn render(&mut self, event: &Event, game: &game::Game) {

        self.window.draw_2d(event, |c, gl, device| {

            self.font.factory.encoder.flush(device);

            self.camera_transform.x = event.render_args().unwrap().window_size[0] / 2.0;
            self.camera_transform.y = event.render_args().unwrap().window_size[1] / 2.0;

            if game.is_over() {
                clear([0.0, 0.0, 0.0, 1.0], gl);
                let game_over_text = if *game.get_player().get_health() > 0.0 { "You won" } else { "You lose"};
                text([1.0, 1.0, 1.0, 1.0], 64, &game_over_text, &mut self.font, c.transform.trans(self.camera_transform.x, self.camera_transform.y), gl).unwrap();
                return;
            }
            clear([1.0, 1.0, 1.0, 1.0], gl);

            

            let map_transform = calculate_transform(&self.map_renderable, &c, &self.camera_position, &self.camera_transform);
            image(&self.map_renderable.texture, map_transform, gl);

            if (game.get_scout().is_visible() || self.god_mode) && *game.get_scout().get_status() != ScoutStatus::Idle {
                self.scout_renderable.position = *game.get_scout().get_position();
                let scout_transform = calculate_transform(&self.scout_renderable, &c, &self.camera_position, &self.camera_transform);
                image(&self.scout_renderable.texture, scout_transform, gl);
            }

            if *game.get_player().get_status() == CharacterStatus::Moving || self.god_mode {
                self.player_target_renderable.position = *game.get_player_target_position();
                let target_transform = calculate_transform(&self.player_target_renderable, &c, &self.camera_position, &self.camera_transform);
                image(&self.player_target_renderable.texture, target_transform, gl);

            }

            if game.is_enemy_visible() || self.god_mode {
                self.enemy_renderable.position = *game.get_enemy_position();
                let enemy_transform = calculate_transform(&self.enemy_renderable, &c, &self.camera_position, &self.camera_transform);
                image(&self.enemy_renderable.texture, enemy_transform, gl);
            }
            else if game.is_enemy_discovered() {
                self.enemy_renderable.position = *game.get_discovered_enemy_position();
                let enemy_transform = calculate_transform(&self.enemy_renderable, &c, &self.camera_position, &self.camera_transform);
                let transparent_image = Image::new().color([1.0, 1.0, 1.0, 0.5]);
                transparent_image.draw(&self.enemy_renderable.texture, &DrawState::default(), enemy_transform, gl);
            }

            self.player_renderable.position = *game.get_player_position();
            let player_transform = calculate_transform(&self.player_renderable, &c, &self.camera_position, &self.camera_transform);
            image(&self.player_renderable.texture, player_transform, gl);

            let player_health = game.get_player().get_health();
            text([0.0, 0.0, 0.0, 1.0], 32, &format!("Health"), &mut self.font, c.transform.trans(self.camera_transform.x * 0.1, self.camera_transform.y * 1.9), gl).unwrap();
            let health_rectangle = rectangle::rectangle_by_corners(0.0, 0.0, *player_health, 20.0);
            let health_loss_rectange = rectangle::rectangle_by_corners(0.0, 0.0, 100.0 - *player_health, 20.0);
            rectangle([0.0, 0.8, 0.0, 1.0], health_rectangle, c.transform.trans(self.camera_transform.x * 0.3, (self.camera_transform.y * 1.9) - 20.0), gl);
            rectangle([0.8, 0.0, 0.0, 1.0], health_loss_rectange, c.transform.trans((self.camera_transform.x * 0.3) + *player_health, (self.camera_transform.y * 1.9) - 20.0), gl);

            if self.god_mode {
                let enemy_health = game.get_enemy().get_health();
                text([0.0, 0.0, 0.0, 1.0], 32, &format!("Enemy health : {enemy_health}"), &mut self.font, c.transform.trans(self.camera_transform.x * 1.5, self.camera_transform.y * 1.9), gl).unwrap(); 
            }

        });
    }

    

    fn update(&mut self, args: &UpdateArgs, game: &mut game::Game) {

        game.update(&args.dt);

        self.camera_position.x += self.camera_speed.x * args.dt;
        self.camera_position.y += self.camera_speed.y * args.dt;
    }

    fn react_to_inputs(&mut self, args: &ButtonArgs, game: &mut game::Game) {
        match args.state {
            ButtonState::Press => {
                match args.button {
                    Button::Mouse(MouseButton::Left) => {
                        let cursor_world_position = self.get_cursor_world_position();
                        game.set_player_target(&cursor_world_position);

                    }
                    Button::Mouse(MouseButton::Right) => {
                        let cursor_world_position = self.get_cursor_world_position();
                        game.set_scout_mission(&cursor_world_position);
                    }
                    Button::Keyboard(Key::W) => {
                        self.camera_speed.y =
                            num::clamp(self.camera_speed.y - CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED)
                    }
                    Button::Keyboard(Key::A) => {
                        self.camera_speed.x =
                            num::clamp(self.camera_speed.x - CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED)
                    }
                    Button::Keyboard(Key::S) => {
                        self.camera_speed.y =
                            num::clamp(self.camera_speed.y + CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED)
                    }
                    Button::Keyboard(Key::D) => {
                        self.camera_speed.x =
                            num::clamp(self.camera_speed.x + CAMERA_MOVE_SPEED, -CAMERA_MOVE_SPEED, CAMERA_MOVE_SPEED)
                    },
                    Button::Keyboard(Key::G) => {
                        self.god_mode = true
                    }
                    _ => (),
                }
            }
            ButtonState::Release => match args.button {
                Button::Keyboard(Key::W) => self.camera_speed.y = 0.0,
                Button::Keyboard(Key::A) => self.camera_speed.x = 0.0,
                Button::Keyboard(Key::S) => self.camera_speed.y = 0.0,
                Button::Keyboard(Key::D) => self.camera_speed.x = 0.0,
                Button::Keyboard(Key::G) => self.god_mode = false,
                _ => (),
            },
        }
    }

    fn get_cursor_world_position(&mut self) -> Vec2D {
        Vec2D {
            x: self.cursor_position.x - self.camera_transform.x + self.camera_position.x,
            y: self.cursor_position.y - self.camera_transform.y + self.camera_position.y,
        }
    }

    fn update_cursor_position(&mut self, args: &[f64]) {
        self.cursor_position.x = args[0];
        self.cursor_position.y = args[1];
    }
}

fn calculate_transform(renderable: &Renderable, c: &Context, camera_position: &vec2d::Vec2D, camera_transform: &vec2d::Vec2D) -> math::Matrix2d {
    c
        .transform
        .trans(renderable.position.x, renderable.position.y)
        .trans(
            renderable.texture.get_width() as f64 * -0.5 * renderable.size,
            renderable.texture.get_height() as f64 * -0.5 * renderable.size,
        )
        .trans(- camera_position.x, - camera_position.y)
        .trans(camera_transform.x, camera_transform.y)
        .scale(renderable.size, renderable.size)
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("War Fog Experiment", [1280, 1080])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };

    let font = window.load_font(Path::new("assets/TheanoDidot-Regular.ttf")).unwrap();

    // Create a new game and run it.
    let mut app = App {
        window,
        cursor_position: vec2d::new(),
        camera_transform: vec2d::new(),
        camera_position: vec2d::new(),
        camera_speed: vec2d::new(),
        player_renderable: Renderable {
            position: vec2d::new(),
            texture: Texture::from_path(&mut texture_context, Path::new("./assets/rust.png"), Flip::None, &TextureSettings::new()).unwrap(),
            size: 1.0
        },
        player_target_renderable: Renderable {
            position: vec2d::new(),
            texture: Texture::from_path(&mut texture_context, Path::new("./assets/target.png"), Flip::None, &TextureSettings::new()).unwrap(),
            size: 0.25
        },
        scout_renderable: Renderable {
            position: vec2d::new(),
            texture: Texture::from_path(&mut texture_context, Path::new("./assets/scout.png"), Flip::None, &TextureSettings::new()).unwrap(),
            size: 0.25
        },
        map_renderable: Renderable {
            position: vec2d::new(),
            texture: Texture::from_path(&mut texture_context, Path::new("./assets/map_2.jpg"), Flip::None, &TextureSettings::new()).unwrap(),
            size: 1.0
        },
        enemy_renderable: Renderable {
            position: vec2d::new(),
            texture: Texture::from_path(&mut texture_context, Path::new("./assets/enemy.png"), Flip::None, &TextureSettings::new()).unwrap(),
            size: 0.25
        },
        god_mode: false,
        font
    };

    while let Some(e) = app.window.next() {

        app.render(&e, &game);
        
        if let Some(args) = e.update_args() {
            app.update(&args, &mut game);
        }

        if let Some(args) = e.button_args() {
            app.react_to_inputs(&args, &mut game);
        }
        if let Some(args) = e.mouse_cursor_args() {
            app.update_cursor_position(&args);
        }
    }
    
}
