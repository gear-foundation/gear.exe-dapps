#![no_std]
#![allow(static_mut_refs)]

use ndarray::{Array1, Array3, Array4};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

use sails_rs::gstd::{exec, msg};
use sails_rs::prelude::*;
struct DigitRecognitionService(());
pub mod tensor_funcs;
pub mod weights_and_biases;
use crate::weights_and_biases::*;
use tensor_funcs::*;
const GREYSCALE_SIZE: u32 = 255;

static mut STATE: Option<State> = None;

#[derive(Default)]
pub struct State {
    x: Array3<Decimal>,
    result: Option<Vec<FixedPoint>>,
}
#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i128,
    pub scale: u32,
}

impl FixedPoint {
    pub fn from_decimal(decimal: &Decimal) -> Self {
        FixedPoint {
            num: decimal.mantissa(),
            scale: decimal.scale(),
        }
    }
}

impl DigitRecognitionService {
    pub fn init() -> Self {
        unsafe { STATE = Some(State::default()) }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut State {
        unsafe { STATE.as_mut().expect("STATE is not initialized") }
    }
    pub fn get(&self) -> &'static State {
        unsafe { STATE.as_ref().expect("STATE is not initialized") }
    }
}

#[sails_rs::service]
impl DigitRecognitionService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn predict(&mut self, pixels: Vec<u16>, continue_calc: bool) {
        self.conv1(pixels, continue_calc);
    }

    /// Converts raw pixels into a 3D tensor
    fn prepare_input(pixels: &Vec<u16>) -> Array3<Decimal> {
        assert!(
            pixels.len() == 784,
            "Input size mismatch: expected 784, got {}",
            pixels.len()
        );
        assert!(
            pixels.iter().all(|&x| x <= GREYSCALE_SIZE as u16),
            "Pixels contain values outside [0, {}]",
            GREYSCALE_SIZE
        );

        let mut input = Array3::<Decimal>::zeros((1, 28, 28));
        let gr_size = Decimal::new(GREYSCALE_SIZE as i64, 0);

        for (idx, &pixel) in pixels.iter().enumerate() {
            let x = idx % 28;
            let y = idx / 28;
            input[[0, y, x]] = Decimal::new(pixel as i64, 0) / gr_size;
        }

        input
    }

    /// Applies the first convolutional layer
    fn conv1(&mut self, pixels: Vec<u16>, continue_calc: bool) {
        let input = Self::prepare_input(&pixels);
        let weights = load_weights_4d(CONV1_WEIGHT);

        let state = self.get_mut();

        state.x = relu(&conv2d(&input, &weights, &CONV1_BIAS.to_vec()));
        state.x = max_pool2d_single(&state.x, 2);

        if continue_calc {
            let payload_bytes = [
                "DigitRecognition".encode(),
                "Conv2".encode(),
                continue_calc.encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload_bytes, 0)
                .expect("Error during msg sending");
        }
    }

    /// Applies the second convolutional layer
    pub fn conv2(&mut self, continue_calc: bool) {
        let weights = load_weights_4d(CONV2_WEIGHT);

        let state = self.get_mut();
        state.x = relu(&conv2d(&state.x, &weights, &CONV2_BIAS.to_vec()));
        state.x = max_pool2d_single(&state.x, 2);
        if continue_calc {
            let payload_bytes = ["DigitRecognition".encode(), "Finish".encode()].concat();
            msg::send_bytes(exec::program_id(), payload_bytes, 0)
                .expect("Error during msg sending");
        }
    }

    /// Applies fully connected layers and produces probabilities
    pub fn finish(&mut self) {
        let state = self.get_mut();
        let x = flatten_single(&state.x);

        // First fully connected layer
        let fc1_weights = load_weights_2d(FC1_WEIGHT);
        let mut x = relu_1d(&linear_single(
            &x,
            &fc1_weights,
            &Array1::from(FC1_BIAS.to_vec()),
        ));

        // Second fully connected layer
        let fc2_weights = load_weights_2d(FC2_WEIGHT);
        x = linear_single(&x, &fc2_weights, &&Array1::from(FC2_BIAS.to_vec()));

        // Compute softmax probabilities
        let probabilities = softmax(&x.to_vec());
        state.result = Some(
            probabilities
                .into_iter()
                .map(|dec| FixedPoint::from_decimal(&dec))
                .collect(),
        );
    }

    pub fn result(&self) -> Vec<FixedPoint> {
        self.get().result.clone().unwrap_or_default()
    }
}

fn relu(input: &Array3<Decimal>) -> Array3<Decimal> {
    input.mapv(|x| {
        if x > Decimal::zero() {
            x
        } else {
            Decimal::zero()
        }
    })
}

pub struct DigitRecognitionProgram(());

#[sails_rs::program]
impl DigitRecognitionProgram {
    pub fn new() -> Self {
        DigitRecognitionService::init();
        Self(())
    }

    pub fn digit_recognition(&self) -> DigitRecognitionService {
        DigitRecognitionService::new()
    }
}

fn load_weights_4d<const O: usize, const I: usize, const H: usize, const W: usize>(
    weights: [[[[Decimal; W]; H]; I]; O],
) -> Array4<Decimal> {
    Array4::from_shape_vec(
        (O, I, H, W),
        weights
            .iter()
            .flat_map(|out| out.iter())
            .flat_map(|inp| inp.iter())
            .flat_map(|row| row.iter())
            .cloned()
            .collect(),
    )
    .expect("Failed to convert 4D weights")
}