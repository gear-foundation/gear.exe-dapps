use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
    prelude::*,
};

use manager_client::traits::*;
use mandelbrot_checker_client::traits::*;
const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn generate_and_store_points() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(manager::WASM_BINARY);

    let program_factory = manager_client::ManagerFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = manager_client::Manager::new(remoting.clone());

    let width = 200;
    let height = 200;

    let x_min = String::from("-2.0");
    let x_max = String::from("1.0");
    let y_min = String::from("-1.5");
    let y_max = String::from("1.5");
    service_client
        .generate_and_store_points(width, height, x_min, x_max, y_min, y_max) // Call service's method (see app/src/lib.rs:14)
        .send_recv(program_id)
        .await
        .unwrap();

    let points = service_client.get_points().recv(program_id).await.unwrap();
    println!("{:?}", points.len());
}

#[tokio::test]
async fn add_checkers() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 200_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(manager::WASM_BINARY);
    let checker_code_id = remoting
        .system()
        .submit_code(mandelbrot_checker::WASM_BINARY);

    let checker_factory =
        mandelbrot_checker_client::MandelbrotCheckerFactory::new(remoting.clone());
    let mut checkers: Vec<ActorId> = Vec::new();

    for i in 0..100 {
        let program_id = checker_factory
            .new()
            .send_recv(checker_code_id, &[i])
            .await
            .unwrap();
        checkers.push(program_id.into());
    }

    let program_factory = manager_client::ManagerFactory::new(remoting.clone());

    let program_id = program_factory
        .new()
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = manager_client::Manager::new(remoting.clone());

    service_client
        .add_checkers(checkers)
        .send_recv(program_id)
        .await
        .unwrap();

    let checkers = service_client
        .get_checkers()
        .recv(program_id)
        .await
        .unwrap();
    println!("{:?}", checkers.len());
}

#[tokio::test]
async fn check_points_set() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 200_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(manager::WASM_BINARY);
    let checker_code_id = remoting
        .system()
        .submit_code(mandelbrot_checker::WASM_BINARY);

    let checker_factory =
        mandelbrot_checker_client::MandelbrotCheckerFactory::new(remoting.clone());

    let mut checkers: Vec<ActorId> = Vec::new();

    for i in 0..100 {
        let program_id = checker_factory
            .new()
            .send_recv(checker_code_id, &[i])
            .await
            .unwrap();
        checkers.push(program_id.into());
    }
    let program_factory = manager_client::ManagerFactory::new(remoting.clone());

    let program_id = program_factory
        .new()
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = manager_client::Manager::new(remoting.clone());

    service_client
        .add_checkers(checkers)
        .send_recv(program_id)
        .await
        .unwrap();

    let checkers = service_client
        .get_checkers()
        .recv(program_id)
        .await
        .unwrap();

    assert_eq!(checkers.len(), 100);

    let width = 200;
    let height = 200;

    let x_min = String::from("-2.0");
    let x_max = String::from("1.0");
    let y_min = String::from("-1.5");
    let y_max = String::from("1.5");
    service_client
        .generate_and_store_points(width, height, x_min, x_max, y_min, y_max) // Call service's method (see app/src/lib.rs:14)
        .send_recv(program_id)
        .await
        .unwrap();

    let points = service_client.get_points().recv(program_id).await.unwrap();
    assert_eq!(points.len(), 40000);

    service_client
        .check_points_set(1000, 20)
        .send_recv(program_id)
        .await
        .unwrap();

    let msg_sent = service_client.points_sent().recv(program_id).await.unwrap();

    assert_eq!(msg_sent, 2000);

    service_client
        .check_points_set(1000, 20)
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .check_points_set(1000, 20)
        .send_recv(program_id)
        .await
        .unwrap();

    let results = service_client.get_results().recv(program_id).await.unwrap();
    println!("{:?}", results);
}
