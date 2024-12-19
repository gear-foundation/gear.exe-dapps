#![no_std]

use ndarray::{linalg::general_mat_mul, s, Array1, Array2, Array3, Array4};
use rust_decimal::{prelude::Zero, Decimal};
use sails_rs::{
    gstd::{exec, msg},
    prelude::*,
};
struct CnnCatsDogsService(());
pub mod model_constants;
use model_constants::*;

static mut STATE: Option<State> = None;

#[derive(Default)]
pub struct State {
    x: Array3<Decimal>,
    filters: Array4<Decimal>,
    filters_reshaped: Array2<Decimal>,
    result: Array2<Decimal>,
    bias: Array1<Decimal>,
    im2col_matrix: Array2<Decimal>,
    im2col_matrix_done: bool,
    proccessed_col: u16,
    output: Array3<Decimal>,
    //result: Option<Vec<FixedPoint>>,
    parts: Vec<Array2<Decimal>>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i128,
    pub scale: u32,
}

impl FixedPoint {
    pub fn from_decimal(decimal: Decimal) -> Self {
        let scale = decimal.scale();
        let num = decimal.mantissa() as i128;
        FixedPoint { num, scale }
    }

    fn to_decimal(&self) -> Decimal {
        Decimal::from_i128_with_scale(self.num, self.scale)
    }
}

fn fixed_point_to_array4(data: Vec<Vec<Vec<Vec<FixedPoint>>>>) -> Array4<Decimal> {
    let depth = data.len();
    let height = data[0].len();
    let rows = data[0][0].len();
    let cols = data[0][0][0].len();

    let flattened: Vec<Decimal> = data
        .into_iter()
        .flat_map(|matrix| {
            matrix.into_iter().flat_map(|filter| {
                filter
                    .into_iter()
                    .flat_map(|row| row.into_iter().map(|fp| fp.to_decimal()))
            })
        })
        .collect();

    Array4::from_shape_vec((depth, height, rows, cols), flattened)
        .expect("Shape mismatch during conversion")
}

fn array2_to_vec_fixed_point(array: Array2<rust_decimal::Decimal>) -> Vec<Vec<FixedPoint>> {
    array
        .outer_iter() // Итерируемся по строкам (Vec<Decimal>)
        .map(|row| {
            row.iter()
                .map(|&decimal| FixedPoint::from_decimal(decimal)) // Преобразуем Decimal -> FixedPoint
                .collect::<Vec<FixedPoint>>() // Собираем строку как Vec<FixedPoint>
        })
        .collect::<Vec<Vec<FixedPoint>>>() // Собираем весь массив как Vec<Vec<FixedPoint>>
}

fn fixed_point_to_array3(data: Vec<Vec<Vec<FixedPoint>>>) -> Array3<Decimal> {
    // Extract dimensions from the nested Vec
    let depth = data.len();
    let rows = data[0].len();
    let cols = data[0][0].len();

    // Flatten the nested Vec into a single Vec<Decimal>
    let flattened: Vec<Decimal> = data
        .into_iter()
        .flat_map(|matrix| {
            matrix
                .into_iter()
                .flat_map(|row| row.into_iter().map(|fp| fp.to_decimal()))
        })
        .collect();

    // Construct the Array3<Decimal> from the flattened vector
    Array3::from_shape_vec((depth, rows, cols), flattened)
        .expect("Shape mismatch during conversion")
}

fn const_to_array4(data: [[[[Decimal; 32]; 3]; 3]; 3]) -> Array4<Decimal> {
    let depth = data.len();
    let height = data[0].len();
    let rows = data[0][0].len();
    let cols = data[0][0][0].len();

    let flattened: Vec<Decimal> = data
        .iter()
        .flat_map(|matrix| {
            matrix
                .iter()
                .flat_map(|filter| filter.iter().flat_map(|row| row.iter().cloned()))
        })
        .collect();

    Array4::from_shape_vec((depth, height, rows, cols), flattened)
        .expect("Shape mismatch during conversion")
}

fn const_to_array1(data: [Decimal; 32]) -> Array1<Decimal> {
    Array1::from_vec(data.to_vec())
}

