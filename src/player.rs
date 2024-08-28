// I've shed tears of pain while writing this just so you know
// my lifespan probably shortened by a few years because of rust
use std::collections::HashMap;

pub struct Player {
    pub player_position: [f32; 3],
    pub camera_rotation: [f32; 3],
}

impl Player {
    pub fn new() -> Self {
        Self { player_position: [0.0, 0.0, 0.0], camera_rotation: [0.0, 0.0, 0.0] }
    }

    // uncomment if ever needed
    // pub fn set_position(&mut self, position: [f32; 3]) {
    //     self.player_position = position;
    // }

    // pub fn set_camera_rotation(&mut self, rotation: [f32; 3]) {
    //     self.camera_rotation = rotation;
    // }

    // you will need to call this every frame
    pub fn update(&mut self, inputs: &HashMap<u8, bool>, dt: f32) {
        let mut rotation: [f32; 3] = self.camera_rotation;
        if inputs.get(&('w' as u8)).is_some() {
            if *inputs.get(&('w' as u8)).unwrap() {
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        if inputs.get(&('a' as u8)).is_some() {
            if *inputs.get(&('a' as u8)).unwrap() {
                rotation[0] += 270.0;
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                rotation[0] -= 270.0;
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        if inputs.get(&('s' as u8)).is_some() {
            if *inputs.get(&('s' as u8)).unwrap() {
                rotation[0] += 180.0;
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                rotation[0] -= 180.0;
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        if inputs.get(&('d' as u8)).is_some() {
            if *inputs.get(&('d' as u8)).unwrap() {
                rotation[0] += 90.0;
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                rotation[0] -= 90.0;
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        if inputs.get(&('e' as u8)).is_some() {
            if *inputs.get(&('e' as u8)).unwrap() {
                rotation[1] += 90.0;
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                rotation[1] -= 90.0;
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        if inputs.get(&('q' as u8)).is_some() {
            if *inputs.get(&('q' as u8)).unwrap() {
                rotation[1] += 270.0;
                let direction: [f32; 3] = self.rotation_to_direction(rotation);
                rotation[1] -= 270.0;
                let movement: f32 = 1.0 * dt;
                self.player_position[0] -= movement * direction[0];
                self.player_position[1] += movement * direction[1];
                self.player_position[2] += movement * direction[2];
            }
        }
        //println!("The player's position is [{}, {}, {}].", self.player_position[0], self.player_position[1], self.player_position[2]);
    }

    pub fn get_relative_position(&self, position: [f32; 3]) -> [f32; 3] {
        let mut new_position: [f32; 3] = [0.0; 3];
        new_position[0] = position[0] - self.player_position[0];
        new_position[1] = position[1] - self.player_position[1];
        new_position[2] = position[2] - self.player_position[2];

        return new_position;
    }

    // pub fn get_relative_rotation(&self, rotation: [f32; 3]) -> [f32; 3] {
    //     // idk yet
    // }

    pub fn degrees_to_radians(&self, degrees: f32) -> f32 {
        return (f64::from(degrees) * std::f64::consts::PI / 180.0) as f32;
    }
    
    pub fn rotation_to_direction(&self, rotation: [f32; 3]) -> [f32; 3] {
        let x: f32 = self.degrees_to_radians(rotation[0]);
        let y: f32 = self.degrees_to_radians(rotation[1]);

        let mut direction: [f32; 3] = [0.0; 3];
        direction[0] = (y.cos() * x.sin() * 10000.0).round() / 10000.0;
        direction[1] = (y.sin() * 10000.0) / 10000.0;
        direction[2] = (y.cos() * x.cos() * 10000.0).round() / 10000.0;
    
        return direction;
    }
}