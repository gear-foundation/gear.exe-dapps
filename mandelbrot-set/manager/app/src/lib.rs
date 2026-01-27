#![no_std]

use rust_decimal::Decimal;
use sails_rs::{
    cell::RefCell,
    collections::HashMap,
    gstd::{exec, msg},
    prelude::*,
};

#[derive(Default)]
struct ManagerState {
    checkers: Vec<ActorId>,
    point_results: HashMap<u32, (FixedPoint, FixedPoint, u32, bool)>,
    points_sent: u32,
}

impl ManagerState {
    pub fn new() -> Self {
        Self {
            point_results: HashMap::with_capacity(400_000),
            ..Default::default()
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
    pub index: u32,
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
pub struct ManagerService<'a> {
    state: &'a RefCell<ManagerState>,
}

impl<'a> ManagerService<'a> {
    fn create(state: &'a RefCell<ManagerState>) -> Self {
        Self { state }
    }
    #[inline]
    fn get_mut(&self) -> sails_rs::cell::RefMut<'_, ManagerState> {
        self.state.borrow_mut()
    }

    #[inline]
    fn get(&self) -> sails_rs::cell::Ref<'_, ManagerState> {
        self.state.borrow()
    }
}

#[sails_rs::service]
impl<'a> ManagerService<'a> {
    #[export]
    pub async fn add_checkers(&mut self, checkers: Vec<[u16; 32]>) {
        let checkers_bytes: Vec<[u8; 32]> = checkers
            .into_iter()
            .map(|arr_u16| {
                let mut arr_u8 = [0u8; 32];
                for (i, val) in arr_u16.into_iter().enumerate() {
                    arr_u8[i] = u8::try_from(val).expect("value out of range for u8");
                }
                arr_u8
            })
            .collect();
        let checkers: Vec<ActorId> = checkers_bytes.into_iter().map(ActorId::from).collect();
        self.get_mut().checkers.extend(checkers);
    }

    #[export]
    pub fn restart(&mut self) {
        self.get_mut().point_results.clear();
        self.get_mut().points_sent = 0;
    }

    #[export]
    pub fn generate_and_store_points(
        &mut self,
        width: u32,
        height: u32,
        x_min_num: i64,
        x_min_scale: u32,
        x_max_num: i64,
        x_max_scale: u32,
        y_min_num: i64,
        y_min_scale: u32,
        y_max_num: i64,
        y_max_scale: u32,
        points_per_call: u32,
        continue_generation: bool,
        check_points_after_generation: u32,
        max_iter: u32,
        batch_size: u32,
    ) {
        let x_min_dec = Decimal::new(x_min_num, x_min_scale);
        let x_max_dec = Decimal::new(x_max_num, x_max_scale);
        let y_min_dec = Decimal::new(y_min_num, y_min_scale);
        let y_max_dec = Decimal::new(y_max_num, y_max_scale);

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
                    x_min_num,
                    x_min_scale,
                    x_max_num,
                    x_max_scale,
                    y_min_num,
                    y_min_scale,
                    y_max_num,
                    y_max_scale,
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

        if check_points_after_generation > 0
            && self.get_mut().point_results.len() as u32 >= total_points
        {
            let payload = [
                "Manager".encode(),
                "CheckPointsSet".encode(),
                (max_iter, batch_size, check_points_after_generation).encode(),
            ]
            .concat();
            msg::send_bytes(exec::program_id(), payload, 0).expect("Error during msg sending");
        }
    }

    #[export]
    pub fn check_points_set(&mut self, max_iter: u32, batch_size: u32, rounds_left: u32) {
        if rounds_left == 0 {
            return;
        }

        let (checkers, points_len) = {
            let state = self.get();

            if state.checkers.is_empty() || state.point_results.is_empty() {
                return;
            }

            (state.checkers.clone(), state.point_results.len() as u32)
        };
        for checker in checkers.iter() {
            if self.get().points_sent >= points_len {
                break;
            }
            self.send_next_batch(*checker, max_iter, batch_size);
        }

        if rounds_left > 1 && self.get().points_sent < self.get().point_results.len() as u32 {
            let payload = [
                "Manager".encode(),
                "CheckPointsSet".encode(),
                (max_iter, batch_size, rounds_left - 1).encode(),
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

        let points_u16: Vec<u16> = points_to_send.encode().iter().map(|&x| x as u16).collect();
        self.get_mut().points_sent += points_to_send.len() as u32;

        let payload = [
            "MandelbrotChecker".encode(),
            "CheckMandelbrotPoints".encode(),
            (points_u16, max_iter).encode(),
        ]
        .concat();

        msg::send_bytes(checker, payload, 0).expect("Failed to send points to checker");
    }

    #[export]
    pub fn result_calculated(&mut self, indexes: Vec<u32>, results: Vec<u32>) {
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

    #[export]
    pub fn get_points_len(&self) -> u32 {
        self.get().point_results.len() as u32
    }

    #[export]
    pub fn get_checkers(&self) -> Vec<u8> {
        self.get().checkers.clone().encode()
    }

    #[export]
    pub fn points_sent(&self) -> u32 {
        self.get().points_sent
    }

    #[export]
    pub fn get_results(&self, start_index: u32, end_index: u32) -> Vec<u8> {
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
                        index,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<PointResult>>()
            .encode()
    }

    #[export]
    pub fn get_checked_count(&self) -> u32 {
        self.get()
            .point_results
            .values()
            .filter(|(_, _, _, checked)| *checked)
            .count() as u32
    }
}

pub struct ManagerProgram {
    state: RefCell<ManagerState>,
}
#[sails_rs::program]
impl ManagerProgram {
    pub fn init() -> Self {
        Self {
            state: RefCell::new(ManagerState::new()),
        }
    }

    // Exposed service
    pub fn manager(&self) -> ManagerService<'_> {
        ManagerService::create(&self.state)
    }
}
