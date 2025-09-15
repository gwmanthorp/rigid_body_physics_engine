mod physics_engine;
mod rigid_body;

use crate::physics_engine::PhysicsEngine;
use crate::rigid_body::RigidBody;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics::{self, Color, DrawMode, MeshBuilder};
use ggez::mint::Point2;
use ggez::{event, Context, ContextBuilder, GameResult};
use nalgebra::Vector2;

struct MainState {
    physics_engine: PhysicsEngine,
    is_mouse_held: bool,
    start_point: Vector2<f32>,
    end_point: Vector2<f32>,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let mut physics_engine = PhysicsEngine::new(Vector2::new(0.0, 980.0)); // Gravity downwards

        // Add some rigid bodies
        //physics_engine.add_body(RigidBody::new(1.0, Vector2::new(400.0, 100.0), false, None));
        //physics_engine.add_body(RigidBody::new(1.0, Vector2::new(400.0, 300.0), false, None));
        //physics_engine.add_body(RigidBody::new(0.0, Vector2::new(400.0, 300.0), true, None)); // Static ground body

        Ok(MainState {
            physics_engine,
            is_mouse_held: false,
            start_point: Vector2::new(0.0, 0.0),
            end_point: Vector2::new(0.0, 0.0),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let dt = 1.0 / 60.0; // Fixed time step
        self.physics_engine.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);

        let mut mesh_builder = MeshBuilder::new();
        for body in self.physics_engine.bodies() {
            let _ = mesh_builder.circle(
                DrawMode::fill(),
                Point2 {
                    x: body.position.x,
                    y: body.position.y,
                }, // Explicitly construct Point2
                10.0,
                0.1,
                Color::WHITE,
            );
        }

        if self.is_mouse_held {
            let _ = mesh_builder.line(
                &[
                    Point2 {
                        x: self.start_point.x,
                        y: self.start_point.y,
                    },
                    Point2 {
                        x: self.start_point.x + (self.start_point.x - self.end_point.x),
                        y: self.start_point.y + (self.start_point.y - self.end_point.y),
                    },
                ],
                5.0,
                Color::WHITE,
            );
        }

        if !self.physics_engine.bodies().is_empty() || self.is_mouse_held {
            let mesh = mesh_builder.build(ctx)?;
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        if self.is_mouse_held {
            self.end_point.x = _x;
            self.end_point.y = _y;
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match _button {
            MouseButton::Left => {
                self.is_mouse_held = true; // Track the mouse hold
                self.start_point.x = _x;
                self.start_point.y = _y;
                self.end_point.x = _x;
                self.end_point.y = _y;

                self.physics_engine.create_starting_pos(RigidBody::new(
                    0.0,
                    Vector2::new(_x, _y),
                    true,
                    None,
                    10.0, // radius (added fifth argument)
                ));
                //println!("Mouse button pressed");
            }
            _ => {
                println!("Other button is clicked");
            }
        }
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        match _button {
            MouseButton::Left => {
                if self.is_mouse_held {
                    self.is_mouse_held = false; // Reset the hold state on release
                    self.end_point.x = _x;
                    self.end_point.y = _y;

                    self.physics_engine.delete_starting_pos(); // Deletes the starting point for the object created

                    let starting_velocity = Vector2::new(
                        5.0 * (self.start_point.x - self.end_point.x),
                        5.0 * (self.start_point.y - self.end_point.y),
                    );

                    self.physics_engine.add_body(RigidBody::new(
                        1.0,
                        Vector2::new(self.start_point.x, self.start_point.y),
                        false,
                        Some(starting_velocity),
                        10.0, // radius (added fifth argument)
                    ));
                    //println!("Mouse button released");
                }
            }
            _ => {
                println!("Other button released");
            }
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("my_physics_engine", "George")
        .build()
        .expect("Failed to create ggez context");

    let state = MainState::new()?;
    event::run(ctx, event_loop, state) // Pass without mutable references
}
