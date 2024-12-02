use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use mandelbrot_checker_client::{traits::*, FixedPoint, Point};

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
            index: 0,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint {
                num: -135,
                scale: 3,
            },
        },
        Point {
            index: 1,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint { num: 54, scale: 2 },
        },
        Point {
            index: 2,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 3,
            c_re: FixedPoint {
                num: -135,
                scale: 3,
            },
            c_im: FixedPoint { num: 27, scale: 2 },
        },
        Point {
            index: 4,
            c_re: FixedPoint { num: 135, scale: 3 },
            c_im: FixedPoint { num: 265, scale: 3 },
        },
        Point {
            index: 5,
            c_re: FixedPoint { num: 265, scale: 3 },
            c_im: FixedPoint { num: 248, scale: 3 },
        },
        Point {
            index: 6,
            c_re: FixedPoint { num: 295, scale: 3 },
            c_im: FixedPoint { num: 24, scale: 2 },
        },
        Point {
            index: 7,
            c_re: FixedPoint { num: 31, scale: 2 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 8,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint {
                num: -135,
                scale: 3,
            },
        },
        Point {
            index: 9,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint { num: 54, scale: 2 },
        },
        Point {
            index: 10,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 11,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 27, scale: 2 },
        },
        Point {
            index: 12,
            c_re: FixedPoint { num: 135, scale: 3 },
            c_im: FixedPoint { num: 265, scale: 3 },
        },
        Point {
            index: 13,
            c_re: FixedPoint { num: 265, scale: 3 },
            c_im: FixedPoint { num: 248, scale: 3 },
        },
        Point {
            index: 14,
            c_re: FixedPoint { num: 295, scale: 3 },
            c_im: FixedPoint { num: 24, scale: 2 },
        },
        Point {
            index: 15,
            c_re: FixedPoint { num: 31, scale: 2 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 16,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint {
                num: -135,
                scale: 3,
            },
        },
        Point {
            index: 17,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint { num: 54, scale: 2 },
        },
        Point {
            index: 18,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 19,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 27, scale: 2 },
        },
        Point {
            index: 20,
            c_re: FixedPoint { num: 135, scale: 3 },
            c_im: FixedPoint { num: 265, scale: 3 },
        },
        Point {
            index: 21,
            c_re: FixedPoint { num: 265, scale: 3 },
            c_im: FixedPoint { num: 248, scale: 3 },
        },
        Point {
            index: 22,
            c_re: FixedPoint { num: 295, scale: 3 },
            c_im: FixedPoint { num: 24, scale: 2 },
        },
        Point {
            index: 23,
            c_re: FixedPoint { num: 31, scale: 2 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 24,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint {
                num: -135,
                scale: 3,
            },
        },
        Point {
            index: 25,
            c_re: FixedPoint { num: 25, scale: 2 },
            c_im: FixedPoint { num: 54, scale: 2 },
        },
        Point {
            index: 25,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 26,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 25, scale: 2 },
        },
        Point {
            index: 27,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 135, scale: 3 },
        },
        Point {
            index: 28,
            c_re: FixedPoint { num: 355, scale: 3 },
            c_im: FixedPoint { num: 25, scale: 2 },
        },
    ];
    service_client
        .check_mandelbrot_points(points, 1000)
        .send_recv(program_id)
        .await
        .unwrap();
}
