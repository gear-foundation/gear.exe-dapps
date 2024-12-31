use ndarray::{s, Array1, Array2, Array3};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, MathematicalOps};
use sails_rs::prelude::*;

// 2^32
pub const SCALE: i128 = 4294967296;
pub const SHIFT: u8 = 32;

const VARIANCE_EPS: i128 = 430;

#[derive(Debug, Default)]
pub struct Model {
    pub layer_1: Layer,
    pub layer_2: Layer,
    pub layer_3: Layer,
    pub layer_4: Layer,
    pub dense_layer_1: DenseLayer,
    pub dense_layer_2: DenseLayer,
    pub conv1_dim: (usize, usize, usize, usize),
    pub conv2_dim: (usize, usize, usize, usize),
    pub conv3_dim: (usize, usize, usize, usize),
    pub conv4_dim: (usize, usize, usize, usize),
}

#[derive(Debug, Default)]
pub struct Layer {
    pub filters: Array2<i128>,
    bias: Array1<i128>,
    mean: Array1<i128>,
    variance: Array1<i128>,
    gamma: Array1<i128>,
    beta: Array1<i128>,
    pub output_shape: (usize, usize, usize),
}

#[derive(Debug, Default)]
pub struct DenseLayer {
    pub filters: Array2<i128>,
    bias: Array1<i128>,
    mean: Array1<i128>,
    variance: Array1<i128>,
    gamma: Array1<i128>,
    beta: Array1<i128>,
}

impl DenseLayer {
    pub fn new(filters: Vec<Vec<i64>>, bias: Vec<i64>) -> Self {
        let filters: Array2<i128> = Array2::from_shape_vec(
            (filters.len(), filters[0].len()),
            filters
                .into_iter()
                .flatten()
                .map(|x| x as i128) // преобразуем i32 в i128
                .collect(),
        )
        .expect("Invalid filter dimensions");

        let bias = Array1::from(bias.into_iter().map(|x| x as i128).collect::<Vec<i128>>());

        Self {
            filters,
            bias,
            ..Default::default()
        }
    }

    pub fn add_filters_part(&mut self, part: Vec<Vec<i32>>, row_start: usize) {
        let rows = part.len();
        let cols = if rows > 0 { part[0].len() } else { 0 };

        assert_eq!(cols, self.filters.ncols(), "Column size mismatch");
        assert!(
            row_start + rows <= self.filters.nrows(),
            "Row range out of bounds"
        );

        let flat_part: Vec<i128> = part.into_iter().flatten().map(|x| x as i128).collect();
        let part_array =
            Array2::from_shape_vec((rows, cols), flat_part).expect("Invalid part dimensions");

        self.filters
            .slice_mut(s![row_start..row_start + rows, ..])
            .assign(&part_array);
    }

