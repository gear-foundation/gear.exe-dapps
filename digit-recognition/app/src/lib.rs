#![no_std]

use ndarray::{s, Array, Array1, Array2, Array3, Array4, Axis};
use rust_decimal::prelude::Zero;
use rust_decimal::Decimal;

use rust_decimal::MathematicalOps;
use rust_decimal_macros::dec;
use sails_rs::gstd::services::Service;
use sails_rs::prelude::*;
struct DigitRecognitionService(());
pub mod weights_and_biases;
use crate::weights_and_biases::*;
const GREYSCALE_SIZE: u32 = 255;

static mut STATE: Option<State> = None;

#[derive(Default)]
pub struct State {
    x: Array3<Decimal>,
    partials: Vec<Array3<Decimal>>,
}
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

        let width = 28;
        let height = 28;
        let mut input = Array3::<Decimal>::zeros((1, height as usize, width as usize));

        for (idx, &pixel) in pixels.iter().enumerate() {
            let x = idx % width;
            let y = idx / width;

            // Преобразуем пиксель в Decimal и записываем в input
            input[[0, y, x]] = Decimal::new(pixel as i64, 0)
                .checked_div(gr_size)
                .expect("Division error")
        }

        let conv1_weight_vec: Vec<Vec<Vec<Vec<Decimal>>>> = CONV1_WEIGHT
            .into_iter()
            .map(|matrix| {
                matrix
                    .into_iter()
                    .map(|row| row.into_iter().map(|col| col.to_vec()).collect())
                    .collect()
            })
            .collect();

        let conv1_weight_as_array = conv1_weight_as_array(conv1_weight_vec).unwrap();


        let x = conv2d_single(&input, &conv1_weight_as_array, &CONV1_BIAS.to_vec());
        let x = relu(&x);
        let x = max_pool2d_single(&x, 2);

        self.get_mut().x = x;

        Vec::new()
    }

    // pub fn tanh_single(&mut self) {
    //     sails_rs::gstd::debug!("tanh_single");
    //     let state = self.get_mut();
    //     state.x = tanh_single(&state.x);
    //     state.x = max_pool2d_single(&state.x, 2);
    //     sails_rs::gstd::debug!("{:?}", sails_rs::gstd::exec::gas_available());
    // }

    pub fn conv2d_single(&mut self) {
        let state = self.get_mut();
        let array: Vec<Vec<Vec<Vec<Decimal>>>> = CONV2_WEIGHT
            .into_iter()
            .map(|matrix| {
                matrix
                    .into_iter()
                    .map(|row| row.into_iter().map(|col| col.to_vec()).collect())
                    .collect()
            })
            .collect();
        let conv2_weight_as_array = conv1_weight_as_array(array).unwrap();

        let x = conv2d_single(&state.x, &conv2_weight_as_array, &CONV2_BIAS.to_vec());
        let x = relu(&x);
        let x = max_pool2d_single(&x, 2);

        state.x = x;
    }

    pub fn finish(&mut self) -> Vec<FixedPoint> {
        let state = self.get_mut();
        let x = flatten_single(&state.x);
        let fc1_vec = FC1_WEIGHT.iter().map(|row| row.to_vec()).collect();

        let x = linear_single(
            &x,
            &fc1_weight_as_array(fc1_vec).unwrap(),
            &Array1::from_vec(FC1_BIAS.to_vec()),
        );
        let x = x.mapv(|v| {
            if v > Decimal::zero() {
                v
            } else {
                Decimal::zero()
            }
        });

        let fc2_vec = FC2_WEIGHT.iter().map(|row| row.to_vec()).collect();
        let x = linear_single(
            &x,
            &fc2_weight_as_array(fc2_vec).unwrap(),
            &Array1::from(FC2_BIAS.to_vec()),
        );
        let prob = softmax(&x.to_vec());
        prob.iter().map(|p| FixedPoint::from_decimal(p)).collect()
        
    }
}

fn flatten_single(input: &Array3<Decimal>) -> Array1<Decimal> {
    Array::from_shape_vec(input.len(), input.iter().cloned().collect()).unwrap()
}

