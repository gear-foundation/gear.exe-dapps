#![no_std]
#![allow(static_mut_refs)]

use ndarray::{Array1, Array2, Array3, ShapeBuilder};
use rust_decimal::Decimal;
use sails_rs::{
    gstd::{exec, msg},
    prelude::*,
};
struct CnnCatsDogsService(());

pub mod model;
use model::*;
use rust_decimal::prelude::ToPrimitive;

static mut STATE: Option<State> = None;

#[derive(Default)]
pub struct State {
    model: Model,
    config: Config,
    current_layer_id: u8,
    x: Array3<i128>,
    result: Array2<i128>,
    result_1_d: Array1<i128>,
    im2col_matrix: Array2<i128>,
    probability: (Decimal, bool),
}

#[derive(Default)]
pub struct Config {
    conv_1_batch_size: u16,
    conv_2_batch_size: u16,
    conv_3_batch_size: u16,
    conv_4_batch_size: u16,
    bias_1_batch_size: u16,
    bias_2_batch_size: u16,
    bias_3_batch_size: u16,
    bias_4_batch_size: u16,
    norm_1_batch_size: u16,
    norm_2_batch_size: u16,
    norm_3_batch_size: u16,
    norm_4_batch_size: u16,
}
#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i128,
    pub scale: u32,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct CalcResult {
    pub probability: FixedPoint,
    pub calculated: bool,
}

impl FixedPoint {
    fn from_decimal(decimal: Decimal) -> Self {
        let scale = decimal.scale();
        let num = decimal.mantissa() as i128;
        FixedPoint { num, scale }
    }
}

impl CnnCatsDogsService {
    fn init() -> Self {
        unsafe {
            STATE = Some(State {
                model: Model::init(),
                ..Default::default()
            })
        }
        Self(())
    }
    fn get_mut(&mut self) -> &'static mut State {
        unsafe { STATE.as_mut().expect("STATE is not initialized") }
    }
    fn get(&self) -> &'static State {
        unsafe { STATE.as_ref().expect("STATE is not initialized") }
    }
}

#[sails_rs::service]
impl CnnCatsDogsService {
    pub fn new() -> Self {
        Self(())
    }

    pub fn set_layer_filters(
        &mut self,
        layer: u8,
        filters: Vec<Vec<i64>>,
        row_start: u16,
    ) {
        self.get_mut()
            .model
            .set_layer_filters(layer, filters, row_start as usize);
    }

    pub fn set_layer_bias(
        &mut self,
        layer: u8,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
    ) {
        self.get_mut()
            .model
            .set_layer_bias(layer, bias, gamma, beta, mean, variance);
    }

    pub fn set_dense_1_weight_const(&mut self, filters: Vec<Vec<i32>>, row_start: u16) {
        self.get_mut()
            .model
            .set_dense_1_weight_const(filters, row_start);
    }

    pub fn set_dense_1_bias_const(
        &mut self,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
    ) {
        self.get_mut()
            .model
            .set_dense_bias_const(bias, gamma, beta, mean, variance);
    }

    pub fn set_dense_2_const(&mut self, filters: Vec<Vec<i64>>, bias: Vec<i64>) {
        self.get_mut().model.set_dense_2_const(filters, bias);
    }

