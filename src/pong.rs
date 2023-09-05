use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

const PONG_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
    pub body: LinkedList<Block>,
}

impl Pong {
    pub fn new(x: i32, y: i32) -> Pong {
        let body = LinkedList::from([Block { x: x + 2, y }, Block { x: x + 1, y }, Block { x, y }]);
        Pong { body }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(PONG_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn update_position(&mut self, dir: Direction) {
        let head = self.body.front().unwrap();
        let tail = self.body.back().unwrap();

        match dir {
            Direction::Right => {
                if head.x >= 18 {
                    return;
                }
                let new_block = Block {
                    x: head.x + 1,
                    y: head.y,
                };
                self.body.push_front(new_block);
                self.body.pop_back().unwrap();
            }
            Direction::Left => {
                if tail.x <= 1 {
                    return;
                }
                let new_block = Block {
                    x: tail.x - 1,
                    y: tail.y,
                };
                self.body.push_back(new_block);
                self.body.pop_front().unwrap();
            }
        }
    }
}
