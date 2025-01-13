use cnn_cats_dogs_client::{traits::*, CalcResult};
use image::io::Reader as ImageReader;
use ndarray::{s, Array3, Array4};
use sails_rs::{calls::*, gtest::System, ActorId, Encode};
use serde_json::Value;
use std::fs::File;
use std::fs::OpenOptions;

use std::io::Write;
pub mod model_constants;
use model_constants::*;
const ACTOR_ID: u64 = 42;
use cnn_cats_dogs_client::FixedPoint;
use sails_rs::calls::Action;
use sails_rs::gtest::calls::GTestRemoting;

fn flatten_3d_to_1d(pixels: Vec<Vec<Vec<u8>>>) -> Vec<u8> {
    pixels.into_iter().flatten().flatten().collect()
}

fn convert_to_vec<const ROWS: usize, const COLS: usize>(
    input: [[i64; COLS]; ROWS],
) -> Vec<Vec<i64>> {
    input.iter().map(|row| row.to_vec()).collect()
}

fn convert_to_vec_i32<const ROWS: usize, const COLS: usize>(
    input: [[i32; COLS]; ROWS],
) -> Vec<Vec<i32>> {
    input.iter().map(|row| row.to_vec()).collect()
}

fn load_and_preprocess_image(path: &str) -> Array3<u8> {
    let img = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let resized_img = img
        .resize_exact(128, 128, image::imageops::FilterType::Nearest)
        .to_rgba8();

    let mut array = Array4::<u8>::zeros((1, 128, 128, 3));

    for (x, y, pixel) in resized_img.enumerate_pixels() {
        let [r, g, b, _] = pixel.0;
        array[[0, y as usize, x as usize, 0]] = r;
        array[[0, y as usize, x as usize, 1]] = g;
        array[[0, y as usize, x as usize, 2]] = b;
    }

    array.slice(s![0, .., .., ..]).to_owned()
}

fn array3_to_fixed_point(array: Array3<u8>) -> Vec<Vec<Vec<u8>>> {
    array
        .outer_iter()
        .map(|filter| {
            filter
                .outer_iter()
                .map(|row| row.iter().copied().collect::<Vec<u8>>())
                .collect::<Vec<Vec<u8>>>()
        })
        .collect()
}

fn fixed_point_to_float(fixed_point: &FixedPoint) -> f64 {
    let scale_factor = 10_f64.powi(fixed_point.scale as i32);
    fixed_point.num as f64 / scale_factor
}

// Main prediction test
#[tokio::test]
async fn model_predict() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 10_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    let image_path = "images/10014.jpg";
    let pixels = flatten_3d_to_1d(array3_to_fixed_point(load_and_preprocess_image(image_path)));

    save_pixels_to_json("pixels.json", &pixels);

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(cnn_cats_dogs::WASM_BINARY);

    let program_factory = cnn_cats_dogs_client::CnnCatsDogsFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = cnn_cats_dogs_client::CnnCatsDogs::new(remoting.clone());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("layer_data.txt")
        .expect("Unable to open file");

    // Upload layers
    upload_layer(1, &mut service_client, program_id, &mut file).await;
    upload_layer(2, &mut service_client, program_id, &mut file).await;
    upload_layer(3, &mut service_client, program_id, &mut file).await;
    upload_layer(4, &mut service_client, program_id, &mut file).await;

    // Dense layers
    upload_dense_layer(1, &mut service_client, program_id, &mut file).await;
    upload_dense_layer(2, &mut service_client, program_id, &mut file).await;

    // START
    service_client
        .predict(pixels, false)
        .send_recv(program_id)
        .await
        .unwrap();

    // Layer 1
    process_layer(
        &mut service_client,
        program_id,
        15876,
        3000,
        vec![(0, 16), (16, 16)],
        vec![(0, 10), (10, 10), (20, 12)],
    )
    .await;

    // Layer 2
    process_layer(
        &mut service_client,
        program_id,
        3721,
        150,
        vec![(0, 70)],
        vec![(0, 70)],
    )
    .await;

    // Layer 3
    process_layer(
        &mut service_client,
        program_id,
        784,
        50,
        vec![(0, 100)],
        vec![(0, 100)],
    )
    .await;

    // Layer 4
    process_layer(
        &mut service_client,
        program_id,
        144,
        17,
        vec![(0, 200)],
        vec![(0, 200)],
    )
    .await;

    // Flatten
    service_client
        .flatten(false)
        .send_recv(program_id)
        .await
        .unwrap();

    // Dense Layers
    process_dense_layer(&mut service_client, program_id).await;
    process_dense_layer(&mut service_client, program_id).await;

    // Final Result
    let CalcResult {
        probability,
        calculated,
    } = service_client
        .get_probability()
        .recv(program_id)
        .await
        .unwrap();

    println!("Calculated {:?}", calculated);
    println!("Probability {:?}", fixed_point_to_float(&probability));
}

