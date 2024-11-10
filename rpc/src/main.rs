use manager_client::Result as CheckedPoint;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let payload = hex::encode(manager_client::manager::io::GetCheckers::encode_call());
    let program_id = "0x15616Be06B07A93bb89614c32cbCb979c37784AD";
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

    let url = "http://35.246.144.70:9944";

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&params)
        .send()
        .await?;

    let text = response.json::<Ip>().await?;

    let bytes = hex::decode(&text.result.payload[2..]).unwrap();
    let mut checkers = manager_client::manager::io::GetCheckers::decode_reply(bytes).unwrap();
    checkers.retain(|&addr| addr != ActorId::zero());
    println!("Amount of checkers {:?}", checkers.len());

    let mut file = File::create("checkers.txt")?;

    let json_data: Vec<String> = checkers.iter().map(|id| id.to_string()).collect();

    let json = to_string_pretty(&json_data)?;
    writeln!(file, "{}", json)?;

    let payload = hex::encode(manager_client::manager::io::GetPoints::encode_call());

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

    let url = "http://35.246.144.70:9944";

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&params)
        .send()
        .await?;

    let text = response.json::<Ip>().await?;

    let bytes = hex::decode(&text.result.payload[2..]).unwrap();
    println!(
        "Total amount of points {:?}",
        manager_client::manager::io::GetPoints::decode_reply(bytes)
            .unwrap()
            .len()
    );

    let payload = hex::encode(manager_client::manager::io::GetResults::encode_call(0, 90000));

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

    let url = "http://35.246.144.70:9944";

    let response = client
        .post(url)
        .header("Content-Type", "application/json;charset=utf-8")
        .json(&params)
        .send()
        .await?;

    let text = response.json::<Ip>().await?;
    // println!("Response: {:?}", text.result.payload);

    let bytes = hex::decode(&text.result.payload[2..]).unwrap();

    let checked_points: Vec<CheckedPoint> =
        manager_client::manager::io::GetResults::decode_reply(bytes).unwrap();
    println!("Amount of processed points {:?}", checked_points.len());

    let mut file = File::create("checked_points.txt")?;

    let json_data: Vec<(String, String, u32)> = checked_points
        .iter()
        .map(|point| ((point.c_re.clone(), point.c_im.clone(), point.iter)))
        .collect();

    let json = to_string_pretty(&json_data)?;
    writeln!(file, "{}", json)?;

    Ok(())
}