    pub fn add_bias(
        &mut self,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
    ) {
        self.bias = Array1::from(bias.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.gamma = Array1::from(gamma.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.beta = Array1::from(beta.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.mean = Array1::from(mean.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.variance = Array1::from(
            variance
                .into_iter()
                .map(|x| x as i128)
                .collect::<Vec<i128>>(),
        );
    }
}
impl Layer {
    pub fn add_filters_part(&mut self, part: Vec<Vec<i64>>, row_start: usize) {
        let rows = part.len();
        let cols = if rows > 0 { part[0].len() } else { 0 };

        assert_eq!(cols, self.filters.ncols(), "Column size mismatch");
        assert!(
            row_start + rows <= self.filters.nrows(),
            "Row range out of bounds"
        );

        let flat_part: Vec<i128> = part.into_iter().flatten().map(|x| x as i128).collect();
        let part_array =
            Array2::from_shape_vec((rows, cols), flat_part).expect("Invalid part dimensions");

        self.filters
            .slice_mut(s![row_start..row_start + rows, ..])
            .assign(&part_array);
    }

    pub fn add_bias(
        &mut self,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
    ) {
        self.bias = Array1::from(bias.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.gamma = Array1::from(gamma.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.beta = Array1::from(beta.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.mean = Array1::from(mean.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        self.variance = Array1::from(
            variance
                .into_iter()
                .map(|x| x as i128)
                .collect::<Vec<i128>>(),
        );
    }

    pub fn new(
        filters: Vec<Vec<i64>>,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
        output_shape: (usize, usize, usize),
    ) -> Self {
        let filters: Array2<i128> = Array2::from_shape_vec(
            (filters.len(), filters[0].len()),
            filters
                .into_iter()
                .flatten()
                .map(|x| x as i128) // преобразуем i32 в i128
                .collect(),
        )
        .expect("Invalid filter dimensions");

        let bias = Array1::from(bias.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        let gamma = Array1::from(gamma.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        let beta = Array1::from(beta.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        let mean = Array1::from(mean.into_iter().map(|x| x as i128).collect::<Vec<i128>>());
        let variance = Array1::from(
            variance
                .into_iter()
                .map(|x| x as i128)
                .collect::<Vec<i128>>(),
        );

        Self {
            filters,
            bias,
            mean,
            variance,
            gamma,
            beta,
            output_shape,
        }
    }
}
impl Model {
    pub fn init() -> Self {
        Self {
            conv1_dim: (3, 3, 3, 32),
            conv2_dim: (3, 3, 32, 64),
            conv3_dim: (3, 3, 64, 96),
            conv4_dim: (3, 3, 96, 192),
            layer_1: Layer {
                filters: Array2::<i128>::zeros((32, 27)),
                bias: Array1::<i128>::zeros(32),
                mean: Array1::<i128>::zeros(32),
                variance: Array1::<i128>::zeros(32),
                gamma: Array1::<i128>::zeros(32),
                beta: Array1::<i128>::zeros(32),
                output_shape: (126, 126, 32),
            },
            layer_2: Layer {
                filters: Array2::<i128>::zeros((64, 288)),
                bias: Array1::<i128>::zeros(64),
                mean: Array1::<i128>::zeros(64),
                variance: Array1::<i128>::zeros(64),
                gamma: Array1::<i128>::zeros(64),
                beta: Array1::<i128>::zeros(64),
                output_shape: (61, 61, 64),
            },
            layer_3: Layer {
                filters: Array2::<i128>::zeros((96, 576)),
                bias: Array1::<i128>::zeros(96),
                mean: Array1::<i128>::zeros(96),
                variance: Array1::<i128>::zeros(96),
                gamma: Array1::<i128>::zeros(96),
                beta: Array1::<i128>::zeros(96),
                output_shape: (28, 28, 96),
            },
            layer_4: Layer {
                filters: Array2::<i128>::zeros((192, 864)),
                bias: Array1::<i128>::zeros(192),
                mean: Array1::<i128>::zeros(192),
                variance: Array1::<i128>::zeros(192),
                gamma: Array1::<i128>::zeros(192),
                beta: Array1::<i128>::zeros(192),
                output_shape: (12, 12, 192),
            },
            dense_layer_1: DenseLayer {
                filters: Array2::<i128>::zeros((6912, 256)),
                bias: Array1::<i128>::zeros(256),
                gamma: Array1::<i128>::zeros(256),
                beta: Array1::<i128>::zeros(256),
                mean: Array1::<i128>::zeros(256),
                variance: Array1::<i128>::zeros(256),
            },
            ..Default::default()
        }
    }

    pub fn set_layer_filters(&mut self, layer: u8, filters: Vec<Vec<i64>>, row_start: usize) {
        match layer {
            1 => {
                self.layer_1.add_filters_part(filters, row_start);
            }
            2 => {
                self.layer_2.add_filters_part(filters, row_start);
            }
            3 => {
                self.layer_3.add_filters_part(filters, row_start);
            }
            4 => {
                self.layer_4.add_filters_part(filters, row_start);
            }
            _ => panic!("Unknown layer"),
        }
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
        match layer {
            1 => {
                self.layer_1.add_bias(bias, gamma, beta, mean, variance);
            }
            2 => {
                self.layer_2.add_bias(bias, gamma, beta, mean, variance);
            }
            3 => {
                self.layer_3.add_bias(bias, gamma, beta, mean, variance);
            }
            4 => {
                self.layer_4.add_bias(bias, gamma, beta, mean, variance);
            }
            _ => panic!("Unknown layer"),
        }
    }

    pub fn set_dense_1_weight_const(&mut self, filters: Vec<Vec<i32>>, row_start: u16) {
        self.dense_layer_1
            .add_filters_part(filters, row_start as usize);
    }

    pub fn set_dense_bias_const(
        &mut self,
        bias: Vec<i64>,
        gamma: Vec<i64>,
        beta: Vec<i64>,
        mean: Vec<i64>,
        variance: Vec<i64>,
    ) {
        self.dense_layer_1
            .add_bias(bias, gamma, beta, mean, variance);
    }

    pub fn set_dense_2_const(&mut self, filters: Vec<Vec<i64>>, bias: Vec<i64>) {
        self.dense_layer_2 = DenseLayer::new(filters, bias);
    }

    pub fn conv(
        &self,
        layer: u8,
        im2col_matrix: &Array2<i128>,
        start_col: u16,
        batch_size: u16,
        result: &mut Array2<i128>,
    ) -> (usize, bool) {
        let cols = im2col_matrix.shape()[1];
        let end_col = (start_col + batch_size).min(cols as u16) as usize;

        if start_col >= cols as u16 {
            return (end_col, true);
        }

        let filters = match layer {
            1 => &self.layer_1.filters,
            2 => &self.layer_2.filters,
            3 => &self.layer_3.filters,
            4 => &self.layer_4.filters,
            _ => panic!("Unknown layer"),
        };

        let im2col_matrix_batch = im2col_matrix
            .slice(s![.., start_col as usize..end_col])
            .to_owned();

        let batch_result = multiply_fixed_point_matrices(filters, &im2col_matrix_batch);
        result
            .slice_mut(s![.., start_col as usize..end_col])
            .assign(&batch_result);
        (end_col, end_col >= cols)
    }

    pub fn bias_and_relu(
        &self,
        layer: u8,
        start_filter_idx: u16,
        batch_size: u16,
        result: &mut Array2<i128>,
    ) -> (usize, bool) {
        let bias = match layer {
            1 => &self.layer_1.bias,
            2 => &self.layer_2.bias,
            3 => &self.layer_3.bias,
            4 => &self.layer_4.bias,
            _ => panic!("Unknown layer"),
        };

        let num_filters = bias.len();

        let spatial_size = result.ncols();
        let end_filter_idx = (start_filter_idx + batch_size).min(num_filters as u16) as usize;

        if start_filter_idx >= num_filters as u16 {
            return (end_filter_idx, true);
        }

        for filter_idx in start_filter_idx as usize..end_filter_idx {
            for spatial_idx in 0..spatial_size {
                let current_value = result[[filter_idx, spatial_idx]];
                let bias_value = bias[filter_idx];

                let updated_value = if let Some(sum) = current_value.checked_add(bias_value.into())
                {
                    sum
                } else {
                    panic!("Addition overflow");
                };
                let relu = |value: i128| if value > 0 { value } else { 0 };
                result[[filter_idx, spatial_idx]] = relu(updated_value);
            }
        }
        (end_filter_idx, end_filter_idx >= num_filters)
    }

    pub fn batch_norm(
        &self,
        layer: u8,
        result: &mut Array2<i128>,
        start_channel_id: u16,
        batch_size: u16,
    ) -> (usize, bool) {
        let num_channels = result.nrows();
        let spatial_size = result.ncols();

        let end_channel_id = (start_channel_id + batch_size).min(num_channels as u16) as usize;

        if start_channel_id >= num_channels as u16 {
            return (end_channel_id, true);
        }

        let (gamma, beta, mean, variance) = match layer {
            1 => (
                &self.layer_1.gamma,
                &self.layer_1.beta,
                &self.layer_1.mean,
                &self.layer_1.variance,
            ),
            2 => (
                &self.layer_2.gamma,
                &self.layer_2.beta,
                &self.layer_2.mean,
                &self.layer_2.variance,
            ),
            3 => (
                &self.layer_3.gamma,
                &self.layer_3.beta,
                &self.layer_3.mean,
                &self.layer_3.variance,
            ),
            4 => (
                &self.layer_4.gamma,
                &self.layer_4.beta,
                &self.layer_4.mean,
                &self.layer_4.variance,
            ),
            _ => panic!("Unknown layer"),
        };
        for channel_idx in start_channel_id as usize..end_channel_id {
            let gamma_c = gamma[channel_idx];
            let beta_c = beta[channel_idx];
            let mean_c = mean[channel_idx];
            let variance_c = variance[channel_idx];

            let adjusted_variance = variance_c + VARIANCE_EPS;

            let variance_sqrt = fixed_point_sqrt(adjusted_variance);

            for spatial_idx in 0..spatial_size {
                let value = result[[channel_idx, spatial_idx]];

                // Normalization: (value - mean) / sqrt(variance + eps)
                let diff = value.checked_sub(mean_c).expect("Subtraction overflow");
                let shifted_diff = diff.checked_shl(SHIFT as u32).expect("Shift overflow");
                let normalized = shifted_diff / variance_sqrt;

                let scaled_value = normalized.checked_mul(gamma_c).expect("Overflow") >> SHIFT;

                let final_value = scaled_value + beta_c;

                result[[channel_idx, spatial_idx]] = final_value;
            }
        }

        (end_channel_id, end_channel_id >= num_channels)
    }

    pub fn max_pool_2_d(input: &mut Array3<i128>) {
        let (h, w, c) = input.dim();
        let (ph, pw) = (2, 2);
        let out_h = h / ph;
        let out_w = w / pw;
        let mut output = Array3::<i128>::zeros((out_h, out_w, c));

        for ch in 0..c {
            for j in 0..out_h {
                for k in 0..out_w {
                    let start_h = j * ph;
                    let start_w = k * pw;
                    let end_h = start_h + ph;
                    let end_w = start_w + pw;

                    let mut max_val = i128::MIN;
                    for hh in start_h..end_h {
                        for ww in start_w..end_w {
                            max_val = max_val.max(input[[hh, ww, ch]]);
                        }
                    }

                    output[[j, k, ch]] = max_val;
                }
            }
        }
        *input = output;
    }

    pub fn dense_1_apply(&self, input: &Array2<i128>) -> Array1<i128> {
        let (weights, bias, mean, variance, gamma, beta) = (
            &self.dense_layer_1.filters,
            &self.dense_layer_1.bias,
            &self.dense_layer_1.mean,
            &self.dense_layer_1.variance,
            &self.dense_layer_1.gamma,
            &self.dense_layer_1.beta,
        );

        let result = multiply_fixed_point_matrices(&input.t().to_owned(), weights);

        let reduced_result = result
            .clone()
            .into_shape_with_order((result.shape()[1],))
            .unwrap();

        let mut result = reduced_result + bias;

        // relu
        result.mapv_inplace(|x| if x > 0 { x } else { 0 });

        for idx in 0..result.len() {
            let value = result[idx];
            let mean_c = mean[idx];
            let variance_c = variance[idx];
            let gamma_c = gamma[idx];
            let beta_c = beta[idx];

            let variance_sqrt = fixed_point_sqrt(variance_c);

            let diff = value
                .checked_sub(mean_c)
                .expect("Subtraction overflow");
            let shifted_diff = diff
                .checked_shl(SHIFT as u32)
                .expect("Shift overflow");
            let normalized = shifted_diff / variance_sqrt;

            let scaled_value = normalized
                .checked_mul(gamma_c)
                .expect("Scaling overflow")
                >> SHIFT;
            let final_value = scaled_value
                .checked_add(beta_c)
                .expect("Addition overflow");

            result[idx] = final_value;
        }
        result
    }

    pub fn dense_2_apply(&self, input: &Array1<i128>) -> Decimal {
        let (weights, bias) = (&self.dense_layer_2.filters, &self.dense_layer_2.bias);

        let input_reshaped = input
            .clone()
            .into_shape_with_order((1, input.len()))
            .expect("Failed to reshape Array1 to Array2");
        let result = multiply_fixed_point_matrices(&input_reshaped, weights);

        let reduced_result = result
            .clone()
            .into_shape_with_order((result.shape()[1],))
            .unwrap();

        let result = reduced_result + bias;

        let mut result_decimal = result.mapv(|value| Decimal::from(value) / Decimal::from(SCALE));

        // sigmoid
        result_decimal.mapv_inplace(|x| {
            let exp_neg_x = if x >= Decimal::from(15) {
                Decimal::new(0, 0)
            } else if x <= Decimal::from(-15) {
                Decimal::MAX
            } else {
                (-x).exp()
            };

            Decimal::from(1)
                / (Decimal::from(1)
                    .checked_add(exp_neg_x)
                    .unwrap_or_else(|| Decimal::MAX))
        });
        result_decimal[0]
    }

    pub fn flatten_apply(input: &Array3<i128>) -> Array2<i128> {
        let flattened = input.iter().cloned().collect::<Vec<i128>>();
        Array2::from_shape_vec((flattened.len(), 1), flattened).unwrap()
    }
}

fn multiply_fixed_point_matrices(a: &Array2<i128>, b: &Array2<i128>) -> Array2<i128> {
    assert_eq!(
        a.shape()[1],
        b.shape()[0],
        "Wrong matrix size"
    );

    let rows = a.shape()[0];
    let cols = b.shape()[1];
    let inner = a.shape()[1];

    let mut c = Array2::<i128>::zeros((rows, cols));

    let a_slice = a.as_slice().unwrap();
    let b_slice = b.as_slice().unwrap();
    let c_slice = c.as_slice_mut().unwrap();

    let total_elements = rows * cols;
    let mut idx = 0;

    while idx < total_elements {
        let i = idx / cols;
        let j = idx % cols;

        let mut sum: i128 = 0;
        let row_offset = i * inner;

        for k in 0..inner {
            sum = sum.wrapping_add(
                (a_slice[row_offset + k].wrapping_mul(b_slice[k * cols + j])) >> SHIFT,
            );
        }

        c_slice[idx] = sum;
        idx += 1;
    }

    c
}

fn fixed_point_sqrt(value: i128) -> i128 {
    let decimal_value = Decimal::from(value)
        .checked_div(Decimal::from(SCALE)) 
        .unwrap();

    let sqrt_decimal = decimal_value.sqrt().unwrap();

    let scaled_result = sqrt_decimal
        .checked_mul(Decimal::from(SCALE))
        .unwrap()
        .round();

    scaled_result.to_i128().unwrap()
}
