use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use mandelbrot_checker_client::{traits::*, Point};

const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn do_something_works() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting
        .system()
        .submit_code(mandelbrot_checker::WASM_BINARY);

    let program_factory =
        mandelbrot_checker_client::MandelbrotCheckerFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = mandelbrot_checker_client::MandelbrotChecker::new(remoting.clone());

    let points = vec![
        Point {
            c_re: "0.25".to_string(),
            c_im: "-0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "0.54".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.27".to_string(),
        },
        Point {
            c_re: "0.135".to_string(),
            c_im: "0.265".to_string(),
        },
        Point {
            c_re: "0.265".to_string(),
            c_im: "0.248".to_string(),
        },
        Point {
            c_re: "0.295".to_string(),
            c_im: "0.24".to_string(),
        },
        Point {
            c_re: "0.31".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "-0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "0.54".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.27".to_string(),
        },
        Point {
            c_re: "0.135".to_string(),
            c_im: "0.265".to_string(),
        },
        Point {
            c_re: "0.265".to_string(),
            c_im: "0.248".to_string(),
        },
        Point {
            c_re: "0.295".to_string(),
            c_im: "0.24".to_string(),
        },
        Point {
            c_re: "0.31".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "-0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "0.54".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.27".to_string(),
        },
        Point {
            c_re: "0.135".to_string(),
            c_im: "0.265".to_string(),
        },
        Point {
            c_re: "0.265".to_string(),
            c_im: "0.248".to_string(),
        },
        Point {
            c_re: "0.295".to_string(),
            c_im: "0.24".to_string(),
        },
        Point {
            c_re: "0.31".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "-0.135".to_string(),
        },
        Point {
            c_re: "0.25".to_string(),
            c_im: "0.54".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.27".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.135".to_string(),
        },
        Point {
            c_re: "0.355".to_string(),
            c_im: "0.27".to_string(),
        },
    ];
    service_client
        .check_mandelbrot_points(points, 1000)
        .send_recv(program_id)
        .await
        .unwrap();
}
