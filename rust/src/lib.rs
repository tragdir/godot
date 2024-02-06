use godot::engine::{ISprite2D, Sprite2D};
use godot::prelude::*;

struct MyExtenstion;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtenstion {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
#[allow(dead_code)]
struct Player {
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, World!");

        Self {
            speed: 600.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}
