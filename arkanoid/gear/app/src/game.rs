use sails_rs::prelude::*;

use crate::Event;

pub const BLOCK_WIDTH: i16 = 40;
pub const BLOCK_HEIGHT: i16 = 30;
pub const BLOCK_MARGIN: i16 = 2;
pub const TOTAL_BLOCKS_WIDTH: i16 = BLOCK_WIDTH * 11 + BLOCK_MARGIN * 10; // Total width for 11 bricks with margins
pub const HORIZONTAL_OFFSET: i16 = (SCREEN_WIDTH - TOTAL_BLOCKS_WIDTH) / 2; // Center the bricks horizontally
pub const VERTICAL_OFFSET: i16 = 50;
pub const PADDLE_HEIGHT: i16 = 15;
pub const SCREEN_WIDTH: i16 = 800;
pub const SCREEN_HEIGHT: i16 = 800;
pub const PADDLE_WIDTH: i16 = 350;

const BRICK_TEMPLATE: [[bool; 11]; 16] = [
    [
        false, false, true, false, false, false, false, false, true, false, false,
    ],
    [
        false, false, true, false, false, false, false, false, true, false, false,
    ],
    [
        false, false, false, true, false, false, false, true, false, false, false,
    ],
    [
        false, false, false, true, false, false, false, true, false, false, false,
    ],
    [
        false, false, true, true, true, true, true, true, true, false, false,
    ],
    [
        false, false, true, false, true, true, true, false, true, false, false,
    ],
    [
        false, true, true, false, true, true, true, false, true, true, false,
    ],
    [
        false, true, true, true, true, true, true, true, true, true, false,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true,
    ],
    [
        true, true, true, true, true, true, true, true, true, true, true,
    ],
    [
        true, false, true, true, true, true, true, true, true, false, true,
    ],
    [
        true, false, true, false, false, false, false, false, true, false, true,
    ],
    [
        true, false, true, false, false, false, false, false, true, false, true,
    ],
    [
        false, false, false, true, true, false, true, true, false, false, false,
    ],
    [
        false, false, false, true, true, false, true, true, false, false, false,
    ],
];

#[derive(Default, Encode, Decode, TypeInfo, Clone)]
pub struct Ball {
    pub x: i16,
    pub y: i16,
    pub radius: i16,
    pub velocity_x: i16,
    pub velocity_y: i16,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            x: (270 + PADDLE_WIDTH / 2 - 10),
            y: SCREEN_HEIGHT - PADDLE_HEIGHT - 20 - 20,
            radius: 10,
            velocity_x: 6,
            velocity_y: -6,
        }
    }
}

#[derive(Default)]
pub struct Block {
    rect_x1: i16,
    rect_y1: i16,
    rect_x2: i16,
    rect_y2: i16,
}

impl Block {
    pub fn new(x1: i16, y1: i16) -> Self {
        Block {
            rect_x1: x1,
            rect_y1: y1,
            rect_x2: x1 + BLOCK_WIDTH,
            rect_y2: y1 + BLOCK_HEIGHT,
        }
    }
}

#[derive(Default, Encode, Decode, TypeInfo, Clone)]
pub struct Paddle {
    x: i16,
    y: i16,
    width: i16,
    speed: i16,
    direction: i16,
}

impl Paddle {
    pub fn new() -> Self {
        Paddle {
            x: 270,
            y: SCREEN_HEIGHT - PADDLE_HEIGHT - 30,
            width: 350,
            speed: 6,
            direction: 1,
        }
    }

