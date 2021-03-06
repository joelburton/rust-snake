use piston_window::*;
use piston_window::types::Color;

use draw::to_coord;
use game::Game;

mod draw;
mod snake;
mod game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];


fn main() {
    let (width, height) = (20,  20);
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord(width) as u32, to_coord(height) as u32],
    ).exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    // main event loop for the game
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
