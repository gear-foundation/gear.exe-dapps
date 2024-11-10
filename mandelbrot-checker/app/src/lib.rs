#![no_std]

use rust_decimal::Decimal;
use sails_rs::{gstd::msg, prelude::*};
struct MandelbrotCheckerService(());

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Point {
    pub c_re: String,
    pub c_im: String,
}

#[sails_rs::service]
impl MandelbrotCheckerService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn check_mandelbrot_points(&mut self, points: Vec<Point>, max_iter: u32) {
        let results: Vec<u32> = points
            .clone()
            .into_iter()
            .map(|point| {
                let c_re = Decimal::from_str_exact(&point.c_re).expect("Error: convert to Decimal");
                let c_im = Decimal::from_str_exact(&point.c_im).expect("Error: convert to Decimal");

                self.check_mandelbrot(c_re, c_im, max_iter)
            })
            .collect();
        let payload = [
            "Manager".encode(),
            "ResultCalculated".encode(),
            (points, results).encode(),
        ]
        .concat();
        msg::send_bytes(msg::source(), payload, 0).expect("Error during msg sending");
    }

    fn check_mandelbrot(&self, c_re: Decimal, c_im: Decimal, max_iter: u32) -> u32 {
        let mut z_re = c_re;
        let mut z_im = c_im;

        // Threshold
        let threshold = Decimal::from(4);

        for _i in 0..max_iter {
            let modulus_squared = z_re * z_re + z_im * z_im;
            if modulus_squared > threshold {
                return _i;
            }

            // z: z = z^2 + c
            let new_re = z_re * z_re - z_im * z_im + c_re;
            z_im = Decimal::from(2) * z_re * z_im + c_im;
            z_re = new_re;
        }

        max_iter
    }
}

pub struct MandelbrotCheckerProgram(());

#[sails_rs::program]
impl MandelbrotCheckerProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn mandelbrot_checker(&self) -> MandelbrotCheckerService {
        MandelbrotCheckerService::new()
    }
}