fn fc1_weight_as_array(input: Vec<Vec<Decimal>>) -> Result<Array2<Decimal>, String> {
    let flattened: Vec<Decimal> = input
        .iter()
        .flat_map(|x| x.iter()) // Убираем вложенность
        .cloned() // Клонируем данные
        .collect(); // Собираем в одномерный вектор
    Array2::from_shape_vec((64, 8 * 4 * 4), flattened)
        .map_err(|e| format!("Error creating fc_weight array: {}", e))
}

fn fc2_weight_as_array(input: Vec<Vec<Decimal>>) -> Result<Array2<Decimal>, String> {
    let flattened: Vec<Decimal> = input
        .iter()
        .flat_map(|x| x.iter()) // Убираем вложенность
        .cloned() // Клонируем данные
        .collect(); // Собираем в одномерный вектор
    Array2::from_shape_vec((10, 64), flattened)
        .map_err(|e| format!("Error creating fc_weight array: {}", e))
}


fn linear_single(
    input: &Array1<Decimal>,
    weights: &Array2<Decimal>,
    bias: &Array1<Decimal>,
) -> Array1<Decimal> {
    sails_rs::gstd::debug!("{:?}", bias.dim());
    sails_rs::gstd::debug!("{:?}", input.dim());
    sails_rs::gstd::debug!("{:?}", weights.dim());
    let mut output = weights.dot(input);
    output += bias;
    output
}

fn convert_array_to_vec(input: [[[[Decimal; 5]; 5]; 1]; 6]) -> Vec<Vec<Vec<Vec<Decimal>>>> {
    input
        .into_iter()
        .map(|matrix| {
            matrix
                .into_iter()
                .map(|row| row.into_iter().map(|col| col.to_vec()).collect())
                .collect()
        })
        .collect()
}

