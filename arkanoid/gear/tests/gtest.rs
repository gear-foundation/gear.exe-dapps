use futures::stream::StreamExt;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameResult};
use sails_rs::{
    calls::*,
    events::Listener,
    gtest::{calls::*, System},
};
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;
use vara_arkanoid_client::traits::*;
use vara_arkanoid_client::vara_arkanoid::events::{self, VaraArkanoidEvents};

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn simulate_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(vara_arkanoid::WASM_BINARY);

    let program_factory = vara_arkanoid_client::VaraArkanoidFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = vara_arkanoid_client::VaraArkanoid::new(remoting.clone());

    let mut listener = events::listener(remoting);

    let mut events = listener.listen().await.unwrap();

    let steps = 600;
    service_client
        .simulate_game(steps)
        .send_recv(program_id)
        .await
        .unwrap();

    // service_client
    //     .simulate_game(steps)
    //     .send_recv(program_id)
    //     .await
    //     .unwrap();

    let mut game_steps = Vec::new();
    for i in 0..steps {
        let event = events.next().await.unwrap();
        match event.1 {
            VaraArkanoidEvents::GameStep {
                ball,
                paddle,
                block_hits,
            } => {
                game_steps.push(GameStep {
                    ball_x: ball.x as f32,
                    ball_y: ball.y as f32,
                    ball_velocity_x: ball.velocity_x as f32,
                    ball_velocity_y: ball.velocity_y as f32,
                    paddle_x: paddle.x as f32,
                    paddle_y: paddle.y as f32,
                    block_hits: block_hits
                        .iter()
                        .map(|&(x, y)| (x as f32, y as f32))
                        .collect(),
                });
            }
            VaraArkanoidEvents::GameOver {
                paddle_hits,
                destroyed_blocks,
            } => {
                println!("paddle_hits {:?}", paddle_hits);
                println!("destroyed_blocks {:?}", destroyed_blocks);
                break;
            }
        }
    }

    println!("game steps: {:?}", game_steps.len());
    let file = File::create("/Users/luisa/vara-arkanoid/visualization/game_steps.json")
        .expect("Unable to create file");
    serde_json::to_writer(file, &game_steps).expect("Unable to write data to file");
}
#[derive(Serialize)]
struct GameStep {
    ball_x: f32,
    ball_y: f32,
    ball_velocity_x: f32,
    ball_velocity_y: f32,
    paddle_x: f32,
    paddle_y: f32,
    block_hits: Vec<(f32, f32)>,
}