impl CnnCatsDogsService {
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

fn vec_to_array3(pixels: Vec<Vec<Vec<u8>>>) -> Array3<Decimal> {
    let depth = pixels.len();
    let height = pixels[0].len();
    let width = pixels[0][0].len();

    // Создаем пустой Array3<Decimal>
    let mut array = Array3::<Decimal>::zeros((depth, height, width));

    for (d, layer) in pixels.iter().enumerate() {
        for (h, row) in layer.iter().enumerate() {
            for (w, &value) in row.iter().enumerate() {
                array[[d, h, w]] = Decimal::from(value) / Decimal::from(255u8);
            }
        }
    }

    array
}

#[sails_rs::service]
impl CnnCatsDogsService {
    pub fn new() -> Self {
        Self(())
    }
    pub fn predict(&mut self, pixels: Vec<u8>, continue_execution: bool) {
        let mut three_d_array = vec![vec![vec![0u8; 3]; 128]; 128];
        let mut index = 0;
        for d in 0..128 {
            for h in 0..128 {
                for w in 0..3 {
                    three_d_array[d][h][w] = pixels[index];
                    index += 1;
                }
            }
        }
        self.get_mut().x = vec_to_array3(three_d_array);
        self.get_mut().filters = const_to_array4(CONV1_WEIGHT);
        self.get_mut().bias = const_to_array1(CONV1_BIAS);
        if continue_execution {
            let bytes = ["CnnCatsDogs".encode(), "Im2Col".encode(), true.encode()].concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn im2col(&mut self, continue_execution: bool) {
        let state = self.get_mut();
        state.im2col_matrix = im2col(&state.x, 3, 3, 1);

        let (fh, fw, c, fc) = state.filters.dim();
        state.filters_reshaped = state
            .filters
            .view()
            .into_shape((fh * fw * c, fc))
            .expect("Failed to reshape filters")
            .to_owned();
        let num_filters = state.filters_reshaped.dim().1;
        let cols = state.im2col_matrix.dim().1;
        state.result = Array2::<Decimal>::zeros((num_filters, cols));
        state.im2col_matrix_done = true;
        if continue_execution {
            let batch_size: u16 = 200;
            let start_col: u16 = 0;
            let bytes = [
                "CnnCatsDogs".encode(),
                "ProcessSingleBatch".encode(),
                (batch_size, start_col, true).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), bytes, 0).expect("Error during msg sending");
        }
    }

    pub fn process_single_batch(
        &mut self,
        batch_size: u16,
        start_col: u16,
        continue_execution: bool,
    ) {
        let state = self.get_mut();
        let (rows, cols) = state.im2col_matrix.dim();
        
        assert_eq!(
            rows,
            state.filters_reshaped.dim().0,
            "Row mismatch for dot product!"
        );

        if start_col >= cols as u16 {
            // All batches processed
            return;
        }

        let end_col = (start_col + batch_size).min(cols as u16) as usize;

        let im2col_chunk = state
            .im2col_matrix
            .slice(s![.., start_col as usize..end_col]);
        let chunk_result = state.filters_reshaped.t().dot(&im2col_chunk);
        state
            .result
            .slice_mut(s![.., start_col as usize..end_col])
            .assign(&chunk_result);

        state.proccessed_col = start_col;
        if continue_execution {
            if end_col >= cols {
                let start_filter_idx: u16 = 0;
                let batch_size: u16 = 16;
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
                    "ProcessSingleBatch".encode(),
                    (batch_size, start_col, true).encode(),
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
        let bias_array = Array1::from_vec(CONV1_BIAS.to_vec());
        let num_filters = bias_array.len();
        let spatial_size = state.result.ncols();

        let end_filter_idx = (start_filter_idx + batch_size).min(num_filters as u16) as usize;

        for filter_idx in start_filter_idx as usize..end_filter_idx {
            for spatial_idx in 0..spatial_size {
                state.result[[filter_idx, spatial_idx]] += bias_array[filter_idx];
                state.result[[filter_idx, spatial_idx]] =
                    relu(state.result[[filter_idx, spatial_idx]]);
            }
        }
        if continue_execution {
            if end_filter_idx >= num_filters {
                state.output = state
                    .result
                    .clone()
                    .into_shape((num_filters, 126, 126))
                    .unwrap()
                    .permuted_axes([1, 2, 0]);
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

    pub fn get_output(&self) -> Vec<Vec<Vec<FixedPoint>>> {
        let state = self.get();
        array3_to_fixedpoint_vec(&state.output)
    }

    pub fn get_im2col_done(&self) -> bool {
        self.get().im2col_matrix_done
    }

    pub fn get_processed_col(&self) -> u16 {
        self.get().proccessed_col
    }
}

fn array3_to_fixedpoint_vec(array: &Array3<Decimal>) -> Vec<Vec<Vec<FixedPoint>>> {
    let (dim0, dim1, dim2) = array.dim(); // Get dimensions of the Array3
    let mut result = Vec::with_capacity(dim0);

    // Iterate over the first dimension
    for i in 0..dim0 {
        let mut sub_vec_1 = Vec::with_capacity(dim1);
        for j in 0..dim1 {
            let mut sub_vec_2 = Vec::with_capacity(dim2);
            for k in 0..dim2 {
                // Convert each Decimal value to FixedPoint
                let fixed_point = FixedPoint::from_decimal(array[(i, j, k)]);
                sub_vec_2.push(fixed_point);
            }
            sub_vec_1.push(sub_vec_2);
        }
        result.push(sub_vec_1);
    }

    result
}

fn im2col(
    input: &Array3<Decimal>, // Input: (height, width, channels)
    kernel_height: usize,
    kernel_width: usize,
    stride: usize,
) -> Array2<Decimal> {
    let (h, w, c) = input.dim(); // Input dimensions
    sails_rs::gstd::debug!("input.dim() {:?}", input.dim());
    let output_height = (h - kernel_height) / stride + 1;
    let output_width = (w - kernel_width) / stride + 1;

    // Rows: kernel_height * kernel_width * channels
    // Columns: output_height * output_width
    let mut im2col_matrix = Array2::<Decimal>::zeros((
        kernel_height * kernel_width * c,
        output_height * output_width,
    ));

    let mut col_index = 0;
    for i in (0..=(h - kernel_height)).step_by(stride) {
        for j in (0..=(w - kernel_width)).step_by(stride) {
            let mut row_index = 0;
            for ki in 0..kernel_height {
                for kj in 0..kernel_width {
                    for ch in 0..c {
                        // Flatten kernel patches into rows
                        im2col_matrix[[row_index, col_index]] = input[(i + ki, j + kj, ch)];
                        row_index += 1;
                    }
                }
            }
            col_index += 1;
        }
    }

    im2col_matrix
}

fn relu(x: Decimal) -> Decimal {
    if x > Decimal::zero() {
        x
    } else {
        Decimal::zero()
    }
}
pub struct CnnCatsDogsProgram(());

#[sails_rs::program]
impl CnnCatsDogsProgram {
    // Program's constructor
    pub fn new() -> Self {
        CnnCatsDogsService::init();
        Self(())
    }

    // Exposed service
    pub fn cnn_cats_dogs(&self) -> CnnCatsDogsService {
        CnnCatsDogsService::new()
    }
}
