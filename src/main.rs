extern crate glutin_window;
extern crate graphics;
extern crate num;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, ButtonArgs, ButtonEvent, ButtonState, Key};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::{window::*, MouseButton, MouseCursorEvent};
use vec2d::Vec2D;
use graphics::{Context, math};
use crate::graphics::{ImageSize, Transformed};

mod character;
mod collision;
mod scout;
mod vec2d;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    player: character::Character,
    scout: scout::Scout,
    cursor_position: vec2d::Vec2D,
    camera_transform: vec2d::Vec2D,
    camera_position: vec2d::Vec2D,
    camera_speed: vec2d::Vec2D,
    player_renderable: Renderable,
    player_target_renderable: Renderable,
    scout_renderable: Renderable,
    map_renderable: Renderable
}

pub struct Renderable {
    position: vec2d::Vec2D,
    texture: Texture,
    size: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0, 1.0, 1.0, 1.0], gl);

            self.camera_transform.x = args.window_size[0] / 2.0;
            self.camera_transform.y = args.window_size[1] / 2.0;

            let map_transform = calculate_transform(&self.map_renderable, &c, &self.camera_position, &self.camera_transform);
            image(&self.map_renderable.texture, map_transform, gl);

            if !self.scout.is_idle() && self.scout.is_visible() {
                self.scout_renderable.position = *self.scout.get_position();
                let scout_transform = calculate_transform(&self.scout_renderable, &c, &self.camera_position, &self.camera_transform);
                image(&self.scout_renderable.texture, scout_transform, gl);
            }

            if self.player.is_target_set() {
                self.player_target_renderable.position = *self.player.get_target_position();
                let target_transform = calculate_transform(&self.player_target_renderable, &c, &self.camera_position, &self.camera_transform);
                image(&self.player_target_renderable.texture, target_transform, gl);
            }

            self.player_renderable.position = *self.player.get_position();
            let player_transform = calculate_transform(&self.player_renderable, &c, &self.camera_position, &self.camera_transform);
            image(&self.player_renderable.texture, player_transform, gl);
        });
    }

    

    fn update(&mut self, args: &UpdateArgs) {
        self.player.update_position(&args.dt);
        self.scout.update_position(&args.dt);

        if !self.scout.is_target_set()
            && !self.scout.is_idle()
            && collision::are_positions_colliding(
                self.player.get_position(),
                self.scout.get_position(),
                collision::CollisionType::ScoutRetrieving,
            )
        {
            self.scout.set_idle();
        }

        let is_scout_visible = collision::are_positions_colliding(self.player.get_position(), self.scout.get_position(), collision::CollisionType::View);
        self.scout.set_visible(is_scout_visible);

        self.camera_position.x += self.camera_speed.x * args.dt;
        self.camera_position.y += self.camera_speed.y * args.dt;
    }

    fn change_direction(&mut self, args: &ButtonArgs) {
        match args.state {
            ButtonState::Press => {
                match args.button {
                    Button::Mouse(MouseButton::Left) => {
                        let cursor_world_position = self.get_cursor_world_position();
                        self.player.set_target(&cursor_world_position);
                    }
                    Button::Mouse(MouseButton::Right) => {
                        if self.scout.is_idle() {
                            let cursor_world_position = self.get_cursor_world_position();
                            self.scout.set_position(self.player.get_position());
                            self.scout
                                .set_target(&cursor_world_position, self.player.get_position());
                        }
                    }
                    Button::Keyboard(Key::W) => {
                        self.camera_speed.y =
                            num::clamp(self.camera_speed.y - 50.0, -50.0, 50.0)
                    }
                    Button::Keyboard(Key::A) => {
                        self.camera_speed.x =
                            num::clamp(self.camera_speed.x - 50.0, -50.0, 50.0)
                    }
                    Button::Keyboard(Key::S) => {
                        self.camera_speed.y =
                            num::clamp(self.camera_speed.y + 50.0, -50.0, 50.0)
                    }
                    Button::Keyboard(Key::D) => {
                        self.camera_speed.x =
                            num::clamp(self.camera_speed.x + 50.0, -50.0, 50.0)
                    }
                    _ => (),
                }
            }
            ButtonState::Release => match args.button {
                Button::Keyboard(Key::W) => self.camera_speed.y = 0.0,
                Button::Keyboard(Key::A) => self.camera_speed.x = 0.0,
                Button::Keyboard(Key::S) => self.camera_speed.y = 0.0,
                Button::Keyboard(Key::D) => self.camera_speed.x = 0.0,
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
        player: character::new(),
        scout: scout::new(),
        cursor_position: vec2d::new(),
        camera_transform: vec2d::new(),
        camera_position: vec2d::new(),
        camera_speed: vec2d::new(),
        player_renderable: Renderable {
            position: vec2d::new(),
            texture: load_texture_from_path("./assets/rust.png"),
            size: 1.0
        },
        player_target_renderable: Renderable {
            position: vec2d::new(),
            texture: load_texture_from_path("./assets/target.png"),
            size: 0.25
        },
        scout_renderable: Renderable {
            position: vec2d::new(),
            texture: load_texture_from_path("./assets/scout.png"),
            size: 0.25
        },
        map_renderable: Renderable {
            position: vec2d::new(),
            texture: load_texture_from_path("./assets/map_2.jpg"),
            size: 1.0
        },
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            app.change_direction(&args);
        }
        if let Some(args) = e.mouse_cursor_args() {
            app.update_cursor_position(&args);
        }
    }
}

fn load_texture_from_path(path: &str) -> Texture {
    Texture::from_path(Path::new(path), &TextureSettings::new()).unwrap()
}