fn save_pixels_to_json(file_path: &str, pixels: &[u8]) {
    let json_data = Value::Array(
        pixels
            .iter()
            .map(|&value| Value::Number(value.into()))
            .collect(),
    );
    let json_string = serde_json::to_string_pretty(&json_data).expect("Serialization failed");

    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(json_string.as_bytes())
        .expect("Unable to write data");

    println!("Data saved to {}", file_path);
}

async fn process_layer(
    service_client: &mut cnn_cats_dogs_client::CnnCatsDogs<GTestRemoting>,
    program_id: ActorId,
    cols: u16,
    batch_size: usize,
    bias_steps: Vec<(u16, u16)>,
    norm_steps: Vec<(u16, u16)>,
) {
    // Allocate and perform im_2_col
    service_client
        .allocate_im_2_col(false)
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .im_2_col(false)
        .send_recv(program_id)
        .await
        .unwrap();

    // Perform convolution in batches
    for start_col in (0..cols).step_by(batch_size) {
        service_client
            .conv(start_col, batch_size as u16, false)
            .send_recv(program_id)
            .await
            .unwrap();
    }

    // Apply bias and ReLU
    for (start, size) in bias_steps {
        service_client
            .add_bias_and_relu(start, size as u16, false)
            .send_recv(program_id)
            .await
            .unwrap();
    }

    // Apply normalization
    for (start, size) in norm_steps {
        service_client
            .norm(start, size as u16, false)
            .send_recv(program_id)
            .await
            .unwrap();
    }

    // Convert 2D to 3D and apply pooling
    service_client
        .convert_2_d_to_3_d(false)
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .max_pool_2_d(false)
        .send_recv(program_id)
        .await
        .unwrap();
}

async fn process_dense_layer(
    service_client: &mut cnn_cats_dogs_client::CnnCatsDogs<GTestRemoting>,
    program_id: ActorId,
) {
    service_client
        .dense_apply(false)
        .send_recv(program_id)
        .await
        .unwrap();
}

