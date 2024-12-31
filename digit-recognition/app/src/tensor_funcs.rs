use crate::FixedPoint;
use ndarray::{s, Array, Array1, Array2, Array3, Array4, ArrayBase, Axis, Data, Dimension};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sails_rs::prelude::*;
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

// Convolution operation
pub fn conv2d(
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
                    let kernel = &weights.slice(s![o, i, .., ..]); // Теперь это срез 4D массива

                    assert_eq!(
                        slice.shape(),
                        kernel.shape(),
                        "Shape mismatch between slice and kernel"
                    );

                    output[[o, y, x]] += (slice.into_owned() * kernel.into_owned()).sum();
                }
            }
        }

        output
            .index_axis_mut(Axis(0), o)
            .mapv_inplace(|v| v + bias[o]);
    }

    output
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
