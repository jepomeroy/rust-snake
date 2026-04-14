use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;
use rand::{rng, Rng};

const SNAKE_COLOR: Color = [0.0, 1.0, 0.2, 1.0];

// Snakes possible Directions
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Check to prevent the snake for doubling back on itself
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

pub struct Snake {
    body: LinkedList<Block>,
    dir: Direction,
    tail: Option<Block>,
}

impl Snake {
    /// New Snake
    pub fn new(len: u32, width: u32, height: u32) -> Self {
        let dir = Snake::make_direction();
        let body = Snake::make_snake(len, width, height, &dir);

        Self {
            body,
            dir,
            tail: None,
        }
    }

    /// Determine if snake will touch the block, either head or an apple
    pub fn bad_touch(&self, head: &Block) -> bool {
        for block in &self.body {
            if block.x == head.x && block.y == head.y {
                return true;
            }
        }

        false
    }

    /// Draw snake
    pub fn draw(&self, con: Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g)
        }
    }

    /// Determine where the snake will be next
    pub fn get_next_head(&self) -> Block {
        let old_head = self.body.front().expect("I though snakes had heads");

        match self.dir {
            Direction::Up => Block {
                x: old_head.x,
                y: old_head.y - 1,
            },
            Direction::Down => Block {
                x: old_head.x,
                y: old_head.y + 1,
            },
            Direction::Right => Block {
                x: old_head.x + 1,
                y: old_head.y,
            },
            Direction::Left => Block {
                x: old_head.x - 1,
                y: old_head.y,
            },
        }
    }

    /// Add the old tail to the end of the snake
    pub fn grow_snake(&mut self) {
        if let Some(t) = self.tail {
            self.body.push_back(t);
        }
    }

    /// Create random direction
    fn make_direction() -> Direction {
        let mut rnd = rng();
        let dir = rnd.random_range(0..4);

        match dir {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        }
    }

    /// Create random position based on length, board size, and direction
    fn make_snake(len: u32, width: u32, height: u32, dir: &Direction) -> LinkedList<Block> {
        let mut body: LinkedList<Block> = LinkedList::new();

        let mut rnd = rng();
        let start_x = rnd.random_range(len + 2..width - 3);
        let start_y = rnd.random_range(len + 2..height - 3);

        for i in 0..len {
            match dir {
                Direction::Up => {
                    body.push_back(Block {
                        x: start_x,
                        y: start_y + i,
                    });
                }
                Direction::Down => {
                    body.push_back(Block {
                        x: start_x,
                        y: start_y - i,
                    });
                }
                Direction::Left => {
                    body.push_back(Block {
                        x: start_x + i,
                        y: start_y,
                    });
                }
                Direction::Right => {
                    body.push_back(Block {
                        x: start_x - i,
                        y: start_y,
                    });
                }
            }
        }

        body
    }

    /// Move the snake
    pub fn move_snake(&mut self) {
        let head = self.get_next_head();

        self.body.push_front(head);
        self.tail = self.body.pop_back();
    }

    /// Update the snake's direction
    pub fn new_direction(&mut self, dir: Direction) {
        if self.dir.opposite() == dir {
            return;
        }

        self.dir = dir;
    }
}
