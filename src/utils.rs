use crate::vector::Vector3;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    return (f64::from(degrees) * std::f64::consts::PI / 180.) as f32;
}

pub fn rotation_to_direction(rotation: Vector3) -> Vector3 {
    let x: f32 = degrees_to_radians(rotation.x);
    let y: f32 = degrees_to_radians(rotation.y);

    let mut direction: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    direction.x = (y.cos() * x.sin() * 1000.).round() / 1000.;
    direction.y = (y.sin() * 1000.) / 1000.;
    direction.z = (y.cos() * x.cos() * 1000.).round() / 1000.;

    return direction;
}

#[allow(unused)] // TODO remove this once it is used
pub fn sqr_magnitude(vector: Vector3) -> f32 {
    return vector.x * vector.x + vector.y * vector.y + vector.z * vector.z;
}

pub fn magnitude(vector: Vector3) -> f32 {
    return (vector.x * vector.x + vector.y * vector.y + vector.z * vector.z).sqrt();
}

pub fn normalize(vector: Vector3) -> Vector3 {
    let magnitude: f32 = magnitude(vector);

    let normalized: Vector3 = vector / magnitude;
    return normalized;
}