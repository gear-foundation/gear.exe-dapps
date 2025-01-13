use crate::FixedPoint;
use ndarray::{s, Array, Array1, Array2, Array3, Array4, ArrayBase, Data, Dimension};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sails_rs::prelude::*;

pub fn fixed_points_to_decimal_vector(weights: &[FixedPoint]) -> Vec<Decimal> {
    weights
        .iter()
        .map(|fp| Decimal::new(fp.num as i64, fp.scale))
        .collect()
}

pub fn filter_to_matrix_from_flat(
    weights: &[Decimal],
    num_filters: usize,
    channels: usize,
    filter_height: usize,
    filter_width: usize,
) -> Array2<Decimal> {
    let mut filter_matrix =
        Array2::<Decimal>::zeros((num_filters, channels * filter_height * filter_width));

    for n in 0..num_filters {
        let mut row_vector = filter_matrix.row_mut(n);
        let start_idx = n * channels * filter_height * filter_width;
        let end_idx = start_idx + channels * filter_height * filter_width;
        row_vector.assign(
            &Array2::from_shape_vec(
                (1, channels * filter_height * filter_width),
                weights[start_idx..end_idx].to_vec(),
            )
            .unwrap()
            .row(0),
        );
    }

    filter_matrix
}

pub fn im2col(input: &Array3<Decimal>, filter_size: usize) -> Array2<Decimal> {
    let (channels, height, width) = (input.shape()[0], input.shape()[1], input.shape()[2]);

    let output_height = height - filter_size + 1;
    let output_width = width - filter_size + 1;

    let mut col_matrix = Array2::<Decimal>::zeros((
        channels * filter_size * filter_size,
        output_height * output_width,
    ));

    let mut col_idx = 0;
    for y in 0..=height - filter_size {
        for x in 0..=width - filter_size {
            let mut col_vector = col_matrix.column_mut(col_idx);
            let mut vec_idx = 0;
            for c in 0..channels {
                for dy in 0..filter_size {
                    for dx in 0..filter_size {
                        col_vector[vec_idx] = input[(c, y + dy, x + dx)];
                        vec_idx += 1;
                    }
                }
            }
            col_idx += 1;
        }
    }

    col_matrix
}

// Load weights into Array4
pub fn load_weights_4d<const O: usize, const I: usize, const H: usize, const W: usize>(
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

/// Converts a 2D array of weights into an `Array2<Decimal>`
pub fn load_weights_2d<const ROWS: usize, const COLS: usize>(
    weights: [[Decimal; COLS]; ROWS],
) -> Array2<Decimal> {
    Array2::from_shape_vec(
        (ROWS, COLS),
        weights.iter().flat_map(|row| row.iter().cloned()).collect(),
    )
    .expect("Failed to convert 2D weights")
}

pub fn apply_conv_layer(
    input: &Array2<Decimal>,
    weights: &Array2<Decimal>,
    bias: &Array1<Decimal>,
    output_size: usize,
    pool_stride: usize,
) -> Array3<Decimal> {
    let conv_result = relu(&conv2d(input, weights, bias, output_size));
    max_pool2d_single(&conv_result, pool_stride)
}
pub fn conv2d(
    input_col: &Array2<Decimal>,
    weights: &Array2<Decimal>,
    bias: &Array1<Decimal>,
    output_size: usize,
) -> Array3<Decimal> {
    let mut output = weights.dot(input_col);

    for (mut feature_map, &b) in output.outer_iter_mut().zip(bias.iter()) {
        feature_map.mapv_inplace(|x| x + b);
    }

    output.into_shape_with_order((8, output_size, output_size)).unwrap()
}

// Relu activation
pub fn relu<T, D>(input: &ArrayBase<T, D>) -> Array<Decimal, D>
where
    T: Data<Elem = Decimal>,
    D: Dimension,
{
    input.mapv(|x| if x > Decimal::ZERO { x } else { Decimal::ZERO })
}

pub fn relu_1d(input: &Array1<Decimal>) -> Array1<Decimal> {
    input.mapv(|x| if x > Decimal::ZERO { x } else { Decimal::ZERO })
}

// Softmax function
pub fn softmax(values: &[Decimal]) -> Vec<Decimal> {
    let max_val = values.iter().cloned().max().unwrap_or(Decimal::ZERO);
    let exp_values: Vec<Decimal> = values
        .iter()
        .map(|x| exp_approx(*x - max_val, dec!(1e-6)))
        .collect();
    let sum: Decimal = exp_values.iter().sum();
    exp_values.iter().map(|x| *x / sum).collect()
}

// Helper functions for approximations
pub fn exp_approx(x: Decimal, tolerance: Decimal) -> Decimal {
    if x < Decimal::new(-15, 0) {
        return Decimal::ZERO;
    }

    let mut term = Decimal::ONE;
    let mut sum = Decimal::ONE;

    let mut n = 1;
    while term.abs() >= tolerance {
        term *= x / Decimal::from(n);
        sum += term;
        n += 1;
    }
    sum
}

pub fn flatten_single(input: &Array3<Decimal>) -> Array1<Decimal> {
    Array::from_shape_vec(input.len(), input.iter().cloned().collect()).unwrap()
}

// the linear transformation of an input vector
pub fn linear_single(
    input: &Array1<Decimal>,
    weights: &Array2<Decimal>,
    bias: &Array1<Decimal>,
) -> Array1<Decimal> {
    let mut output = weights.dot(input);
    output += bias;
    output
}

pub fn max_pool2d_single(input: &Array3<Decimal>, pool_size: usize) -> Array3<Decimal> {
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

pub fn flatten_4d_to_1d(input: &Vec<Vec<Vec<Vec<Decimal>>>>) -> Vec<Decimal> {
    input
        .iter()
        .flat_map(|x| x.iter())
        .flat_map(|y| y.iter())
        .flat_map(|z| z.iter())
        .cloned()
        .collect()
}

pub fn fixed_points_to_array4(
    fixed_points: Vec<FixedPoint>,
    dimensions: (usize, usize, usize, usize),
) -> Array4<Decimal> {
    let (dim1, dim2, dim3, dim4) = dimensions;

    assert_eq!(
        fixed_points.len(),
        dim1 * dim2 * dim3 * dim4,
        "Input Vec<FixedPoint> does not match target dimensions"
    );

    let mut result = Array4::<Decimal>::zeros((dim1, dim2, dim3, dim4));

    for (index, fixed_point) in fixed_points.into_iter().enumerate() {
        let d1 = index / (dim2 * dim3 * dim4);
        let d2 = (index / (dim3 * dim4)) % dim2;
        let d3 = (index / dim4) % dim3;
        let d4 = index % dim4;

        result[[d1, d2, d3, d4]] = Decimal::new(fixed_point.num as i64, fixed_point.scale);
    }

    result
}

pub fn fixed_points_to_array2(
    fixed_points: Vec<FixedPoint>,
    dimensions: (usize, usize),
) -> Array2<Decimal> {
    let (rows, cols) = dimensions;

    assert_eq!(
        fixed_points.len(),
        rows * cols,
        "Input Vec<FixedPoint> does not match target dimensions"
    );

    let mut result = Array2::<Decimal>::zeros((rows, cols));

    for (index, fixed_point) in fixed_points.into_iter().enumerate() {
        let row = index / cols;
        let col = index % cols;

        result[[row, col]] = Decimal::new(fixed_point.num as i64, fixed_point.scale);
    }

    result
}
