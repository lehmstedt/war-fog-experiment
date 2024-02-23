extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::input::{ButtonArgs, ButtonState, Button, Key, ButtonEvent};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    posX: f64,
    posY: f64,
    speedX: f64,
    speedY: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(self.posX, self.posY)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.posX += self.speedX;
        self.posY += self.speedY;
    }

    fn change_direction(&mut self, args: &ButtonArgs){
        self.speedX = 0.0;
        self.speedY = 0.0;

        if args.state == ButtonState::Press {
            match args.button {
                Button::Keyboard(Key::Up) => self.speedY -= 1.0,
                Button::Keyboard(Key::Down) => self.speedY += 1.0,
                Button::Keyboard(Key::Left) => self.speedX -= 1.0,
                Button::Keyboard(Key::Right) => self.speedX += 1.0,
                _ => ()
            }
        }
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
        posX: 0.0,
        posY: 0.0,
        speedX: 0.0,
        speedY: 0.0
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
            app.change_direction(&args)
        }
    }
}