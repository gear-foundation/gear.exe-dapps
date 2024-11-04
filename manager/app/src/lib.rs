#![no_std]

use rust_decimal::Decimal;
use sails_rs::{
    calls::*,
    gstd::{calls::{GStdRemoting}, msg},
    prelude::*,
};
static mut STATE: Option<ManagerState> = None;
use mandelbrot_checker_client::{
    mandelbrot_checker,
    traits::MandelbrotCheckerFactory as MandelbrotCheckerFactoryTrait,
    MandelbrotCheckerFactory,
};
#[derive(Default)]
struct ManagerState {
    checkers: Vec<ActorId>,
    points: Vec<(String, String)>,
    results: Vec<(String, String, u32)>,
    points_sent: u32,
}
struct ManagerService(());


impl ManagerService 
{
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
impl ManagerService
{
    pub fn new() -> Self {
        Self(())
    }

    pub async fn deploy_checkers(&mut self, code_id: CodeId, amount: u32) {
        let checker_factory = MandelbrotCheckerFactory::new(GStdRemoting);
       
        let msg_id: [u8; 32] = msg::id().into();
        for i in 0..amount {
            let salt = generate_salt(&msg_id, i);
            let program_id = checker_factory
                .new()
                .send_recv(code_id, salt)
                .await
                .expect("Error during program creation");
            self.get_mut().checkers.push(program_id);
        }
    }

    pub fn generate_and_store_points(
        &mut self,
        width: u32,
        height: u32,
        x_min: String,
        x_max: String,
        y_min: String,
        y_max: String,
    ) {
        let x_min = Decimal::from_str_exact(&x_min).expect("Invalid x_min format");
        let x_max = Decimal::from_str_exact(&x_max).expect("Invalid x_max format");
        let y_min = Decimal::from_str_exact(&y_min).expect("Invalid y_min format");
        let y_max = Decimal::from_str_exact(&y_max).expect("Invalid y_max format");

        let scale_x = (x_max - x_min) / Decimal::from(width);
        let scale_y = (y_max - y_min) / Decimal::from(height);

        self.get_mut().points.clear();
        for x in 0..width {
            for y in 0..height {
                let c_re = x_min + Decimal::from(x) * scale_x;
                let c_im = y_min + Decimal::from(y) * scale_y;
                self.get_mut()
                    .points
                    .push((c_re.to_string(), c_im.to_string()));
            }
        }
        self.get_mut().points_sent = 0;
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
    }

    pub fn send_next_batch(&mut self, checker: ActorId, max_iter: u32, batch_size: u32) {
        let points = &self.get().points;
        let start = self.get().points_sent as usize;
        let end = (self.get().points_sent + batch_size).min(points.len() as u32) as usize;

        if start >= end {
            return;
        }

        let points_chunk = points[start..end].to_vec();
        let request = mandelbrot_checker::io::CheckMandelbrotPoints::encode_call(
            points_chunk.clone(),
            max_iter,
        );

        msg::send_bytes(checker, request, 0).expect("Failed to send points to checker");

        self.get_mut().points_sent += points_chunk.len() as u32;
    }

    pub fn result_calculated(&mut self, points: Vec<(String, String)>, results: Vec<u32>) {
        for (i, iterations) in results.into_iter().enumerate() {
            let point = &points[i];
            self.get_mut()
                .results
                .push((point.0.clone(), point.1.clone(), iterations));
        }
    }

    pub fn get_points(&self) -> Vec<(String, String)> {
        self.get().points.clone()
    }

    pub fn get_checkers(&self) -> Vec<ActorId> {
        self.get().checkers.clone()
    }

    pub fn points_sent(&self) -> u32 {
        self.get().points_sent
    }

    pub fn get_results(&self) -> Vec<(String, String, u32)> {
        self.get().results.clone()
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

fn generate_salt(base_value: &[u8; 32], index: u32) -> Vec<u8> {
    let mut salt = base_value.clone();
    let index_bytes = index.to_le_bytes();

    salt[28..32].copy_from_slice(&index_bytes);

    salt.to_vec()
}