async fn upload_layer(
    layer_number: u8,
    service_client: &mut cnn_cats_dogs_client::CnnCatsDogs<GTestRemoting>,
    program_id: ActorId,
    file: &mut File,
) {
    let filters = match layer_number {
        1 => convert_to_vec(CONV1_FILTERS),
        2 => convert_to_vec(CONV2_FILTERS),
        3 => convert_to_vec(CONV3_FILTERS),
        4 => convert_to_vec(CONV4_FILTERS),
        _ => panic!("Invalid layer number"),
    };
    let bias = match layer_number {
        1 => CONV1_BIAS.to_vec(),
        2 => CONV2_BIAS.to_vec(),
        3 => CONV3_BIAS.to_vec(),
        4 => CONV4_BIAS.to_vec(),
        _ => panic!("Invalid layer number"),
    };

    let gamma = match layer_number {
        1 => BATCH_NORM1_GAMMA.to_vec(),
        2 => BATCH_NORM2_GAMMA.to_vec(),
        3 => BATCH_NORM3_GAMMA.to_vec(),
        4 => BATCH_NORM4_GAMMA.to_vec(),
        _ => panic!("Invalid layer number"),
    };

    let beta = match layer_number {
        1 => BATCH_NORM1_BETA.to_vec(),
        2 => BATCH_NORM2_BETA.to_vec(),
        3 => BATCH_NORM3_BETA.to_vec(),
        4 => BATCH_NORM4_BETA.to_vec(),
        _ => panic!("Invalid layer number"),
    };

    let mean = match layer_number {
        1 => BATCH_NORM1_MEAN.to_vec(),
        2 => BATCH_NORM2_MEAN.to_vec(),
        3 => BATCH_NORM3_MEAN.to_vec(),
        4 => BATCH_NORM4_MEAN.to_vec(),
        _ => panic!("Invalid layer number"),
    };

    let variance = match layer_number {
        1 => BATCH_NORM1_VARIANCE.to_vec(),
        2 => BATCH_NORM2_VARIANCE.to_vec(),
        3 => BATCH_NORM3_VARIANCE.to_vec(),
        4 => BATCH_NORM4_VARIANCE.to_vec(),
        _ => panic!("Invalid layer number"),
    };

    let chunk_size = match layer_number {
        1 => 50,
        2 => 10,
        3 => 10,
        4 => 10,
        _ => panic!("Invalid layer number"),
    };
    let parts: Vec<_> = filters
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect();

    for (i, part) in parts.into_iter().enumerate() {
        let row_start = i * chunk_size;
        let bytes = hex::encode(
            [
                "CnnCatsDogs".encode(),
                "SetLayerFilters".encode(),
                (layer_number, part.clone(), (row_start as u16)).encode(),
            ]
            .concat(),
        );

        println!("LAYER {}, {:?}", layer_number, bytes.len());
        writeln!(file, "\"0x{}\",", bytes).unwrap();

           

        service_client
            .set_layer_filters(layer_number, part, row_start as u16)
            .send_recv(program_id)
            .await
            .unwrap();
    }

    let bytes = hex::encode(
        [
            "CnnCatsDogs".encode(),
            "SetLayerBias".encode(),
            (
                layer_number,
                bias.clone(),
                gamma.clone(),
                beta.clone(),
                mean.clone(),
                variance.clone(),
            )
                .encode(),
        ]
        .concat(),
    );
    writeln!(file, "\"0x{}\",", bytes).unwrap();

    service_client
        .set_layer_bias(layer_number, bias, gamma, beta, mean, variance)
        .send_recv(program_id)
        .await
        .unwrap();
}

async fn upload_dense_layer(
    layer_number: u8,
    service_client: &mut cnn_cats_dogs_client::CnnCatsDogs<GTestRemoting>,
    program_id: ActorId,
    file: &mut File,
) {
    if layer_number == 1 {
        let filters = convert_to_vec_i32(DENSE1_WEIGHT);
        let chunk_size = 1000;
        let parts: Vec<_> = filters
            .chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        for (i, part) in parts.into_iter().enumerate() {
            let row_start = i * chunk_size;
            let bytes = hex::encode(
                [
                    "CnnCatsDogs".encode(),
                    "SetDense1WeightConst".encode(),
                    (part.clone(), row_start as u16).encode(),
                ]
                .concat(),
            );
            writeln!(file, "\"0x{}\",", bytes).unwrap();
            service_client
                .set_dense_1_weight_const(part, row_start as u16)
                .send_recv(program_id)
                .await
                .unwrap();
        }

        let bias = DENSE1_BIAS.to_vec();
        let gamma = BATCH_NORM5_GAMMA.to_vec();
        let beta = BATCH_NORM5_BETA.to_vec();
        let mean = BATCH_NORM5_MEAN.to_vec();
        let variance = BATCH_NORM5_VARIANCE.to_vec();
        let bytes = hex::encode(
            [
                "CnnCatsDogs".encode(),
                "SetDense1BiasConst".encode(),
                (
                    bias.clone(),
                    gamma.clone(),
                    beta.clone(),
                    mean.clone(),
                    variance.clone(),
                )
                    .encode(),
            ]
            .concat(),
        );
        writeln!(file, "\"0x{}\",", bytes).unwrap();

        service_client
            .set_dense_1_bias_const(bias, gamma, beta, mean, variance)
            .send_recv(program_id)
            .await
            .unwrap();
    } else if layer_number == 2 {
        let filters = convert_to_vec(DENSE2_WEIGHT);
        let bias = DENSE2_BIAS.to_vec();

        let bytes = hex::encode(
            [
                "CnnCatsDogs".encode(),
                "SetDense2Const".encode(),
                (filters.clone(), bias.clone()).encode(),
            ]
            .concat(),
        );
        writeln!(file, "\"0x{}\",", bytes).unwrap();

        service_client
            .set_dense_2_const(filters, bias)
            .send_recv(program_id)
            .await
            .unwrap();
    }
}
