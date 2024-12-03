#![no_std]

use rust_decimal::Decimal;
use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use sails_rs::prelude::*;
struct DigitRecognitionService(());
pub mod weights_and_biases;
use crate::weights_and_biases::*;
const GREYSCALE_SIZE: u32 = 255;

#[derive(Encode, Decode, TypeInfo)]
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

enum Layer {
    Weights(Vec<Vec<Decimal>>),
    Biases(&'static [Decimal]),
}

#[sails_rs::service]
impl DigitRecognitionService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn predict(&mut self, pixels: Vec<u16>) -> Vec<FixedPoint> {
        assert!(
            pixels.iter().all(|&x| x <= GREYSCALE_SIZE as u16),
            "Pixels contain values outside the [0, 255] range"
        );
        assert_eq!(pixels.len(), 784, "Wrong picture size");

        let gr_size = Decimal::new(GREYSCALE_SIZE as i64, 0);

        let mut layer_input: Vec<Decimal> = pixels
            .iter()
            .map(|&x| {
                Decimal::new(x as i64, 0)
                    .checked_div(gr_size)
                    .expect("Division error")
            })
            .collect();

        let layers = vec![
            (
                Layer::Weights(reshape_weights(&W0, 784, 128)),
                Layer::Biases(&B0),
            ),
            (
                Layer::Weights(reshape_weights(&W1, 128, 64)),
                Layer::Biases(&B1),
            ),
            (
                Layer::Weights(reshape_weights(&W2, 64, 32)),
                Layer::Biases(&B2),
            ),
            (
                Layer::Weights(reshape_weights(&W3, 32, 10)),
                Layer::Biases(&B3),
            ),
        ];

        for (i, layer) in layers.iter().enumerate() {
            let (weights, biases) = match layer {
                (Layer::Weights(weights), Layer::Biases(biases)) => (weights, biases),
                _ => panic!("Invalid layer structure"),
            };

            let linear_output = add_biases(&vec_matmul(&layer_input, weights), biases);

            layer_input = if i < layers.len() - 1 {
                relu(&linear_output)
            } else {
                linear_output
            };
        }

        let probability = softmax(&layer_input);
        probability
            .iter()
            .map(|a| FixedPoint::from_decimal(a))
            .collect()
    }
}

pub struct DigitRecognitionProgram(());

#[sails_rs::program]
impl DigitRecognitionProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn digit_recognition(&self) -> DigitRecognitionService {
        DigitRecognitionService::new()
    }
}

fn reshape_weights(weights: &[Decimal], rows: usize, cols: usize) -> Vec<Vec<Decimal>> {
    assert_eq!(
        weights.len(),
        rows * cols,
        "Weights size does not match the specified rows and cols"
    );
    weights.chunks(cols).map(|chunk| chunk.to_vec()).collect()
}

fn add_biases(values: &[Decimal], biases: &[Decimal]) -> Vec<Decimal> {
    values
        .iter()
        .zip(biases)
        .map(|(val, bias)| *val + *bias)
        .collect()
}

fn relu(values: &[Decimal]) -> Vec<Decimal> {
    values
        .iter()
        .map(|x| {
            if *x > Decimal::ZERO {
                *x
            } else {
                Decimal::ZERO
            }
        })
        .collect()
}

fn exp_approx(x: Decimal, tolerance: Decimal) -> Decimal {
    let mut term = Decimal::ONE;
    let mut sum = Decimal::ONE;
    let mut n = 1;
    sails_rs::gstd::debug!("x {:?}", x);
    if x < Decimal::new(-15, 0) {
        return Decimal::ZERO;
    }
    loop {
        term *= x / Decimal::from(n);
        if term.abs() < tolerance {
            break;
        }
        sum += term;
        n += 1;
    }

    sails_rs::gstd::debug!("sum {:?}", sum);
    sum
}

fn softmax(values: &[Decimal]) -> Vec<Decimal> {
    let max_val = values.iter().cloned().max().unwrap_or(Decimal::ZERO);

    let tolerance = dec!(0.000001);

    let exp_values: Vec<Decimal> = values
        .iter()
        .map(|x| exp_approx(*x - max_val, tolerance)) // Сдвиг значений
        .collect();
    let sum: Decimal = exp_values.iter().sum();
    exp_values.iter().map(|x| *x / sum).collect()
}

fn dot(a: &[Decimal], b: &[Decimal]) -> Decimal {
    assert_eq!(a.len(), b.len(), "The size of vectors must be equal");

    a.iter().zip(b).map(|(x, y)| *x * *y).sum()
}

fn transpose(matrix: &[Vec<Decimal>]) -> Vec<Vec<Decimal>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut transposed = vec![vec![Decimal::ZERO; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }
    transposed
}

fn vec_matmul(vector: &[Decimal], matrix: &[Vec<Decimal>]) -> Vec<Decimal> {
    assert!(
        matrix.len() == vector.len(),
        "Vector size must be equal to the amount of rows"
    );

    let transposed = transpose(matrix);

    transposed
        .iter()
        .map(|column| dot(vector, column))
        .collect()
}
