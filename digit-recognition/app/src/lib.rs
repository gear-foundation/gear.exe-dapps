#![no_std]
#![allow(static_mut_refs)]

use ndarray::{Array1, Array2, Array3};
use rust_decimal::Decimal;
use sails_rs::prelude::*;
struct DigitRecognitionService(());
pub mod tensor_funcs;
use tensor_funcs::*;

const GREYSCALE_SIZE: u32 = 255;

static mut STATE: Option<State> = None;

#[derive(Default)]
pub struct State {
    conv1_weights: Array2<Decimal>,
    conv1_bias: Array1<Decimal>,
    conv2_weights: Array2<Decimal>,
    conv2_bias: Array1<Decimal>,
    fc1_weights: Array2<Decimal>,
    fc1_bias: Array1<Decimal>,
    fc2_weights: Array2<Decimal>,
    fc2_bias: Array1<Decimal>,

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

    pub fn set_conv1_weights(&mut self, weights: Vec<FixedPoint>, bias: Vec<FixedPoint>) {
        let state = self.get_mut();

        state.conv1_weights =
            filter_to_matrix_from_flat(&fixed_points_to_decimal_vector(&weights), 8, 1, 5, 5);
        state.conv1_bias = Array1::from(bias).mapv(|fp| Decimal::new(fp.num as i64, fp.scale));
    }

    pub fn set_conv2_weights(&mut self, weights: Vec<FixedPoint>, bias: Vec<FixedPoint>) {
        let state = self.get_mut();
        state.conv2_weights =
            filter_to_matrix_from_flat(&fixed_points_to_decimal_vector(&weights), 8, 8, 5, 5);

        state.conv2_bias = Array1::from(bias).mapv(|fp| Decimal::new(fp.num as i64, fp.scale));
    }

    pub fn set_fc1_weights(&mut self, weights: Vec<FixedPoint>, bias: Vec<FixedPoint>) {
        let state = self.get_mut();
        state.fc1_weights = fixed_points_to_array2(weights, (64, 128));
        state.fc1_bias = Array1::from(bias).mapv(|fp| Decimal::new(fp.num as i64, fp.scale));
    }

    pub fn set_fc2_weights(&mut self, weights: Vec<FixedPoint>, bias: Vec<FixedPoint>) {
        let state = self.get_mut();
        state.fc2_weights = fixed_points_to_array2(weights, (10, 64));
        state.fc2_bias = Array1::from(bias).mapv(|fp| Decimal::new(fp.num as i64, fp.scale));
    }

    /// Converts raw pixels into a 3D tensor
    fn prepare_input(pixels: &Vec<u16>) -> Array2<Decimal> {
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

        im2col(&input, 5)
    }

    /// Applies the first convolutional layer
    pub fn predict(&mut self, pixels: Vec<u16>) {
        let input = Self::prepare_input(&pixels);
        let state = self.get_mut();

        // Step 1: First convolutional layer
        let mut x = apply_conv_layer(
            &input,
            &state.conv1_weights,
            &state.conv1_bias,
            24,
            2, // Apply max-pooling with stride 2
        );

        // Step 2: Second convolutional layer
        let input_col = im2col(&x, 5);
        x = apply_conv_layer(
            &input_col,
            &state.conv2_weights,
            &state.conv2_bias,
            8,
            2, // Apply max-pooling with stride 2
        );

        // Step 3: Flatten the result
        let x = flatten_single(&x);

        // Step 4: Fully connected layers
        let x = relu_1d(&linear_single(&x, &state.fc1_weights, &state.fc1_bias));
        let x = linear_single(&x, &state.fc2_weights, &state.fc2_bias);

        // Step 5: Compute softmax probabilities
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
