use image::{ImageBuffer, Rgb};
//use cnn_cats_dogs_client::{FixedPoint, PointResult};
use parity_scale_codec::{Decode, Encode};
use primitive_types::U256;
use reqwest::Client;
use sails_rs::calls::ActionIo;
use sails_rs::ActorId;
use serde::Deserialize;
use serde_json::json;
use serde_json::to_string_pretty;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct Ip {
    jsonrpc: String,
    id: u32,
    result: Res,
}
#[derive(Deserialize, Debug)]
struct Res {
    payload: String,
    value: u32,
    code: Code,
}

#[derive(Deserialize, Debug)]
struct Code {
    Success: String,
}

#[derive(Deserialize, Debug)]
pub struct Point {
    pub c_re: f64,
    pub c_im: f64,
    pub iter: u32,
}

// fn fixed_point_to_f64(fixed_point: &FixedPoint) -> f64 {
//     fixed_point.num as f64 / 10f64.powi(fixed_point.scale as i32)
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let payload =
        hex::encode(cnn_cats_dogs_client::cnn_cats_dogs::io::GetIm2ColDone::encode_call());
    let program_id = "0x7D5321eC8FBb7dA141113d35f7795ade4598A50F";
    let params = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "program_calculateReplyForHandle",
        "params": {
            "source": "0xf823ba3F10922DCca6970D1e012D8701f462Aa33",
            "program_id": program_id,
            "payload": payload,
            "value": 0
        }
    });

    //let url = "http://135.181.114.201:9944";
    let url = "http://localhost:9944";

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&params)
        .send()
        .await?;

    let text = response.json::<Ip>().await?;

    let bytes = hex::decode(&text.result.payload[2..]).unwrap();
    println!(
        "{:?}",
        cnn_cats_dogs_client::cnn_cats_dogs::io::GetIm2ColDone::decode_reply(bytes).unwrap()
    );

    let payload =
        hex::encode(cnn_cats_dogs_client::cnn_cats_dogs::io::GetProcessedCol::encode_call());
    let program_id = "c";
    let params = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "program_calculateReplyForHandle",
        "params": {
            "source": "0xf823ba3F10922DCca6970D1e012D8701f462Aa33",
            "program_id": program_id,
            "payload": payload,
            "value": 0
        }
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&params)
        .send()
        .await?;

    //  println!("{:?}", response);

    let text = response.json::<Ip>().await?;

    let bytes = hex::decode(&text.result.payload[2..]).unwrap();
    // println!("{:?}", cnn_cats_dogs_client::cnn_cats_dogs::io::GetProcessedCol::decode_reply(bytes).unwrap());

    // let mut file = File::create("checkers.txt")?;

    // let json_data: Vec<String> = checkers.iter().map(|id| id.to_string()).collect();

    // let json = to_string_pretty(&json_data)?;
    // writeln!(file, "{}", json)?;

    // let total_points = 250_000;
    // let batch_size = 50_000;

    // let mut checked_points: Vec<PointResult> = Vec::new();
    // for start in (0..total_points).step_by(batch_size) {
    //     let end = (start + batch_size).min(total_points);
    //     let payload = hex::encode(manager_client::manager::io::GetResults::encode_call(
    //         start as u32,
    //         end as u32,
    //     ));

    //     let params = json!({
    //         "jsonrpc": "2.0",
    //         "id": 1,
    //         "method": "program_calculateReplyForHandle",
    //         "params": {
    //             "source": "0xf823ba3F10922DCca6970D1e012D8701f462Aa33",
    //             "program_id": program_id,
    //             "payload": payload,
    //             "value": 0
    //         }
    //     });

    //     let response = client
    //         .post(url)
    //         .header("Content-Type", "application/json;charset=utf-8")
    //         .json(&params)
    //         .send()
    //         .await?;

    //     let text = response.json::<Ip>().await?;

    //     let bytes = hex::decode(&text.result.payload[2..]).unwrap();
    //     let points = manager_client::manager::io::GetResults::decode_reply(bytes).unwrap();
    //     checked_points.extend(points);
    // }

    // println!("Total amount of points {:?}", checked_points.len());
    // if checked_points.iter().all(|point| point.checked) {
    //     println!("All points are checked!");
    // } else {
    //     let unchecked_count = checked_points.iter().filter(|point| !point.checked).count();
    //     println!(
    //         "Some points are not checked. Unchecked points: {}",
    //         unchecked_count
    //     );
    // }

    // let points: Vec<Point> = checked_points
    //     .into_iter()
    //     .map(|point_result| Point {
    //         c_re: fixed_point_to_f64(&point_result.c_re),
    //         c_im: fixed_point_to_f64(&point_result.c_im),
    //         iter: point_result.iter,
    //     })
    //     .collect();

    // //   println!("{:?}", points);

    // let mandelbrot_points_count = points.iter().filter(|point| point.iter == 1000).count();

    // println!("Amount of mandelbrot points {:?}", mandelbrot_points_count);

    // let width = 600;
    // let height = 600;
    // let x_max: f64 = 1.0;
    // let x_min: f64 = -2.0;
    // let y_max: f64 = 1.5;
    // let y_min: f64 = -1.5;
    // let max_iter = 1000;

    // let mut img = ImageBuffer::new(width, height);
    // let scale_x = (x_max - x_min) / width as f64;
    // let scale_y = (y_max - y_min) / height as f64;

    // for point in points {
    //     let x = ((point.c_re - x_min) / scale_x) as u32;
    //     let y = ((point.c_im - y_min) / scale_y) as u32;

    //     if x < width && y < height {
    //         let pixel = img.get_pixel_mut(x, y);

    //         let color = if point.iter == max_iter {
    //             [0, 0, 0]
    //         } else {
    //             let ratio = point.iter as f64 / max_iter as f64;
    //             [
    //                 (255.0 * ratio) as u8,
    //                 (255.0 * (1.0 - ratio)) as u8,
    //                 (255.0 * (0.5 - ratio / 2.0)) as u8,
    //             ]
    //         };

    //         *pixel = Rgb(color);
    //     }
    // }
    //img.save("output.png").expect("Failed to save image");
    // println!("Total amount of points {}", checked_points.len());
    // let mut file = File::create("checked_points.txt")?;
    // //  let json_data: Vec<(String, String, u32)> = checked_points
    // //     .iter()
    // //     .map(|point| ((point.c_re.clone(), point.c_im.clone(), point.iter)))
    // //     .collect();

    // let json = to_string_pretty(&checked_points)?;
    // writeln!(file, "{}", json)?;

    // let payload = hex::encode(manager_client::manager::io::GetResults::encode_call(
    //     0, 90000,
    // ));

    // let params = json!({
    //     "jsonrpc": "2.0",
    //     "id": 1,
    //     "method": "program_calculateReplyForHandle",
    //     "params": {
    //         "source": "0xf823ba3F10922DCca6970D1e012D8701f462Aa33",
    //         "program_id": program_id,
    //         "payload": payload,
    //         "value": 0
    //     }
    // });

    // let url = "http://35.246.144.70:9944";

    // let response = client
    //     .post(url)
    //     .header("Content-Type", "application/json;charset=utf-8")
    //     .json(&params)
    //     .send()
    //     .await?;

    // let text = response.json::<Ip>().await?;
    // // println!("Response: {:?}", text.result.payload);

    // let bytes = hex::decode(&text.result.payload[2..]).unwrap();

    // // let checked_points: Vec<CheckedPoint> =
    //     manager_client::manager::io::GetResults::decode_reply(bytes).unwrap();
    // println!("Amount of processed points {:?}", checked_points.len());

    // let mut file = File::create("checked_points.txt")?;

    // let json_data: Vec<(String, String, u32)> = checked_points
    //     .iter()
    //     .map(|point| ((point.c_re.clone(), point.c_im.clone(), point.iter)))
    //     .collect();

    // let json = to_string_pretty(&json_data)?;
    // writeln!(file, "{}", json)?;

    Ok(())
}
