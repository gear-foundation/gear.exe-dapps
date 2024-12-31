use gclient::ext::sp_runtime::traits::Hash;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
    prelude::*,
};

use manager_client::{traits::*, FixedPoint};
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
        .new()
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = manager_client::Manager::new(remoting.clone());

    let width = 600;
    let height = 600;

    for _i in 0..12 {
        service_client
            .generate_and_store_points(
                width,
                height,
                FixedPoint { num: -2, scale: 0 },
                FixedPoint { num: 1, scale: 0 },
                FixedPoint { num: -15, scale: 2 },
                FixedPoint { num: 15, scale: 1 },
                30_000,
                false,
                true,
                1000,
                20,
            )
            .send_recv(program_id)
            .await
            .unwrap();
    }

    let points_len = service_client
        .get_points_len()
        .recv(program_id)
        .await
        .unwrap();

    assert_eq!(points_len, 360_000);

    let mut points: sails_rs::collections::HashMap<u32, u32> = sails_rs::collections::HashMap::with_capacity(10_000_000);
    println!("capacity {:?}", points.capacity());
    for i in 0..5_000_000 {
        points.insert(i, 2);
    }
    println!("capacity {:?}", points.capacity());
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
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

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

    let width = 600;
    let height = 600;

    for _i in 0..12 {
        service_client
            .generate_and_store_points(
                width,
                height,
                FixedPoint { num: -2, scale: 0 },
                FixedPoint { num: 1, scale: 0 },
                FixedPoint { num: -15, scale: 2 },
                FixedPoint { num: 15, scale: 1 },
                30_000,
                false,
                false,
                0,
                0,
            )
            .send_recv(program_id)
            .await
            .unwrap();
    }

    let points_len = service_client
        .get_points_len()
        .recv(program_id)
        .await
        .unwrap();

    assert_eq!(points_len, 360_000);

    for _i in 0..30 {
        println!("{:?}", _i);
        service_client
            .check_points_set(1000, 10, false)
            .send_recv(program_id)
            .await
            .unwrap();
        remoting.system().run_next_block();
    }

    let msg_sent = service_client.points_sent().recv(program_id).await.unwrap();

    assert_eq!(msg_sent, 30_000);

    let point_results = service_client
        .get_results(0, 30_000)
        .recv(program_id)
        .await
        .unwrap();
    if point_results.iter().all(|point| point.checked) {
        println!("All points are checked!");
    } else {
        let unchecked_count = point_results.iter().filter(|point| !point.checked).count();
        println!(
            "Some points are not checked. Unchecked points: {}",
            unchecked_count
        );
    }
}
