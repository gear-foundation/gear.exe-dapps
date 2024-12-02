#![no_std]

use rust_decimal::Decimal;
use sails_rs::{
    collections::HashMap,
    gstd::{exec, msg},
    prelude::*,
};
static mut STATE: Option<ManagerState> = None;
#[derive(Default)]
struct ManagerState {
    checkers: Vec<ActorId>,
    point_results: HashMap<u32, (FixedPoint, FixedPoint, u32, bool)>,
    points_sent: u32,
}

impl ManagerState {
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
            point_results: HashMap::with_capacity(400_000),
            points_sent: 0,
        }
    }
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Point {
    pub index: u32,
    pub c_re: FixedPoint,
    pub c_im: FixedPoint,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct PointResult {
    pub c_re: FixedPoint,
    pub c_im: FixedPoint,
    pub iter: u32,
    pub checked: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i64,
    pub scale: u32,
}

impl FixedPoint {
    pub fn from_decimal(decimal: Decimal) -> Self {
        let scale = decimal.scale();
        let num = decimal.mantissa() as i64;
        Self { num, scale }
    }
}
struct ManagerService(());

impl ManagerService {
    pub fn init() -> Self {
        unsafe { STATE = Some(ManagerState::new()) }
        Self(())
    }
    pub fn get_mut(&mut self) -> &'static mut ManagerState {
        unsafe { STATE.as_mut().expect("STATE is not initialized") }
    }
    pub fn get(&self) -> &'static ManagerState {
        unsafe { STATE.as_ref().expect("STATE is not initialized") }
    }
}

#[sails_rs::service]
impl ManagerService {
    pub fn new() -> Self {
        Self(())
    }

    pub async fn add_checkers(&mut self, checkers: Vec<ActorId>) {
        self.get_mut().checkers.extend(checkers);
    }

    pub fn restart(&mut self) {
        self.get_mut().point_results.clear();
        self.get_mut().points_sent = 0;
    }
    pub fn generate_and_store_points(
        &mut self,
        width: u32,
        height: u32,
        x_min: FixedPoint,
        x_max: FixedPoint,
        y_min: FixedPoint,
        y_max: FixedPoint,
        points_per_call: u32,
        continue_generation: bool,
        check_points_after_generation: bool,
        max_iter: u32,
        batch_size: u32,
    ) {
        let x_min_dec = Decimal::new(x_min.num, x_min.scale);
        let x_max_dec = Decimal::new(x_max.num, x_max.scale);
        let y_min_dec = Decimal::new(y_min.num, y_min.scale);
        let y_max_dec = Decimal::new(y_max.num, y_max.scale);

        let scale_x = (x_max_dec - x_min_dec) / Decimal::from(width);
        let scale_y = (y_max_dec - y_min_dec) / Decimal::from(height);

        let total_points = width * height;
        let total_generated_points = self.get_mut().point_results.len() as u32;

        if total_generated_points >= total_points {
            return;
        }

        let starting_index = total_generated_points;

        for i in starting_index..starting_index + points_per_call.min(total_points - starting_index)
        {
            let x = i / width;
            let y = i % width;

            let c_re = FixedPoint::from_decimal(x_min_dec + Decimal::from(x) * scale_x);
            let c_im = FixedPoint::from_decimal(y_min_dec + Decimal::from(y) * scale_y);

            self.get_mut()
                .point_results
                .insert(i, (c_re, c_im, 0, false));
        }

        if continue_generation && total_generated_points < total_points {
            let payload = [
                "Manager".encode(),
                "GenerateAndStorePoints".encode(),
                (
                    width,
                    height,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                    points_per_call,
                    continue_generation,
                    check_points_after_generation,
                    max_iter,
                    batch_size,
                )
                    .encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }

        if check_points_after_generation && self.get_mut().point_results.len() as u32 >= total_points {
            let payload = [
                "Manager".encode(),
                "CheckPointsSet".encode(),
                (max_iter, batch_size, true).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }
    }

    pub fn check_points_set(&mut self, max_iter: u32, batch_size: u32, continue_checking: bool) {
        let checkers = &self.get().checkers;
        let points = &self.get().point_results;

        if checkers.is_empty() || points.is_empty() {
            return;
        }

        for checker in checkers.iter() {
            if self.get().points_sent >= points.len() as u32 {
                break;
            }
            self.send_next_batch(*checker, max_iter, batch_size);
        }
        if continue_checking && self.get().points_sent < self.get().point_results.len() as u32 {
            let payload = [
                "Manager".encode(),
                "CheckPointsSet".encode(),
                (max_iter, batch_size, continue_checking).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }
    }

    pub fn send_next_batch(&mut self, checker: ActorId, max_iter: u32, batch_size: u32) {
        let mut points_to_send = Vec::new();

        let start = self.get().points_sent as u32;
        let end = start + batch_size;

        for i in start..end {
            if let Some((c_re, c_im, _, _)) = self.get().point_results.get(&i) {
                points_to_send.push(Point {
                    index: i,
                    c_re: c_re.clone(),
                    c_im: c_im.clone(),
                });
            }
        }

        if points_to_send.is_empty() {
            return;
        }

        self.get_mut().points_sent += points_to_send.len() as u32;

        let payload = [
            "MandelbrotChecker".encode(),
            "CheckMandelbrotPoints".encode(),
            (points_to_send, max_iter).encode(),
        ]
        .concat();

        msg::send_bytes(checker, payload, 0).expect("Failed to send points to checker");
    }

    pub fn result_calculated(&mut self, indexes: Vec<u32>, results: Vec<u32>) {
        // sails_rs::gstd::debug!("Received indexes from {} to {}", indexes[0], indexes[19]);
        indexes
            .into_iter()
            .zip(results)
            .for_each(|(index, result)| {
                if let Some(point) = self.get_mut().point_results.get_mut(&index) {
                    point.2 = result;
                    point.3 = true;
                }
            });
    }

    pub fn get_points_len(&self) -> u32 {
        self.get().point_results.len() as u32
    }

    pub fn get_checkers(&self) -> Vec<ActorId> {
        self.get().checkers.clone()
    }

    pub fn points_sent(&self) -> u32 {
        self.get().points_sent
    }

    pub fn get_results(&self, start_index: u32, end_index: u32) -> Vec<PointResult> {
        let results = &self.get().point_results;

        results
            .iter()
            .filter_map(|(&index, &(ref c_re, ref c_im, iter, checked))| {
                if index >= start_index && index < end_index {
                    Some(PointResult {
                        c_re: c_re.clone(),
                        c_im: c_im.clone(),
                        iter,
                        checked,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

pub struct ManagerProgram(());

#[sails_rs::program]
impl ManagerProgram {
    // Program's constructor
    pub fn new() -> Self {
        ManagerService::init();
        Self(())
    }

    // Exposed service
    pub fn manager(&self) -> ManagerService {
        ManagerService::new()
    }
}
