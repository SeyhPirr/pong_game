use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use rand::{thread_rng, Rng};

const BALL_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
pub struct Ball {
    pub ball_x: i32,
    pub ball_y: i32,
    pub vel_x: i32,
    pub vel_y: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Ball {
        let mut rng = thread_rng();
        let mut vel_y = 1;
        let mut vel_x = 1;

        Ball {
            ball_x: x,
            ball_y: y,
            vel_x,
            vel_y,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_block(BALL_COLOR, self.ball_x, self.ball_y, con, g);
    }
    pub fn move_ball(&mut self) {
        self.ball_x += self.vel_x;
        self.ball_y += self.vel_y;
    }
}