fn tanh_decimal(x: Decimal) -> Decimal {
    let exp_x = x.exp();
    let exp_neg_x = (-x).exp();
    (exp_x - exp_neg_x) / (exp_x + exp_neg_x)
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

// fn tanh_single(input: &Array3<Decimal>) -> Array3<Decimal> {
//     input.mapv(|x| tanh_decimal(x))
// }

// fn fc1_weight_as_array(&self) -> Result<Array2<f64>, String> {
//     let flattened: Vec<f64> = self
//         .fc1_weight
//         .iter()
//         .flat_map(|x| x.iter()) // Убираем вложенность
//         .cloned() // Клонируем данные
//         .collect(); // Собираем в одномерный вектор
//     Array2::from_shape_vec((120, 16 * 5 * 5), flattened)
//         .map_err(|e| format!("Error creating fc1_weight array: {}", e))
// }

// fn fc2_weight_as_array(&self) -> Result<Array2<f64>, String> {
//     let flattened: Vec<f64> = self
//         .fc2_weight
//         .iter()
//         .flat_map(|x| x.iter()) // Убираем вложенность
//         .cloned() // Клонируем данные
//         .collect(); // Собираем в одномерный вектор
//     Array2::from_shape_vec((84, 120), flattened)
//         .map_err(|e| format!("Error creating fc2_weight array: {}", e))
// }

// fn fc3_weight_as_array(&self) -> Result<Array2<f64>, String> {
//     let flattened: Vec<f64> = self
//         .fc3_weight
//         .iter()
//         .flat_map(|x| x.iter()) // Убираем вложенность
//         .cloned() // Клонируем данные
//         .collect(); // Собираем в одномерный вектор
//     Array2::from_shape_vec((10, 84), flattened)
//         .map_err(|e| format!("Error creating fc3_weight array: {}", e))
// }

fn conv1_weight_as_array(
    conv1_weight: Vec<Vec<Vec<Vec<Decimal>>>>,
) -> Result<Array4<Decimal>, String> {
    let flattened = flatten_4d_to_1d(&conv1_weight);
    let (out_channels, in_channels, height, width) = (
        conv1_weight.len(),          // out_channels
        conv1_weight[0].len(),       // in_channels
        conv1_weight[0][0].len(),    // height
        conv1_weight[0][0][0].len(), // width
    );
    Array4::from_shape_vec((out_channels, in_channels, height, width), flattened)
        .map_err(|e| format!("Error creating conv1_weight array: {}", e))
}

fn flatten_4d_to_1d(input: &Vec<Vec<Vec<Vec<Decimal>>>>) -> Vec<Decimal> {
    input
        .iter()
        .flat_map(|x| x.iter())
        .flat_map(|y| y.iter())
        .flat_map(|z| z.iter())
        .cloned()
        .collect()
}

fn conv2d_single(
    input: &Array3<Decimal>,
    weights: &Array4<Decimal>,
    bias: &[Decimal],
) -> Array3<Decimal> {
    let (in_channels, height, width) = (input.shape()[0], input.shape()[1], input.shape()[2]);

    let out_channels = weights.shape()[0];
    let kernel_size = weights.shape()[2]; // Kernel size: height/width
    let mut output = Array::zeros((
        out_channels,
        height - kernel_size + 1,
        width - kernel_size + 1,
    ));

    for o in 0..out_channels {
        for i in 0..in_channels {
            for y in 0..(height - kernel_size + 1) {
                for x in 0..(width - kernel_size + 1) {
                    let slice = input.slice(s![i, y..y + kernel_size, x..x + kernel_size]);
                    // Корректная индексация для 4D массива
                    let kernel = &weights.slice(s![o, i, .., ..]); // Теперь это срез 4D массива

                    // Проверка формы среза и ядра
                    assert_eq!(
                        slice.shape(),
                        kernel.shape(),
                        "Shape mismatch between slice and kernel"
                    );

                    // Перемножаем срез и ядро, затем суммируем результат
                    output[[o, y, x]] += (slice.into_owned() * kernel.into_owned()).sum();
                }
            }
        }

        // Добавляем смещение (bias)
        output
            .index_axis_mut(Axis(0), o)
            .mapv_inplace(|v| v + bias[o]);
    }

    output
}

fn conv2d_partial(
    input: &Array3<Decimal>,
    weights: &Array4<Decimal>,
    bias: &[Decimal],
    channel_start: usize,
    channel_end: usize,
) -> Array3<Decimal> {
    let (in_channels, height, width) = (input.shape()[0], input.shape()[1], input.shape()[2]);
    let kernel_size = weights.shape()[2]; // Kernel size: height/width

    // Создаем выходной массив для выбранных каналов
    let mut output = Array::zeros((
        channel_end - channel_start,
        height - kernel_size + 1,
        width - kernel_size + 1,
    ));

    for o in channel_start..channel_end {
        for i in 0..in_channels {
            for y in 0..(height - kernel_size + 1) {
                for x in 0..(width - kernel_size + 1) {
                    let slice = input.slice(s![i, y..y + kernel_size, x..x + kernel_size]);
                    let kernel = weights.slice(s![o, i, .., ..]);

                    // Проверяем совпадение формы среза и ядра
                    assert_eq!(
                        slice.shape(),
                        kernel.shape(),
                        "Shape mismatch between slice and kernel"
                    );

                    // Выполняем свертку для текущего положения
                    output[[o - channel_start, y, x]] +=
                        (slice.into_owned() * kernel.into_owned()).sum();
                }
            }
        }

        // Добавляем смещение (bias)
        output
            .index_axis_mut(Axis(0), o - channel_start)
            .mapv_inplace(|v| v + bias[o]);
    }

    output
}

fn max_pool2d_single(input: &Array3<Decimal>, pool_size: usize) -> Array3<Decimal> {
    let (channels, height, width) = (input.shape()[0], input.shape()[1], input.shape()[2]);
    let mut output = Array::zeros((channels, height / pool_size, width / pool_size));

    for c in 0..channels {
        for y in 0..(height / pool_size) {
            for x in 0..(width / pool_size) {
                let slice = input.slice(s![
                    c,
                    y * pool_size..(y + 1) * pool_size,
                    x * pool_size..(x + 1) * pool_size
                ]);
                output[[c, y, x]] = slice.iter().cloned().fold(Decimal::MIN, Decimal::max);
            }
        }
    }

    output
}
pub struct DigitRecognitionProgram(());

#[sails_rs::program]
impl DigitRecognitionProgram {
    // Program's constructor
    pub fn new() -> Self {
        DigitRecognitionService::init();
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
