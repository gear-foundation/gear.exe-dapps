#![no_std]

use rust_decimal::Decimal;
use sails_rs::{
    gstd::{exec, msg},
    prelude::*,
};
static mut STATE: Option<ManagerState> = None;
#[derive(Default)]
struct ManagerState {
    checkers: Vec<ActorId>,
    points: Vec<Point>,
    results: Vec<Result>,
    points_sent: u32,
    points_generated: u32,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Point {
    pub c_re: String,
    pub c_im: String,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Result {
    pub c_re: String,
    pub c_im: String,
    pub iter: u32,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i64,
    pub scale: u32,
}

struct ManagerService(());

impl ManagerService {
    pub fn init() -> Self {
        unsafe { STATE = Some(ManagerState::default()) }
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
        self.get_mut().points.clear();
        self.get_mut().results.clear();
        self.get_mut().points_generated = 0;
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
    ) {
        let x_min_dec = Decimal::new(x_min.num, x_min.scale);
        let x_max_dec = Decimal::new(x_max.num, x_max.scale);
        let y_min_dec = Decimal::new(y_min.num, y_min.scale);
        let y_max_dec = Decimal::new(y_max.num, y_max.scale);

        let scale_x = (x_max_dec - x_min_dec) / Decimal::from(width);
        let scale_y = (y_max_dec - y_min_dec) / Decimal::from(height);

        let total_points = width * height;
        let mut total_generated_points = self.get_mut().points.len() as u32;
        let mut generated_in_call = 0;

        let starting_x = total_generated_points / height;
        let starting_y = total_generated_points % height;

        for x in starting_x..width {
            for y in if x == starting_x { starting_y } else { 0 }..height {
                if generated_in_call >= points_per_call {
                    break;
                }

                let c_re = x_min_dec + Decimal::from(x) * scale_x;
                let c_im = y_min_dec + Decimal::from(y) * scale_y;
                self.get_mut().points.push(Point {
                    c_re: c_re.to_string(),
                    c_im: c_im.to_string(),
                });

                generated_in_call += 1;
                total_generated_points += 1;
            }
            if generated_in_call >= points_per_call {
                break;
            }
        }

        self.get_mut().points_generated = total_generated_points;

        if total_generated_points < total_points {
            let payload = [
                "Manager".encode(),
                "GenerateAndStorePoints".encode(),
                (width, height, x_min, x_max, y_min, y_max, points_per_call).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }
    }

    pub fn check_points_set(&mut self, max_iter: u32, batch_size: u32) {
        let checkers = &self.get().checkers;
        let points = &self.get().points;

        if checkers.is_empty() || points.is_empty() {
            return;
        }

        for checker in checkers.iter() {
            if self.get().points_sent >= points.len() as u32 {
                break;
            }
            self.send_next_batch(*checker, max_iter, batch_size);
        }
        if self.get().points_sent < self.get().points.len() as u32 {
            let payload = [
                "Manager".encode(),
                "CheckPointsSet".encode(),
                (max_iter, batch_size).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }
    }

    pub fn send_next_batch(&mut self, checker: ActorId, max_iter: u32, batch_size: u32) {
        let points = &self.get().points;
        let start = self.get().points_sent as usize;
        let end = (self.get().points_sent + batch_size).min(points.len() as u32) as usize;

        if start >= end {
            return;
        }

        let points_chunk = points[start..end].to_vec();

        self.get_mut().points_sent += points_chunk.len() as u32;

        let payload = [
            "MandelbrotChecker".encode(),
            "CheckMandelbrotPoints".encode(),
            (points_chunk, max_iter).encode(),
        ]
        .concat();

        msg::send_bytes(checker, payload, 0).expect("Failed to send points to checker");
    }

    pub fn result_calculated(&mut self, points: Vec<Point>, results: Vec<u32>) {
        for (i, iterations) in results.into_iter().enumerate() {
            let point = &points[i];
            self.get_mut().results.push(Result {
                c_re: point.c_re.clone(),
                c_im: point.c_im.clone(),
                iter: iterations,
            });
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        self.get().points.clone()
    }

    pub fn get_points_len(&self) -> u32 {
        self.get().points.len() as u32
    }

    pub fn get_checkers(&self) -> Vec<ActorId> {
        self.get().checkers.clone()
    }

    pub fn points_sent(&self) -> u32 {
        self.get().points_sent
    }

    pub fn get_results(&self, start_index: u32, end_index: u32) -> Vec<Result> {
        let results = &self.get().results;
        if start_index >= results.len() as u32 {
            return Vec::new();
        }

        let end_index = if end_index < results.len() as u32 {
            end_index
        } else {
            results.len() as u32
        };

        results[start_index as usize..end_index as usize].to_vec()
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
