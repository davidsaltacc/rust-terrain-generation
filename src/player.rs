// I've shed tears of pain while writing this just so you know
// my lifespan probably shortened by a few years because of rust
use std::collections::HashMap;
use crate::{utils::{self, magnitude, normalize}, vector::Vector3};

#[derive(Default)]
pub struct Player {
    pub player_position: Vector3,
    pub camera_rotation: Vector3,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Player {
    pub fn new() -> Self {
        Self { player_position: Vector3::new(0.0, 0.0, 0.0), camera_rotation: Vector3::new(0.0, 0.0, 0.0), speed: 2.0, sensitivity: 0.1 }
    }

    // uncomment if ever needed
    // pub fn set_position(&mut self, position: [f32; 3]) {
    //     self.player_position = position;
    // }

    // pub fn set_camera_rotation(&mut self, rotation: [f32; 3]) {
    //     self.camera_rotation = rotation;
    // }

    fn move_player(&mut self, mut movement: Vector3, amount: f32, direction: Vector3) -> Vector3 {
        movement.x = amount * direction.x;
        movement.y = amount * direction.y;
        movement.z = amount * direction.z;
        return movement;
    }

    pub fn move_camera(&mut self, x: f32, y: f32) {
        self.camera_rotation.x += x * self.sensitivity;
        self.camera_rotation.y -= y * self.sensitivity;

        self.camera_rotation.x = self.camera_rotation.x % 360.0;
        self.camera_rotation.y = self.camera_rotation.y.clamp(-85.0, 85.0);
    }

    // you will need to call this every frame
    pub fn update(&mut self, inputs: &HashMap<u8, bool>, dt: f32) {
        let rotation: Vector3 = self.camera_rotation;
        let move_amount = self.speed * dt;
        let mut movement : Vector3 = Vector3::new(0.0, 0.0, 0.0);

        // these must be in this order if you move a and d after the rest shit will unexpectedly break.
        if *inputs.get(&('a' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(Vector3::new(rotation.x + 270., rotation.y, rotation.z));
            movement += self.move_player(movement, move_amount, direction);
            movement.y = 0.0;
        }
        if *inputs.get(&('d' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(Vector3::new(rotation.x + 90., rotation.y, rotation.z));
            movement += self.move_player(movement, move_amount, direction);
            movement.y = 0.0;
        }
        if *inputs.get(&('w' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(rotation);
            movement += self.move_player(movement, move_amount, direction);
        }
        if *inputs.get(&('s' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(Vector3::new(rotation.x + 180., -rotation.y, rotation.z));
            movement += self.move_player(movement, move_amount, direction);
        }
        if *inputs.get(&('e' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(Vector3::new(rotation.x, rotation.y + 90., rotation.z));
            movement += self.move_player(movement, move_amount, direction);
        }
        if *inputs.get(&('q' as u8)).unwrap_or(&false) {
            let direction: Vector3 = utils::rotation_to_direction(Vector3::new(rotation.x, rotation.y - 90., rotation.z));
            movement += self.move_player(movement, move_amount, direction);
        }
        
        if magnitude(movement) > move_amount {
            movement = normalize(movement);
            movement *= move_amount;
        }
        movement.x = -movement.x;
        self.player_position += movement;
    }

    pub fn get_relative_position(&self, position: Vector3) -> Vector3 {
        let new_position: Vector3 = position - self.player_position;

        return new_position;
    }
}