use cnn_cats_dogs_client::{traits::*, FixedPoint};
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView};
use ndarray::{s, Array3, Array4, Axis};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System}, Encode,
};
use serde_json::Value;
use std::fs::File;
use std::io::Write;

const ACTOR_ID: u64 = 42;

fn fp_from_decimal(decimal: &Decimal) -> FixedPoint {
    let scale = decimal.scale();
    let num = decimal.mantissa() as i128;
    FixedPoint { num, scale }
}

fn flatten_3d_to_1d(pixels: Vec<Vec<Vec<u8>>>) -> Vec<u8> {
    let mut flat_array = Vec::new();

    for layer in pixels.iter() {
        // Проход по слою (depth)
        for row in layer.iter() {
            // Проход по строкам (height)
            for &value in row.iter() {
                // Проход по элементам строки (width)
                flat_array.push(value); // Добавляем элемент в одномерный массив
            }
        }
    }

    flat_array
}

#[tokio::test]
async fn do_something_works() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 1_000_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    let image_path = "10014.jpg";
    let pixels = flatten_3d_to_1d(array3_to_fixed_point(load_and_preprocess_image(image_path)));

    
    // let bytes = hex::encode(["CnnCatsDogs".encode(), "Predict".encode(), (pixels.clone(), true).encode()].concat());
    // println!("bytes {:?}", bytes);
    let batch_size = 200_u16;
    let start_col = 0_u16;
    //let bytes = hex::encode(["CnnCatsDogs".encode(), "ProcessSingleBatch".encode(), (batch_size, start_col, false).encode()].concat());
     let bytes = hex::encode(["CnnCatsDogs".encode(), "Im2Col".encode(), (false).encode()].concat());
     println!("bytes {:?}", bytes);
    let json_data = Value::Array(
        pixels
            .clone()
            .into_iter()
            .map(|value| Value::Number(value.into()))
            .collect(),
    );
    let json_string = serde_json::to_string_pretty(&json_data).expect("Serialization failed");

    // Write to a file
    let mut file = File::create("pixels.json").expect("Unable to create file");
    file.write_all(json_string.as_bytes())
        .expect("Unable to write data");

    println!("Data saved to pixels.json");

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(cnn_cats_dogs::WASM_BINARY);

    let program_factory = cnn_cats_dogs_client::CnnCatsDogsFactory::new(remoting.clone());

    let program_id = program_factory
        .new() // Call program's constructor (see app/src/lib.rs:29)
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = cnn_cats_dogs_client::CnnCatsDogs::new(remoting.clone());

    service_client
        .predict(pixels, false)
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .im_2_col(false)
        .send_recv(program_id)
        .await
        .unwrap();

    let cols = 15876;
    let batch_size = 200;
    for start_col in (0..cols).step_by(batch_size) {
        println!("start col {:?}", start_col);
        service_client
            .process_single_batch(batch_size as u16, start_col, false)
            .send_recv(program_id)
            .await
            .unwrap();
    }

    service_client
        .add_bias_and_relu(0, 16, false)
        .send_recv(program_id)
        .await
        .unwrap();

    service_client
        .add_bias_and_relu(16, 16, false)
        .send_recv(program_id)
        .await
        .unwrap();
}

fn load_and_preprocess_image(path: &str) -> Array3<u8> {
    use image::GenericImageView; // Для доступа к методам изображения

    // Загружаем изображение
    let img = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    // Изменяем размер до 128x128
    let resized_img = img.resize_exact(128, 128, image::imageops::FilterType::Nearest);

    // Преобразуем DynamicImage в RgbaImage (ImageBuffer)
    let resized_img = resized_img.to_rgba8();

    // Преобразуем изображение в массив
    let mut array = Array4::<u8>::zeros((1, 128, 128, 3)); // Размер: 1 x 128 x 128 x 3 (для модели)

    // // Заполняем массив
    for (x, y, pixel) in resized_img.enumerate_pixels() {
        let [r, g, b, _] = pixel.0; // Игнорируем альфа-канал
                                    // array[[0, y as usize, x as usize, 0]] = Decimal::from_f64(r as f64 / 255.0).unwrap();
        array[[0, y as usize, x as usize, 0]] = r;
        // array[[0, y as usize, x as usize, 1]] = Decimal::from_f64(g as f64 / 255.0).unwrap();
        array[[0, y as usize, x as usize, 1]] = g;
        // array[[0, y as usize, x as usize, 2]] = Decimal::from_f64(b as f64 / 255.0).unwrap();
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
                .map(|row| row.iter().map(|&decimal| decimal).collect::<Vec<u8>>())
                .collect::<Vec<Vec<u8>>>()
        })
        .collect::<Vec<Vec<Vec<u8>>>>()
}
