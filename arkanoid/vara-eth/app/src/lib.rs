#![no_std]

use sails_rs::{prelude::*, cell::RefCell};
mod game;
use game::{Game, BallType};
struct VaraArkanoidService<'a> {
    state: &'a RefCell<Game>,
}

impl <'a> VaraArkanoidService<'a> {
    pub fn create(state: &'a RefCell<Game>) -> Self {
        Self { state }
    }
    #[inline]
    pub fn get_mut(&self) -> sails_rs::cell::RefMut<'_, Game> {
        self.state.borrow_mut()
    }

    #[inline]
    pub fn get(&self) -> sails_rs::cell::Ref<'_, Game> {
        self.state.borrow()
    }
}

#[event]
#[derive(Encode, Decode, TypeInfo)]
pub enum Event {
    GameOver {
        paddle_hits: u32,
        destroyed_blocks: u32,
    },
}

#[sails_rs::service(events = Event)]
impl<'a> VaraArkanoidService<'a>{

    #[export]
    pub fn simulate_game(&mut self, num_steps: u32) {
        for _i in 0..num_steps {
            if let Some(event) = self.get_mut().update_game() {
                self.emit_eth_event(event).expect("Error during event emission");
            }
        }
    }

    #[export]
    pub fn ball_position(&self) -> BallType {
        let ball = self.get().ball.clone();
        (ball.x, ball.y, ball.radius, ball.velocity_x, ball.velocity_y)
    }
}

pub struct VaraArkanoidProgram {
    state: RefCell<Game>,
}

#[sails_rs::program]
impl VaraArkanoidProgram {
    // Program's constructor
    pub fn init() -> Self {
        Self {
            state: RefCell::new(Game::new()),
        }
    }

    // Exposed service
    pub fn vara_arkanoid(&self) -> VaraArkanoidService<'_> {
        VaraArkanoidService::create(&self.state)
    }
}
