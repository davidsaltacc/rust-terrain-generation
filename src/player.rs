// I've shed tears of pain while writing this just so you know

struct Player {
    player_position: [f32; 3],
    camera_rotation: [f32; 3],
}

impl Player {
    fn new(player_position: [f32; 3], camera_rotation: [f32; 3]) -> Self {
        Self { player_position, camera_rotation }
    }

    // you will need to call this every frame
    fn update() {

    }

    fn getAbsolutePosition(position: [f32; 3]) {
        return position - player_position;
    }
}