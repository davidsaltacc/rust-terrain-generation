

pub struct VertexData {
    pub length: u32,
    pub positions: Vec<[f32; 3]>
}

fn plane_positions(n: u32, x: f32) -> Vec<[f32; 3]> {
    let mut vertices = Vec::<[f32; 3]>::new();
    for nz in 0..n {
        for nx in 0..=n {
            vertices.push([nx as f32, 0., nz as f32]);
            vertices.push([nx as f32, 0., nz as f32 + 1.]);
        }
        if nz < n - 1 {
            vertices.push([n as f32, 0., nz as f32 + 1.]);
            vertices.push([0., 0., nz as f32 + 1.]);
        }
    }
    return vertices.iter().map(|&vert| { 
        return [vert[0] * x, vert[1] * x, vert[2] * x]; 
    }).collect();
}


impl VertexData {
    
    pub fn new() -> Self {
        let positions = Self::vertex_positions();
        let length = positions.len() as u32;
        println!("Created {} vertices", length);
        return Self {
            length,
            positions
        };
    }

    fn vertex_positions() -> Vec<[f32; 3]> {
        return plane_positions(500, 0.05);
    }

}
