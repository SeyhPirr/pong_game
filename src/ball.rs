use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};

const BALL_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub coordinate: Coordinate,
    pub velocity: Velocity,
}

impl Ball {
    pub fn new(coordinate: Coordinate) -> Ball {
        Ball {
            coordinate,
            velocity: Velocity { x: 1, y: 1 },
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.coordinate.x, self.coordinate.y, con, g);
    }
    pub fn move_ball(&mut self) {
        self.coordinate = Coordinate {
            x: self.coordinate.x + self.velocity.x,
            y: self.coordinate.y + self.velocity.y,
        }
    }
}
