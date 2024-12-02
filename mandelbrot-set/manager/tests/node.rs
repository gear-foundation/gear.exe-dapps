use gclient::{EventProcessor, GearApi, Result};
use manager_client::{FixedPoint, PointResult};
use sails_rs::{ActorId, Decode, Encode};
mod utils;
use tokio::time::{sleep, Duration};
use utils::*;

#[tokio::test]
async fn test_check_point_set() -> Result<()> {
    let api = GearApi::dev().await?;

    let mut listener = api.subscribe().await?;
    assert!(listener.blocks_running().await?);

    // Init
    let (message_id, manager_id, checker_ids) = init(&api).await;
    assert!(listener.message_processed(message_id).await?.succeed());
    println!("Manager is uploaded");

    // Add checkers
    let message_id = send_request!(api: &api, program_id: manager_id, service_name: "Manager", action: "AddCheckers", payload: (checker_ids));
    assert!(listener.message_processed(message_id).await?.succeed());
    println!("Checkers are added");

    // Generate points
    for _i in 0..12 {
        let width: u32 = 600;
        let height = 600;
        let x_min = FixedPoint { num: -2, scale: 0 };
        let x_max = FixedPoint { num: 1, scale: 0 };
        let y_min = FixedPoint { num: -15, scale: 2 };
        let y_max = FixedPoint { num: 15, scale: 1 };
        let point_per_call: u32 = 30_000;
        let max_iter: u32 = 0;
        let batch_size: u32 = 0;
        let continue_generation = false;
        let check_points_after_generation = false;
        let message_id = send_request!(api: &api, program_id: manager_id, service_name: "Manager", action: "GenerateAndStorePoints", payload: (width, height, x_min, x_max, y_min, y_max, point_per_call, continue_generation, check_points_after_generation, max_iter, batch_size));
        assert!(listener.message_processed(message_id).await?.succeed());
        println!("{} are generated", (_i + 1) * point_per_call);
    }

    println!("Points are generated");

    // Check point set
    let max_iter: u32 = 1000;
    let batch_size: u32 = 20;
    let continue_checking = false;
    for _i in 0..1 {
        let message_id = send_request!(api: &api, program_id: manager_id, service_name: "Manager", action: "CheckPointsSet", payload: (max_iter, batch_size, continue_checking));
        assert!(listener.message_processed(message_id).await?.succeed());
        println!("Sent {} message to check points", _i + 1);
    }

    sleep(Duration::from_secs(5)).await;

    // Get checked points
    let start_index: u32 = 0;
    let end_index: u32 = 2_000;
    let point_results = get_state!(api: &api, listener: listener, program_id: manager_id, service_name: "Manager", action: "GetResults", return_type: Vec<PointResult>, payload: (start_index, end_index));

    if point_results.iter().all(|point| point.checked) {
        println!("All points are checked!");
    } else {
        let unchecked_count = point_results.iter().filter(|point| !point.checked).count();
        println!(
            "Some points are not checked. Unchecked points: {}",
            unchecked_count
        );
    }

    Ok(())
}
