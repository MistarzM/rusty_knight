use crate::constants;

use super::game_object;
use std::collections::HashMap;

pub struct World {
    pub quads: Vec<game_object::Object>,
    pub tris: Vec<game_object::Object>,
    pub camera: game_object::Camera,
    pub keys: HashMap<glfw::Key, bool>,
}

impl World {
    pub fn new() -> Self {
        let mut world = World {
            quads: Vec::new(),
            tris: Vec::new(),
            camera: game_object::Camera::new(),
            keys: HashMap::new(),
        };

        world.keys.insert(glfw::Key::W, false);
        world.keys.insert(glfw::Key::A, false);
        world.keys.insert(glfw::Key::S, false);
        world.keys.insert(glfw::Key::D, false);

        world
    }

    pub fn set_key(&mut self, key: glfw::Key, state: bool) {
        self.keys.insert(key, state);
    }

    pub fn update(&mut self, dt: f32, window: &mut glfw::Window) {
        for i in 0..self.tris.len() {
            self.tris[i].angle += 0.001 * dt;
            if self.tris[i].angle > 360.0 {
                self.tris[i].angle -= 360.0;
            }
        }

        let mouse_pos = window.get_cursor_pos();
        window.set_cursor_pos(640.0, 360.0);
        let dx = (-40.0 * (mouse_pos.0 - 640.0) / 640.0) as f32;
        let dy = (-40.0 * (mouse_pos.1 - 360.0) / 360.0) as f32;

        self.camera.camera_spin(dx, dy);

        let mut d_right: f32 = 0.0;
        let mut d_forwards: f32 = 0.0;

        if self.keys[&glfw::Key::W] {
            d_forwards += constants::gameplay::MOVEMENT_SPEED;
        }
        if self.keys[&glfw::Key::A] {
            d_right -= constants::gameplay::MOVEMENT_SPEED;
        }
        if self.keys[&glfw::Key::S] {
            d_forwards -= constants::gameplay::MOVEMENT_SPEED;
        }
        if self.keys[&glfw::Key::D] {
            d_right += constants::gameplay::MOVEMENT_SPEED;
        }

        self.camera.camera_move(d_right, d_forwards);
    }
}
