use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::ball::Ball;
use crate::draw::{draw_block, draw_rectangle};
use crate::pong::{Block, Direction, Pong};
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const MOVING_PERIOD: f64 = 0.2;

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pong1: Pong,
    pong2: Pong,
    ball: Ball,
    width: i32,
    height: i32,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            pong1: Pong::new(2, 2),
            pong2: Pong::new(15, 17),
            ball: Ball::new(10, 10),
            width,
            height,
            waiting_time: 0.0,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.pong1.draw(con, g);
        self.pong2.draw(con, g);
        self.ball.draw(con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.height - 1, 0, 1, self.height, con, g);
    }

    pub fn key_pressed(&mut self, key: Key) {
        let dir_1 = match key {
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _ => None,
        };
        let dir_2 = match key {
            Key::D => Some(Direction::Right),
            Key::A => Some(Direction::Left),
            _ => None,
        };
        self.update_pong1(dir_1);
        self.update_pong2(dir_2);
    }
    fn update_pong1(&mut self, dir: Option<Direction>) {
        let head = self.pong1.body.front().unwrap();
        let tail = self.pong1.body.back().unwrap();

        match dir {
            Some(Direction::Right) => {
                if head.x >= 18 {
                    return;
                }
                let new_block = Block {
                    x: head.x + 1,
                    y: head.y,
                };
                self.pong1.body.push_front(new_block);
                self.pong1.body.pop_back().unwrap();
            }
            Some(Direction::Left) => {
                if tail.x <= 1 {
                    return;
                }
                let new_block = Block {
                    x: tail.x - 1,
                    y: tail.y,
                };
                self.pong1.body.push_back(new_block);
                self.pong1.body.pop_front().unwrap();
            }
            None => {
                return;
            }
        }
    }
    fn update_pong2(&mut self, dir: Option<Direction>) {
        let head = self.pong2.body.front().unwrap();
        let tail = self.pong2.body.back().unwrap();

        match dir {
            Some(Direction::Right) => {
                if head.x >= 18 {
                    return;
                }
                let new_block = Block {
                    x: head.x + 1,
                    y: head.y,
                };
                self.pong2.body.push_front(new_block);
                self.pong2.body.pop_back().unwrap();
            }
            Some(Direction::Left) => {
                if tail.x <= 1 {
                    return;
                }
                let new_block = Block {
                    x: tail.x - 1,
                    y: tail.y,
                };
                self.pong2.body.push_back(new_block);
                self.pong2.body.pop_front().unwrap();
            }
            None => {
                return;
            }
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        let (collision, place) = self.check_collision();

        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            if collision && place == 1 {
                self.ball.vel_y = -self.ball.vel_y;
            }

            if collision && place == 2 {
                self.ball.vel_x = -self.ball.vel_x;
            }
            self.ball.move_ball();

            self.waiting_time = 0.0;
        }
    }
    fn check_collision(&self) -> (bool, i32) {
        let ball_x = self.ball.ball_x;
        let ball_y = self.ball.ball_y;
        let ball_vel_x = self.ball.vel_x;
        let ball_vel_y = self.ball.vel_y;

        for block in &self.pong1.body {
            if ball_y + ball_vel_y == block.y && ball_x + ball_vel_x == block.x {
                return (true, 1);
            }
        }
        for block in &self.pong2.body {
            if ball_y + ball_vel_y == block.y && ball_x + ball_vel_x == block.x {
                return (true, 1);
            }
        }
        if ball_x + ball_vel_x >= 19 || ball_x + ball_vel_x <= 0 {
            return (true, 2);
        }

        return (false, 3);
    }
}
