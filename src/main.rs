use piston_window::*;
use piston_window::types::Color;
use piston_window::Text;

use crate::draw::to_coord_u32;
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

mod draw;
mod game;
mod pong;
mod ball;

use game::Game;

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Pong Game", [
        to_coord_u32(width),
        to_coord_u32(height),
    ])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| { game.update(arg.dt) });
    }
}
