use piston_window::types::Color;
use piston_window::*;

use crate::ball::{Ball, Coordinate};
use crate::draw::draw_rectangle;
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
            ball: Ball::new(Coordinate { x: 10, y: 10 }),
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
            None => (),
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
            None => (),
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        let (collision, place) = self.check_collision();

        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            if collision && place == 1 {
                self.ball.velocity.y = -self.ball.velocity.y;
            }

            if collision && place == 2 {
                self.ball.velocity.x = -self.ball.velocity.x;
            }
            self.ball.move_ball();

            self.waiting_time = 0.0;
        }
    }
    fn check_collision(&self) -> (bool, i32) {
        let coordinate = self.ball.coordinate;
        let velocity = self.ball.velocity;

        for block in &self.pong1.body {
            if coordinate.y + velocity.y == block.y && coordinate.x + velocity.x == block.x {
                return (true, 1);
            }
        }
        for block in &self.pong2.body {
            if coordinate.y + velocity.y == block.y && coordinate.x + velocity.x == block.x {
                return (true, 1);
            }
        }
        if coordinate.x + velocity.x >= 19 || coordinate.x + velocity.x <= 0 {
            return (true, 2);
        }

        (false, 3)
    }
}
