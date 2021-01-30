use glam::Vec2;
use oorandom::Rand32;

fn vec_from_angle(angle: f32) -> Vec2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2::new(vx, vy)
}

pub fn random_angle(rng: &mut Rand32) -> Vec2 {
    let angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
    vec_from_angle(angle)
}
