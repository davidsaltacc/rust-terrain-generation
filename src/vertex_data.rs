use std::process::exit;



pub struct VertexData {
    pub length: u32,
    pub positions: Vec<[f32; 3]>
}

// create a plane (triangle_strip) of N*N quads where each quad is x*x big
fn plane_vertices(n: f32, x: f32) -> Vec<[f32; 3]> {
    let mut vertices = Vec::<[f32; 3]>::new();
    for nz in 0..(n as u32) {
        for nx in 0..=(n as u32) {
            vertices.push([nx as f32, 0., nz as f32]);
            vertices.push([nx as f32, 0., nz as f32 + 1.]);
        }
        if nz < (n as u32) - 1 {
            vertices.push([n as f32, 0., nz as f32 + 1.]);
            vertices.push([0., 0., nz as f32 + 1.]);
        }
    }
    return vertices.iter().map(|&vert| { 
        return [vert[0] * x, 0., vert[2] * x]; 
    }).collect();
}

pub fn render_dist_mul() -> f32 {
    match VertexData::RENDER_DISTANCE {
        0 => {
            return 0.5;
        }
        1 => {
            return 1.;
        }
        2 => {
            return 2.;
        }
        3 => {
            return 4.;
        },
        4 => {
            return 8.;
        }
        _ => {
            exit(-1); // no
        }
    }
}

impl VertexData {

    pub const QUALITY: u32 = 2; 
    // 0: ultralow (why), 1: low, 2: mid, 3: high, 4: ultra (god pc only)

    pub const RENDER_DISTANCE: u32 = 2;
    // 0: half, 1: default, 2: double, 3: quadruple, 4: 8x
    
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
        let mul = render_dist_mul();
        match Self::QUALITY {
            0 => {
                return plane_vertices(125. * mul, 0.04);
            }
            1 => {
                return plane_vertices(250. * mul, 0.02);
            }
            2 => {
                return plane_vertices(500. * mul, 0.01);
            }
            3 => {
                return plane_vertices(1000. * mul, 0.005);
            },
            4 => {
                return plane_vertices(2000. * mul, 0.0025);
            }
            _ => {
                exit(-1); // no
            }
        }
    }

}
