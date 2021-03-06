use piston_window::*;
use piston_window::types::Color;
use rand::{Rng, thread_rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

// food: red
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];

// board around board: black
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

// translucent red overlay on loss
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

// How often to force a move, in seconds
const MOVING_PERIOD: f64 = 0.1;

// How long before a lost game restarts
const RESTART_TIME: f64 = 1.0;


pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    // how much longer to wait until moving
    waiting_time: f64,
}

impl Game {
    /// Create new game with a snake and food
    pub fn new(width: i32, height: i32) -> Game {
        let mut game = Game {
            snake: Snake::new(2, width - 2, 2, height - 2),
            waiting_time: 0.0,
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            game_over: false,
        };
        game.add_food();
        return game;
    }

    /// if arrow key pressed, update snake direction & move snake
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => return
        };

        // snake can't make an immediate U turn
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        // immediately move snake --- snake also moves every "tick"
        self.update_snake(dir);
    }

    /// draw game board, snake and (if lost) red overlay
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    /// a "tick": if time, move snake/restart/add food
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    /// if on food, consume it & lengthen snake
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.grow_snake();
        }
    }

    /// Return T/F for living snake: dies from intersecting self or hitting wall
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_snake(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    /// Adds a random food item to board
    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut x:i32 = -1;
        let mut y: i32 = -1;

        // find legal spot on board not part of snake
        while x == -1 || y == -1 || self.snake.overlap_snake(x, y) {
            x = rng.gen_range(1, self.width - 1);
            y = rng.gen_range(1, self.width - 1);
        }

        self.food_x = x;
        self.food_y = y;
        self.food_exists = true;
    }

    /// update snake on move/tick: decide if dead or eaten, and move
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Restart game
    fn restart(&mut self) {
        self.snake = Snake::new(1, self.width - 2, 1, self.height - 2);
        self.waiting_time = 0.0;
        self.game_over = false;
        self.add_food();
    }
}

