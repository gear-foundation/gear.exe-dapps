#![no_std]

use ndarray::{Array1, Array2, Array3};
use rust_decimal::Decimal;
use sails_rs::{cell::RefCell, prelude::*};

pub mod tensor_funcs;
use tensor_funcs::*;

const GREYSCALE_SIZE: u32 = 255;
pub const WEIGHT_SCALE: u32 = 6;
pub const FC1_SCALE: u32 = 4;

pub type Quant = i32;
pub type QuantFc1 = i16;

#[derive(Default)]
pub struct State {
    conv1: Option<ConvLayer>,
    conv2: Option<ConvLayer>,
    fc1: Option<FcLayer>,
    fc2: Option<FcLayer>,
    result: Option<Vec<Quant>>,
}

#[derive(Clone)]
pub struct ConvLayer {
    pub weights: Array2<Decimal>,
    pub bias: Array1<Decimal>,
}

#[derive(Clone)]
pub struct FcLayer {
    pub weights: Array2<Decimal>,
    pub bias: Array1<Decimal>,
}

pub struct DigitRecognitionService<'a> {
    state: &'a RefCell<State>,
}

impl<'a> DigitRecognitionService<'a> {
    pub fn create(state: &'a RefCell<State>) -> Self {
        Self { state }
    }
    #[inline]
    pub fn get_mut(&self) -> sails_rs::cell::RefMut<'_, State> {
        self.state.borrow_mut()
    }

    #[inline]
    pub fn get(&self) -> sails_rs::cell::Ref<'_, State> {
        self.state.borrow()
    }
}

#[sails_rs::service]
impl<'a> DigitRecognitionService<'a> {
    #[export]
    pub fn set_conv1_weights(&mut self, weights: Vec<Quant>, bias: Vec<Quant>) {
        let mut state = self.get_mut();

        state.conv1 = Some(ConvLayer {
            weights: filter_to_matrix_from_flat(&i32_to_decimal_vector(&weights), 8, 1, 5, 5),
            bias: Array1::from(bias).mapv(|v| Decimal::new(v as i64, WEIGHT_SCALE)),
        });
    }

    #[export]
    pub fn set_conv2_weights(&mut self, weights: Vec<Quant>, bias: Vec<Quant>) {
        let mut state = self.get_mut();

        state.conv2 = Some(ConvLayer {
            weights: filter_to_matrix_from_flat(&i32_to_decimal_vector(&weights), 8, 8, 5, 5),
            bias: Array1::from(bias).mapv(|v| Decimal::new(v as i64, WEIGHT_SCALE)),
        });
    }

    #[export]
    pub fn set_fc1_weights(&mut self, weights: Vec<QuantFc1>, bias: Vec<QuantFc1>) {
        let mut state = self.get_mut();
        state.fc1 = Some(FcLayer {
            weights: quants16_to_array2_decimal(weights, (64, 128), FC1_SCALE),
            bias: Array1::from(bias).mapv(|v| Decimal::new(v as i64, FC1_SCALE)),
        });
    }

    #[export]
    pub fn set_fc2_weights(&mut self, weights: Vec<Quant>, bias: Vec<Quant>) {
        let mut state = self.get_mut();
        state.fc2 = Some(FcLayer {
            weights: quants_to_array2(weights, (10, 64)),
            bias: Array1::from(bias).mapv(|v| Decimal::new(v as i64, WEIGHT_SCALE)),
        });
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
    #[export]
    pub fn predict(&mut self, pixels: Vec<u16>) {
        let input_col = Self::prepare_input(&pixels);

        let probabilities: Vec<Decimal> = {
            let state = self.get();
            let (conv1, conv2, fc1, fc2) = state.layers();

            // Step 1: First convolutional layer
            let conv1_out = apply_conv_layer(
                &input_col,
                &conv1.weights,
                &conv1.bias,
                24,
                2, // Apply max-pooling with stride 2
            );

            // Step 2: Second convolutional layer
            let conv2_in_col = im2col(&conv1_out, 5);
            let conv2_out = apply_conv_layer(
                &conv2_in_col,
                &conv2.weights,
                &conv2.bias,
                8,
                2, // Apply max-pooling with stride 2
            );

            // Step 3: Flatten the result
            let flat_features = flatten_single(&conv2_out);
            // Step 4: Fully connected layers
            let hidden = relu_1d(&linear_single(&flat_features, &fc1.weights, &fc1.bias));
            let logits = linear_single(&hidden, &fc2.weights, &fc2.bias);

            // Step 5: Compute softmax probabilities
            softmax(&logits.to_vec())
        };

        let fixed_probs: Vec<Quant> = probabilities
            .into_iter()
            .map(|mut d| {
                d.rescale(WEIGHT_SCALE);
                i32::try_from(d.mantissa()).unwrap()
            })
            .collect();

        self.get_mut().result = Some(fixed_probs);
    }

    #[export]
    pub fn result(&self) -> Vec<Quant> {
        self.get().result.clone().unwrap_or_default()
    }

    #[export]
    pub fn layers_set(&self) -> (bool, bool, bool, bool) {
        let state = self.get();
        (
            state.conv1.is_some(),
            state.conv2.is_some(),
            state.fc1.is_some(),
            state.fc2.is_some(),
        )
    }
}

impl State {
    fn layers(&self) -> (&ConvLayer, &ConvLayer, &FcLayer, &FcLayer) {
        (
            self.conv1.as_ref().expect("conv1 not set"),
            self.conv2.as_ref().expect("conv2 not set"),
            self.fc1.as_ref().expect("fc1 not set"),
            self.fc2.as_ref().expect("fc2 not set"),
        )
    }
}

pub struct DigitRecognitionProgram {
    state: RefCell<State>,
}

#[sails_rs::program]
impl DigitRecognitionProgram {
    pub fn init() -> Self {
        Self {
            state: RefCell::new(State::default()),
        }
    }

    pub fn digit_recognition(&self) -> DigitRecognitionService<'_> {
        DigitRecognitionService::create(&self.state)
    }
}
