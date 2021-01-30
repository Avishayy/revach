use ggez::graphics::Color;
use glam::Vec2;

const DEFAULT_SPEED: f32 = 3.0;

#[derive(Clone, Copy)]
pub enum Rotation {
    Left,
    Right,
}

pub struct Player {
    pub pos: Vec2,
    pub angle: Vec2,
    pub speed: f32,
    pub color: Color,
    pub rotation: Option<Rotation>,
}

impl Player {
    pub fn new(pos: Vec2, angle: Vec2, color: Color) -> Self {
        Player {
            pos,
            angle,
            speed: DEFAULT_SPEED,
            color,
            rotation: None,
        }
    }
}
