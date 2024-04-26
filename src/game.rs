use piston_window::keyboard::Key;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Block, Direction, Snake};

use rand::{thread_rng, Rng};

// In milliseconds
const MOVE_RATE: u128 = 200;
const GAME_OVER_RATE: u128 = 2000;

// Starting length of 3 blocks
const SNAKE_LEN: u32 = 3;

const BORDER_COLOR: Color = [0.2, 0.2, 0.2, 1.0];
const GAME_OVER_COLOR: Color = [1.0, 0.0, 0.0, 0.4];
const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

pub struct Game {
    snake: Snake,
    has_food: bool,
    food_x: u32,
    food_y: u32,
    game_over: bool,
    next_move: u128,
    width: u32,
    height: u32,
}

impl Game {
    /// New game
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            snake: Snake::new(SNAKE_LEN, width, height),
            has_food: false,
            food_x: 0,
            food_y: 0,
            game_over: false,
            next_move: Game::get_new_time(MOVE_RATE),
            width,
            height,
        }
    }

    /// See if snake hit border or own body
    fn detect_collision(&mut self) -> bool {
        let next_head = self.snake.get_next_head();

        if self.snake.bad_touch(&next_head) {
            self.next_move = Game::get_new_time(GAME_OVER_RATE);
            self.game_over = true;
            return true;
        }

        if next_head.x == 0
            || next_head.y == 0
            || next_head.x == self.width - 1
            || next_head.y == self.height - 1
        {
            self.next_move = Game::get_new_time(GAME_OVER_RATE);
            self.game_over = true;
            return true;
        }

        return false;
    }

    /// See if the snake ate the food
    fn ate_food(&self) -> bool {
        let next_head = self.snake.get_next_head();
        next_head.x == self.food_x && next_head.y == self.food_y
    }

    /// Update the UI and manage game state
    /// Detect collisions to end game
    /// Determine if Head and Apple match
    /// Draw borders, game over, snake, and food
    pub fn draw(&mut self, con: Context, g: &mut G2d) {
        if Game::get_time() > self.next_move {
            if self.game_over {
                self.restart();
            }

            if !self.has_food {
                let food = self.make_food();
                self.food_x = food.0;
                self.food_y = food.1;
                self.has_food = true;
            }

            if self.ate_food() {
                self.snake.grow_snake();
                self.has_food = false;
            }

            self.next_move = Game::get_new_time(MOVE_RATE);

            if !self.detect_collision() {
                self.snake.move_snake();
            }
        }

        if self.game_over {
            // games over, draw red screen tinting
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.height, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.width, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 1, 1, self.height - 1, con, g);
        draw_rectangle(BORDER_COLOR, 1, self.height - 1, self.width - 2, 1, con, g);

        self.draw_food(con, g);
        self.snake.draw(con, g);
    }

    /// Draw food at current food x and y
    fn draw_food(&self, con: Context, g: &mut G2d) {
        draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g)
    }

    /// Current time Plus some rate
    fn get_new_time(rate: u128) -> u128 {
        Game::get_time() + rate
    }

    /// Get current time
    fn get_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Someone broke time")
            .as_millis()
    }

    /// Detect key press and convert them to directions
    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::Up => self.snake.new_direction(Direction::UP),
            Key::Down => self.snake.new_direction(Direction::DOWN),
            Key::Left => self.snake.new_direction(Direction::LEFT),
            Key::Right => self.snake.new_direction(Direction::RIGHT),
            _ => (),
        };
    }

    /// Create a new tuple representing the X and Y of new food location
    fn make_food(&self) -> (u32, u32) {
        let mut rng = thread_rng();
        let mut x = rng.gen_range(1..self.width - 1);
        let mut y = rng.gen_range(1..self.height - 1);

        while self.snake.bad_touch(&Block { x, y }) {
            x = rng.gen_range(1..self.width - 1);
            y = rng.gen_range(1..self.height - 1);
        }

        (x, y)
    }

    /// Reset the game
    fn restart(&mut self) {
        self.snake = Snake::new(SNAKE_LEN, self.width, self.height);
        self.has_food = false;
        self.game_over = false;
    }
}