    // Updates the paddle's position and reverses direction at screen boundaries
    pub fn update_position(&mut self) {
        self.x += self.speed * self.direction;

        // Reverse direction if the paddle reaches the screen edges
        if self.x <= 0 || self.x + self.width >= SCREEN_WIDTH {
            // Change direction
            self.direction = -self.direction;
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub ball: Ball,
    blocks: Vec<Block>,
    paddle: Paddle,
    paddle_hits: u32,
    destroyed_blocks: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut blocks = Vec::new();

        // Iterate over the brick template to initialize blocks
        for (row, block_row) in BRICK_TEMPLATE.iter().enumerate() {
            for (col, &has_brick) in block_row.iter().enumerate() {
                if has_brick {
                    let x = HORIZONTAL_OFFSET + col as i16 * (BLOCK_WIDTH + BLOCK_MARGIN);
                    let y = VERTICAL_OFFSET + row as i16 * (BLOCK_HEIGHT + BLOCK_MARGIN);
                    blocks.push(Block::new(x, y));
                }
            }
        }

        let paddle = Paddle::new();

        Game {
            ball: Ball::new(),
            blocks,
            paddle,
            ..Default::default()
        }
    }

    pub fn update_game(&mut self) -> Option<Event> {
        // Move the ball
        self.ball.x += self.ball.velocity_x;
        self.ball.y += self.ball.velocity_y;
        // Move the paddle based on its direction and speed
        self.paddle.update_position();

        // Check if the ball collides with the screen edges and reverse its direction if needed
        if self.ball.x - self.ball.radius <= 0 || self.ball.x + self.ball.radius >= SCREEN_WIDTH {
            self.ball.velocity_x = -self.ball.velocity_x;
        }
        if self.ball.y - self.ball.radius <= 0 {
            self.ball.velocity_y = -self.ball.velocity_y;
        }

        // Check if the ball hits the paddle
        if self.ball.y + self.ball.radius >= self.paddle.y
            && self.ball.y - self.ball.radius <= self.paddle.y + PADDLE_HEIGHT
            && self.ball.x >= self.paddle.x
            && self.ball.x <= self.paddle.x + self.paddle.width
        {
            self.ball.velocity_y = -self.ball.velocity_y;
            // to avoid sticking effect
            self.ball.y = self.paddle.y - self.ball.radius;
            self.paddle_hits += 1;
        }

        // Check if the ball has missed the paddle
        if self.ball.y - self.ball.radius > SCREEN_HEIGHT {
            // Game Over condition
            return Some(Event::GameOver {
                paddle_hits: self.paddle_hits,
                destroyed_blocks: self.destroyed_blocks,
            });
        }

        let mut block_hits = Vec::new();
        // Check if the ball collides with any blocks and stop once a collision is detected
        for block in self.blocks.iter_mut() {
            if let Some((collision_x, collision_y)) = check_circle_rectangle_collision(
                self.ball.x,
                self.ball.y,
                self.ball.radius,
                block.rect_x1,
                block.rect_y1,
                block.rect_x2,
                block.rect_y2,
            ) {
                if collision_x {
                    self.ball.velocity_x = -self.ball.velocity_x;
                }
                if collision_y {
                    self.ball.velocity_y = -self.ball.velocity_y;
                }
                // Remove the block from the game
                block_hits.push((block.rect_x1, block.rect_y1));
                self.destroyed_blocks += 1;
            }
        }

        self.blocks.retain(|block| {
            !block_hits
                .iter()
                .any(|hit| hit == &(block.rect_x1, block.rect_y1))
        });

        None
    }
}

// Function to check collision between the ball (circle) and blocks (rectangles)
fn check_circle_rectangle_collision(
    circle_x: i16,
    circle_y: i16,
    radius: i16,
    rect_x1: i16,
    rect_y1: i16,
    rect_x2: i16,
    rect_y2: i16,
) -> Option<(bool, bool)> {
    let nearest_x = rect_x1.max(circle_x.min(rect_x2));
    let nearest_y = rect_y1.max(circle_y.min(rect_y2));

    let distance_x = (circle_x - nearest_x) as i64;
    let distance_y = (circle_y - nearest_y) as i64;
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let radius_squared = (radius * radius) as i64;

    if distance_squared <= radius_squared {
        let collision_x = nearest_x == rect_x1 || nearest_x == rect_x2;
        let collision_y = nearest_y == rect_y1 || nearest_y == rect_y2;
        Some((collision_x, collision_y))
    } else {
        None
    }
}
