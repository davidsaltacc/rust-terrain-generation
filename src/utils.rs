pub fn degrees_to_radians(degrees: f32) -> f32 {
    return (f64::from(degrees) * std::f64::consts::PI / 180.0) as f32;
}

pub fn rotation_to_direction(rotation: [f32; 3]) -> [f32; 3] {
    let x: f32 = degrees_to_radians(rotation[0]);
    let y: f32 = degrees_to_radians(rotation[1]);

    let mut direction: [f32; 3] = [0.0; 3];
    direction[0] = (y.cos() * x.sin() * 10000.0).round() / 10000.0;
    direction[1] = (y.sin() * 10000.0) / 10000.0;
    direction[2] = (y.cos() * x.cos() * 10000.0).round() / 10000.0;

    return direction;
}

pub fn sqr_magnitude(vector: [f32; 3]) -> f32 {
    return vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
}

pub fn magnitude(vector: [f32; 3]) -> f32 {
    return (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
}

pub fn normalize(vector: [f32; 3]) -> [f32; 3] {
    let magnitude: f32 = magnitude(vector);

    let mut normalized: [f32; 3] = [0.0; 3];
    normalized[0] = vector[0] / magnitude;
    normalized[1] = vector[1] / magnitude;
    normalized[2] = vector[2] / magnitude;

    return normalized;
}