    pub fn predict(&mut self, pixels: Vec<u8>, continue_execution: bool) {
        let state = self.get_mut();
        state.x = process_pixels_to_array3(pixels);
        state.probability = (Decimal::new(0, 0), false);
        state.current_layer_id = 1;

        if continue_execution {
            let bytes = [
                "CnnCatsDogs".encode(),
                "AllocateIm2Col".encode(),
                true.encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn allocate_im2col(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        let (h, w, c) = state.x.dim();
        let output_height = h - 2;
        let output_width = w - 2;
        let layer = state.current_layer_id;

        state.im2col_matrix = Array2::<i128>::zeros((3 * 3 * c, output_height * output_width));

        let (kh, kw, _, oc) = match layer {
            1 => state.model.conv1_dim,
            2 => state.model.conv2_dim,
            3 => state.model.conv3_dim,
            4 => state.model.conv4_dim,
            _ => panic!("Unknown layer"),
        };

        let h_out = (h - kh) + 1;
        let w_out = (w - kw) + 1;

        state.result = Array2::<i128>::zeros((oc, h_out * w_out));
        if continue_execution {
            let bytes = ["CnnCatsDogs".encode(), "Im2Col".encode(), true.encode()].concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn im2col(&mut self, continue_execution: bool) {
        let state = self.get_mut();

        im2col(&state.x, &mut state.im2col_matrix, 3, 3, 1);
        if continue_execution {
            let layer = state.current_layer_id;
            let batch_size = match layer {
                1 => state.config.conv_1_batch_size,
                2 => state.config.conv_2_batch_size,
                3 => state.config.conv_3_batch_size,
                4 => state.config.conv_4_batch_size,
                _ => panic!("Unknown layer"),
            };
            let start_col: u16 = 0;
            let bytes = [
                "CnnCatsDogs".encode(),
                "Conv1".encode(),
                (start_col, batch_size, true).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn conv(&mut self, start_col: u16, batch_size: u16, continue_execution: bool) {
        let state = self.get_mut();
        let layer = state.current_layer_id;
        let (end_col, finished) = state.model.conv(
            layer,
            &state.im2col_matrix,
            start_col,
            batch_size,
            &mut state.result,
        );

        if continue_execution {
            if finished {
                let start_filter_idx: u16 = 0;
                let batch_size = match layer {
                    1 => state.config.bias_1_batch_size,
                    2 => state.config.bias_2_batch_size,
                    3 => state.config.bias_3_batch_size,
                    4 => state.config.bias_4_batch_size,
                    _ => panic!("Unknown layer"),
                };
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "AddBiasAndRelu".encode(),
                    (start_filter_idx, batch_size, true).encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            } else {
                let start_col = end_col as u16;
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "Conv1".encode(),
                    (start_col, batch_size, true).encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            }
        }
    }

    pub fn add_bias_and_relu(
        &mut self,
        start_filter_idx: u16,
        batch_size: u16,
        continue_execution: bool,
    ) {
        let state = self.get_mut();
        let layer = state.current_layer_id;
        let (end_filter_idx, finished) =
            state
                .model
                .bias_and_relu(layer, start_filter_idx, batch_size, &mut state.result);

        if continue_execution {
            let layer = state.current_layer_id;
            if finished {
                let start_channel_id: u16 = 0;
                let batch_size = match layer {
                    1 => state.config.norm_1_batch_size,
                    2 => state.config.norm_2_batch_size,
                    3 => state.config.norm_3_batch_size,
                    4 => state.config.norm_4_batch_size,
                    _ => panic!("Unknown layer"),
                };
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "Norm".encode(),
                    (start_channel_id, batch_size, true).encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            } else {
                let start_filter_idx = end_filter_idx as u16;
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "AddBiasAndRelu".encode(),
                    (start_filter_idx, batch_size, true).encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            }
        }
    }

    pub fn norm(&mut self, start_channel_id: u16, batch_size: u16, continue_execution: bool) {
        let state = self.get_mut();
        let layer = state.current_layer_id;
        let (end_channel_id, finished) =
            state
                .model
                .batch_norm(layer, &mut state.result, start_channel_id, batch_size);

        if continue_execution {
            if finished {
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "Convert2DTo3D".encode(),
                    true.encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            } else {
                let start_channel_id = end_channel_id as u16;
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "Norm".encode(),
                    (start_channel_id, batch_size, true).encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            }
        }
    }

    pub fn convert_2d_to_3d(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        let layer = state.current_layer_id;
        let (h, w, c) = match layer {
            1 => state.model.layer_1.output_shape,
            2 => state.model.layer_2.output_shape,
            3 => state.model.layer_3.output_shape,
            4 => state.model.layer_4.output_shape,
            _ => panic!("Unknown layer"),
        };
        state.x = convert_2d_to_3d(&state.result, h, w, c);
        if continue_execution {
            let bytes = ["CnnCatsDogs".encode(), "MaxPool2D".encode(), true.encode()].concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }
    pub fn max_pool_2d(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        state.current_layer_id += 1;
        Model::max_pool_2_d(&mut state.x);
        if continue_execution {
            let if_conv_layer = match state.current_layer_id {
                2 | 3 | 4 => true,
                5 => false,
                _ => panic!("Unexpected layer"),
            };

            if if_conv_layer {
                let bytes = [
                    "CnnCatsDogs".encode(),
                    "AllocateIm2Col".encode(),
                    true.encode(),
                ]
                .concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            } else {
                let bytes = ["CnnCatsDogs".encode(), "Flatten".encode(), true.encode()].concat();
                msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
            }
        }
    }

    pub fn flatten(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        state.result = Model::flatten_apply(&state.x);
        if continue_execution {
            let bytes = ["CnnCatsDogs".encode(), "DenseApply".encode(), true.encode()].concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn dense_apply(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        let layer = state.current_layer_id;
        match layer {
            5 => {
                state.result_1_d = state.model.dense_1_apply(&state.result);
                if continue_execution {
                    let bytes =
                        ["CnnCatsDogs".encode(), "DenseApply".encode(), true.encode()].concat();
                    msg::send_bytes(exec::program_id(), bytes, 0)
                        .expect("Error during msg sending");
                }
                state.current_layer_id += 1;
            }
            6 => {
                state.probability = (state.model.dense_2_apply(&state.result_1_d), true);
                state.current_layer_id = 0;
            }
            _ => panic!("Unexpected layer"),
        };
    }

    // pub fn get_result(&self, start_row: u16, num_rows: u16) -> Vec<Vec<i128>> {
    //     let state = self.get();
    //     let end_row = (start_row + num_rows).min(state.result.nrows() as u16) as usize;
    //     state
    //         .result
    //         .slice(ndarray::s![start_row as usize..end_row, ..])
    //         .axis_iter(ndarray::Axis(0))
    //         .map(|row| row.to_vec())
    //         .collect()
    // }

    // pub fn get_input(&self) -> Vec<Vec<Vec<i128>>> {
    //     let state = self.get();
    //     state
    //         .x
    //         .axis_iter(ndarray::Axis(0))
    //         .map(|matrix| {
    //             matrix
    //                 .outer_iter()
    //                 .map(|row| row.to_vec())
    //                 .collect::<Vec<Vec<i128>>>()
    //         })
    //         .collect::<Vec<Vec<Vec<i128>>>>()
    // }

    pub fn get_result_1_d(&self) -> Vec<i128> {
        let state = self.get();
        state.result_1_d.to_vec()
    }

    pub fn get_probability(&self) -> CalcResult {
        let state = self.get();
        CalcResult {
            probability: FixedPoint::from_decimal(state.probability.0),
            calculated: state.probability.1,
        }
    }
}

fn process_pixels_to_array3(pixels: Vec<u8>) -> Array3<i128> {
    let depth = 128;
    let height = 128;
    let width = 3;

    let array = Array3::from_shape_vec((depth, height, width), pixels)
        .expect("The size of the vector does not match the array dimensions.");

    array.mapv(|value| {
        (Decimal::from(value) / Decimal::from(255u8))
            // .round_dp(16)
            .checked_mul(Decimal::from(SCALE))
            .expect("Error in decimal multiplication")
            .round()
            .to_i128()
            .unwrap()
    })
}

fn convert_2d_to_3d(
    input: &Array2<i128>,
    height: usize,
    width: usize,
    channels: usize,
) -> Array3<i128> {
    assert_eq!(
        input.nrows(),
        channels,
        "Number of rows in 2D matrix must match the number of channels"
    );
    assert_eq!(
        input.ncols(),
        height * width,
        "Number of columns must match height * width"
    );

    let mut output = Array3::<i128>::zeros((height, width, channels).f());

    for (channel_idx, row) in input.axis_iter(ndarray::Axis(0)).enumerate() {
        let reshaped = row
            .to_shape((height, width)) // Преобразуем строку в (126, 126)
            .expect("Failed to reshape row into (height, width)");
        output
            .index_axis_mut(ndarray::Axis(2), channel_idx)
            .assign(&reshaped);
    }

    output
}

fn im2col(
    input: &Array3<i128>,
    im2col_matrix: &mut Array2<i128>,
    kernel_height: usize,
    kernel_width: usize,
    stride: usize,
) {
    let (h, w, c) = input.dim(); // Размеры входного массива

    let mut col_index = 0;
    for i in (0..=(h - kernel_height)).step_by(stride) {
        for j in (0..=(w - kernel_width)).step_by(stride) {
            let mut row_index = 0;
            for ki in 0..kernel_height {
                for kj in 0..kernel_width {
                    for ch in 0..c {
                        im2col_matrix[[row_index, col_index]] = input[(i + ki, j + kj, ch)];
                        row_index += 1;
                    }
                }
            }
            col_index += 1;
        }
    }
}
pub struct CnnCatsDogsProgram(());

#[sails_rs::program]
impl CnnCatsDogsProgram {
    pub fn new() -> Self {
        CnnCatsDogsService::init();
        Self(())
    }

    pub fn cnn_cats_dogs(&self) -> CnnCatsDogsService {
        CnnCatsDogsService::new()
    }
}
