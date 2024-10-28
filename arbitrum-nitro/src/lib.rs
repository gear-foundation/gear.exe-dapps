// Allow `cargo stylus export-abi` to generate a main function.
extern crate alloc;

use alloy_sol_types::sol;
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{
    alloy_primitives::{I16, U16},
    console,
    evm::log,
    prelude::*,
    storage::{StorageBool, StorageI16, StorageU16, StorageVec},
};

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
pub const BALL_RADIUS: i16 = 10;

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

sol! {
    event GameOver(uint16 indexed paddle_hits, uint16 indexed destroyed_blocks);
}

#[storage]
#[entrypoint]
pub struct Game {
    ball: Ball,
    blocks: StorageVec<Block>,
    paddle: Paddle,
    paddle_hits: StorageU16,
    destroyed_blocks: StorageU16,
}

impl Game {
    pub fn update_game(&mut self) {
        // Move the ball
        self.ball
            .x
            .set(self.ball.x.get() + self.ball.velocity_x.get());
        self.ball
            .y
            .set(self.ball.y.get() + self.ball.velocity_y.get());

        // Move the paddle based on its direction and speed
        self.paddle.update_position();

        // Check if the ball collides with the screen edges and reverse its direction if needed
        if self.ball.x.get() - self.ball.radius.get() <= I16::ZERO
            || self.ball.x.get() + self.ball.radius.get() >= I16::unchecked_from(SCREEN_WIDTH)
        {
            self.ball.velocity_x.set(-self.ball.velocity_x.get());
        }
        if self.ball.y.get() - self.ball.radius.get() <= I16::ZERO {
            self.ball.velocity_y.set(-self.ball.velocity_y.get());
        }

        // Check if the ball hits the paddle
        if self.ball.y.get() + self.ball.radius.get() >= self.paddle.y.get()
            && self.ball.y.get() - self.ball.radius.get()
                <= self.paddle.y.get() + I16::unchecked_from(PADDLE_HEIGHT)
            && self.ball.x.get() >= self.paddle.x.get()
            && self.ball.x.get() <= self.paddle.x.get() + self.paddle.width.get()
        {
            self.ball.velocity_y.set(-self.ball.velocity_y.get());
            // to avoid sticking effect
            self.ball
                .y
                .set(self.paddle.y.get() - self.ball.radius.get());
            self.paddle_hits.set(self.paddle_hits.get() + U16::from(1));
        }

        // Check if the ball has missed the paddle
        if self.ball.y.get() - self.ball.radius.get() > I16::unchecked_from(SCREEN_HEIGHT) {
            // Game Over condition
            log(GameOver {
                paddle_hits: u16::try_from(self.paddle_hits.get()).unwrap(),
                destroyed_blocks: u16::try_from(self.destroyed_blocks.get()).unwrap(),
            });

            return;
        }

        let block_count = self.blocks.len();

        for i in 0..block_count {
            if let Some(mut block) = self.blocks.get_mut(i) {
                if !block.destroyed.get() {
                    if let Some((collision_x, collision_y)) = check_circle_rectangle_collision(
                        self.ball.x.get(),
                        self.ball.y.get(),
                        self.ball.radius.get(),
                        block.rect_x1.get(),
                        block.rect_y1.get(),
                        block.rect_x2.get(),
                        block.rect_y2.get(),
                    ) {
                        if collision_x {
                            self.ball.velocity_x.set(-self.ball.velocity_x.get());
                        }
                        if collision_y {
                            self.ball.velocity_y.set(-self.ball.velocity_y.get());
                        }
                        self.destroyed_blocks
                            .set(self.destroyed_blocks.get() + U16::from(1));
                        block.destroyed.set(true);
                    }
                }
            }
        }
    }
}

#[storage]
pub struct Ball {
    x: StorageI16,
    y: StorageI16,
    radius: StorageI16,
    velocity_x: StorageI16,
    velocity_y: StorageI16,
}

impl Ball {
    pub fn set(&mut self, x: I16, y: I16, velocity_x: I16, velocity_y: I16) {
        self.x.set(x);
        self.y.set(y);
        self.velocity_x.set(velocity_x);
        self.velocity_y.set(velocity_y);
        self.radius.set(I16::unchecked_from(BALL_RADIUS));
    }
}

#[storage]
pub struct Block {
    rect_x1: StorageI16,
    rect_y1: StorageI16,
    rect_x2: StorageI16,
    rect_y2: StorageI16,
    destroyed: StorageBool,
}

impl Block {
    pub fn set(&mut self, rect_x1: I16, rect_y1: I16) {
        self.rect_x1.set(rect_x1);
        self.rect_y1.set(rect_y1);
        self.rect_x2.set(rect_x1 + I16::unchecked_from(BLOCK_WIDTH));
        self.rect_y2
            .set(rect_y1 + I16::unchecked_from(BLOCK_HEIGHT));
        self.destroyed.set(false);
    }
}

