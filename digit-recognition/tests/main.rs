use digit_recognition_client::{traits::*, FixedPoint};
use eframe::egui;
use eframe::App;
use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};
use std::sync::{Arc, Mutex};
use image::{DynamicImage, GenericImageView, ImageBuffer, imageops::FilterType};

const ACTOR_ID: u64 = 42;

fn downscale_canvas(canvas: &[u8], high_width: usize, high_height: usize, low_width: u32, low_height: u32) -> Vec<u8> {
    let high_res_image = DynamicImage::ImageLuma8(
        ImageBuffer::from_raw(high_width as u32, high_height as u32, canvas.to_vec())
            .expect("Failed to create high-resolution image buffer"),
    );
    let low_res_image = high_res_image.resize_exact(low_width, low_height, FilterType::Lanczos3);

    low_res_image
        .to_luma8()
        .pixels()
        .map(|p| p.0[0])
        .collect()
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to create runtime");

    runtime.block_on(async_main());
}

async fn async_main() {
    let options = eframe::NativeOptions::default();
    let pixels = Arc::new(Mutex::new(Vec::new()));
    let pixels_clone = pixels.clone();

    let _ = eframe::run_native(
        "MNIST",
        options,
        Box::new(|_cc| {
            let app = MnistApp::new(pixels_clone);
            Ok(Box::new(app))
        }),
    );

    let pixels = pixels.lock().unwrap().clone();

    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    let program_code_id = remoting
        .system()
        .submit_code(digit_recognition::WASM_BINARY);

    let program_factory = digit_recognition_client::DigitRecognitionFactory::new(remoting.clone());

    let program_id = program_factory
        .new() 
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = digit_recognition_client::DigitRecognition::new(remoting.clone());

    let result = service_client
        .predict(pixels) // Call service's method
        .send_recv(program_id)
        .await
        .unwrap();

    let result_f64: Vec<f64> = result.iter().map(|fp| fixed_point_to_float(fp)).collect();
    println!("result f64 {:?}", result_f64.clone());
    if let Some((predicted_digit, &max_probability)) = result_f64
        .iter()
        .enumerate()
        .max_by(|(_, prob1), (_, prob2)| prob1.partial_cmp(prob2).unwrap())
    {
        println!(
            "Predicted digit: {}, with probability: {:?}",
            predicted_digit,
            max_probability * 100.0
        );
    } else {
        println!("Error: Probability array is empty");
    }
}

struct MnistApp {
    high_res_canvas: Vec<u8>, 
    canvas_size: (usize, usize), 
    low_res_size: (u32, u32),    
    brush_size: usize,
    brush_intensity: u8,
    done: Arc<Mutex<bool>>,
    pixels: Arc<Mutex<Vec<u16>>>,
    last_mouse_pos: Option<(usize, usize)>
}

impl App for MnistApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if *self.done.lock().unwrap() {
                println!("Stop drawing");
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }

            let (width, height) = self.canvas_size;
            let painter = ui.painter();
            let rect = ui.available_rect_before_wrap();
            let pixel_size = (rect.width() / width as f32, rect.height() / height as f32);

            for y in 0..height {
                for x in 0..width {
                    let color = self.high_res_canvas[y * width + x];
                    let color = egui::Color32::from_gray(color);
                    let pixel_rect = egui::Rect::from_min_size(
                        rect.min + egui::vec2(x as f32 * pixel_size.0, y as f32 * pixel_size.1),
                        egui::vec2(pixel_size.0, pixel_size.1),
                    );
                    painter.rect_filled(pixel_rect, 0.0, color);
                }
            }

            if ui.input(|i| i.pointer.primary_down()) {
                if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    let px = ((mouse_pos.x - rect.min.x) / pixel_size.0) as usize;
                    let py = ((mouse_pos.y - rect.min.y) / pixel_size.1) as usize;

                    if px < width && py < height {
                        self.apply_brush(px, py);
                    }
                }
            } else {
                self.last_mouse_pos = None;
            }

            if ui.button("Finish drawing").clicked() {
                let mut done = self.done.lock().unwrap();
                if !*done {
                    *done = true;
                    self.save_pixels();
                }
            }
        });
    }
}

impl MnistApp {
    pub fn new(pixels: Arc<Mutex<Vec<u16>>>) -> Self {
        Self {
            high_res_canvas: vec![0; 500 * 500],
            canvas_size: (500, 500),
            low_res_size: (28, 28),
            brush_size: 15,
            brush_intensity: 128,
            done: Arc::new(Mutex::new(false)),
            pixels,
            last_mouse_pos: None,
        }
    }

    fn apply_brush(&mut self, px: usize, py: usize) {
        let (width, height) = self.canvas_size;

        if let Some((prev_x, prev_y)) = self.last_mouse_pos {
            let dx = px as isize - prev_x as isize;
            let dy = py as isize - prev_y as isize;
            let steps = dx.abs().max(dy.abs()); 
        
            if steps > 0 {
                for step in 0..=steps {
                    let interp_x = prev_x as isize + (dx * step / steps) as isize;
                    let interp_y = prev_y as isize + (dy * step / steps) as isize;
        
                    self.apply_brush_to_point(interp_x as usize, interp_y as usize);
                }
            }
        }

        self.apply_brush_to_point(px, py);

        self.last_mouse_pos = Some((px, py));
    }

    fn apply_brush_to_point(&mut self, px: usize, py: usize) {
        let (width, height) = self.canvas_size;
        let brush_half = self.brush_size / 2;

        for dy in 0..self.brush_size {
            for dx in 0..self.brush_size {
                let x = px + dx - brush_half;
                let y = py + dy - brush_half;

                if x < width && y < height {
                    let idx = y * width + x;
                    self.high_res_canvas[idx] = self.high_res_canvas[idx].saturating_add(self.brush_intensity);
                }
            }
        }
    }

    fn save_pixels(&self) {
        let low_res_canvas = downscale_canvas(
            &self.high_res_canvas,
            self.canvas_size.0,
            self.canvas_size.1,
            self.low_res_size.0,
            self.low_res_size.1,
        );
        let mut pixels = self.pixels.lock().unwrap();
        *pixels = low_res_canvas.iter().map(|&v| v as u16).collect();

        let low_res_image = DynamicImage::ImageLuma8(
            ImageBuffer::from_raw(self.low_res_size.0, self.low_res_size.1, low_res_canvas)
                .expect("Failed to create low-resolution image buffer"),
        );

        let output_path = "low_res_output.png";
        low_res_image
            .save(output_path)
            .expect("Failed to save low-resolution image");
        println!("Image saved as {}", output_path);
    }
}

fn fixed_point_to_float(fixed_point: &FixedPoint) -> f64 {
    let scale_factor = 10_f64.powi(fixed_point.scale as i32);
    fixed_point.num as f64 / scale_factor
}
