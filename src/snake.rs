use std::collections::LinkedList;

use piston_window::{Context, G2d};
use piston_window::types::Color;
use rand::{Rng, thread_rng};

use Direction::*;

use crate::draw::draw_block;

// green
const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns opposite direction of one given
    pub fn opposite(&self) -> Direction {
        match *self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

/// Board game block
#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}

/// Snake.
/// - direction: Direction snake will move next move
/// - body: Linked list of blocks for body
/// - grow_tail: grow tail once (did we just eat food)
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    grow_tail: bool,
}

impl Snake {
    /// Create new snake within coords supplied
    pub fn new(low_x: i32, high_x: i32, low_y: i32, high_y: i32) -> Snake {
        let mut rng = thread_rng();
        let x = rng.gen_range(low_x, high_x - 2);
        let y = rng.gen_range(low_y, high_y);
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            body,
            grow_tail: false,
        }
    }

    /// Draw each block of snake
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    /// Return game board coords of snake head
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    /// Move snake forward: adds new head, trims tail
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // If direction given, switch -- else will move in same direction
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Up => Block { x: last_x, y: last_y - 1 },
            Down => Block { x: last_x, y: last_y + 1 },
            Left => Block { x: last_x - 1, y: last_y },
            Right => Block { x: last_x + 1, y: last_y },
        };

        self.body.push_front(new_block);

        // if we just ate food, we will grow by not popping last
        if !self.grow_tail {
            self.body.pop_back();
        } else {
            self.grow_tail = false;
        }
    }

    /// Getter for snake direction
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Location for next head, given current direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {
            Up => (head_x, head_y - 1),
            Down => (head_x, head_y + 1),
            Left => (head_x - 1, head_y),
            Right => (head_x + 1, head_y),
        }
    }

    /// "Grow" snake
    pub fn grow_snake(&mut self) {
        self.grow_tail = true;
    }

    /// Does cell at given coords intersect with snake?
    pub fn overlap_snake(&self, x: i32, y: i32) -> bool {
        let mut idx = 0;
        for block in &self.body {
            if x == block.x && y == block.y { return true; }

            idx += 1;
            // Don't count the tail; it's ok touch that
            if idx == self.body.len() - 1 { break; }
        }
        return false;
    }
}