#[storage]
pub struct Paddle {
    x: StorageI16,
    y: StorageI16,
    width: StorageI16,
    speed: StorageI16,
    direction: StorageI16,
}

impl Paddle {
    pub fn set(&mut self, x: I16, y: I16, speed: I16) {
        self.x.set(x);
        self.y.set(y);
        self.speed.set(speed);
        self.direction.set(I16::ONE);
        self.width.set(I16::unchecked_from(PADDLE_WIDTH));
    }

    pub fn update_position(&mut self) {
        self.x
            .set(self.x.get() + (self.speed.get() * self.direction.get()));

        // Reverse direction if the paddle reaches the screen edges
        if self.x.get() <= I16::ZERO
            || self.x.get() + self.width.get() >= I16::unchecked_from(SCREEN_WIDTH)
        {
            // Change direction
            self.direction.set(-self.direction.get());
        }
    }
}
#[public]
impl Game {
    pub fn init_game(&mut self) {
        self.ball.set(
            I16::unchecked_from(270 + PADDLE_WIDTH / 2 - 10),
            I16::unchecked_from(SCREEN_HEIGHT - PADDLE_HEIGHT - 20 - 20),
            I16::unchecked_from(6),
            I16::unchecked_from(-6),
        );
        self.paddle.set(
            I16::unchecked_from(270),
            I16::unchecked_from(SCREEN_HEIGHT - PADDLE_HEIGHT - 30),
            I16::unchecked_from(6),
        );

        // Iterate over the brick template to initialize blocks
        for (row, block_row) in BRICK_TEMPLATE.iter().enumerate() {
            for (col, &has_brick) in block_row.iter().enumerate() {
                if has_brick {
                    let x = HORIZONTAL_OFFSET + col as i16 * (BLOCK_WIDTH + BLOCK_MARGIN);
                    let y = VERTICAL_OFFSET + row as i16 * (BLOCK_HEIGHT + BLOCK_MARGIN);
                    self.blocks
                        .grow()
                        .set(I16::unchecked_from(x), I16::unchecked_from(y));
                }
            }
        }

        self.destroyed_blocks.set(U16::ZERO);
        self.paddle_hits.set(U16::ZERO);
    }

    pub fn simulate_game(&mut self, num_steps: u32) {
        for _i in 0..num_steps {
            self.update_game();
        }
    }

    pub fn paddle_hits(&self) -> u16 {
        u16::try_from(self.paddle_hits.get()).unwrap()
    }

    pub fn destoryed_blocks(&self) -> u16 {
        u16::try_from(self.destroyed_blocks.get()).unwrap()
    }
}

fn check_circle_rectangle_collision(
    circle_x: I16,
    circle_y: I16,
    radius: I16,
    rect_x1: I16,
    rect_y1: I16,
    rect_x2: I16,
    rect_y2: I16,
) -> Option<(bool, bool)> {
    let nearest_x = rect_x1.max(circle_x.min(rect_x2));
    let nearest_y = rect_y1.max(circle_y.min(rect_y2));

    let distance_x = i64::try_from(circle_x - nearest_x).unwrap();
    let distance_y = i64::try_from(circle_y - nearest_y).unwrap();
    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let radius_squared = i64::try_from(radius * radius).unwrap();

    if distance_squared <= radius_squared {
        let collision_x = nearest_x == rect_x1 || nearest_x == rect_x2;
        let collision_y = nearest_y == rect_y1 || nearest_y == rect_y2;
        Some((collision_x, collision_y))
    } else {
        None
    }
}

/// Unit tests
#[cfg(test)]
mod tests {
    use stylus_sdk::contract;

    use super::*;

    //     #[motsu::test]
    //     fn it_gets_number(contract: Counter) {
    //         let number = contract.number();
    //         assert_eq!(U256::ZERO, number);
    //     }

    #[motsu::test]
    fn init_game(contract: Game) {
        contract.init_game();
    }

    #[motsu::test]
    fn simulate_game(contract: Game) {
        contract.init_game();

        for i in 0..600 {
            contract.simulate_game(1);
        }
    }

    //     #[motsu::test]
    //     fn it_multiplies(contract: Counter) {
    //         contract.set_number(U256::from(5));
    //         contract.mul_number(U256::from(2));
    //         let number = contract.number();
    //         assert_eq!(U256::from(10), number);
    //     }

    //     #[motsu::test]
    //     fn it_adds(contract: Counter) {
    //         contract.set_number(U256::from(5));
    //         contract.add_number(U256::from(2));
    //         let number = contract.number();
    //         assert_eq!(U256::from(7), number);
    //     }

    //     #[motsu::test]
    //     fn it_increments(contract: Counter) {
    //         contract.set_number(U256::from(5));
    //         contract.increment();
    //         let number = contract.number();
    //         assert_eq!(U256::from(6), number);
    //     }
}
