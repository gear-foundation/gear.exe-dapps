#![no_std]

use core::ops::{Add, Mul, Sub};
use sails_rs::{gstd::msg, prelude::*};
struct MandelbrotCheckerService(());

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct Point {
    pub index: u32,
    pub c_re: FixedPoint,
    pub c_im: FixedPoint,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct FixedPoint {
    pub num: i64,
    pub scale: u32,
}

impl MandelbrotCheckerService {
    pub fn create() -> Self {
        Self(())
    }
}

#[sails_rs::service]
impl MandelbrotCheckerService {
    #[export]
    pub fn check_mandelbrot_points(&mut self, points: Vec<u16>, max_iter: u32) {
        let point_bytes: Vec<u8> = points
            .iter()
            .map(|&x| u8::try_from(x).expect("value out of range for u8"))
            .collect();

        let points = Vec::<Point>::decode(&mut point_bytes.as_slice()).expect("Unable to decode");
        let (indexes, results): (Vec<u32>, Vec<u32>) = points
            .into_iter()
            .map(|point| {
                let iter = check_mandelbrot(point.c_re, point.c_im, max_iter);
                (point.index, iter)
            })
            .unzip();

        let payload = [
            "Manager".encode(),
            "ResultCalculated".encode(),
            (indexes, results.clone()).encode(),
        ]
        .concat();
        msg::send_bytes(msg::source(), payload, 0).expect("Error: msg sending");
    }
}

pub struct MandelbrotCheckerProgram(());

#[sails_rs::program]
impl MandelbrotCheckerProgram {
    pub fn init() -> Self {
        Self(())
    }

    pub fn mandelbrot_checker(&self) -> MandelbrotCheckerService {
        MandelbrotCheckerService::create()
    }
}

fn check_mandelbrot(c_re_fp: FixedPoint, c_im_fp: FixedPoint, max_iter: u32) -> u32 {
    let c_re = c_re_fp.to_q32();
    let c_im = c_im_fp.to_q32();

    let mut z_re = c_re;
    let mut z_im = c_im;

    let threshold = Q32_32::from_int(4); // 4.0

    for i in 0..max_iter {
        let z_re2 = z_re.sqr();
        let z_im2 = z_im.sqr();

        // |z|^2 = z_re^2 + z_im^2
        let modulus_squared = z_re2 + z_im2;

        if modulus_squared.0 > threshold.0 {
            return i;
        }

        // new_re = z_re^2 - z_im^2 + c_re
        let new_re = z_re2 - z_im2 + c_re;

        // new_im = 2 * z_re * z_im + c_im
        let new_im = (z_re * z_im).mul_i32(2) + c_im;

        z_re = new_re;
        z_im = new_im;
    }

    max_iter
}

#[derive(Clone, Copy)]
pub struct Q32_32(pub i64);

impl Q32_32 {
    pub const FRAC_BITS: u32 = 32;

    pub const fn from_int(n: i32) -> Self {
        Q32_32((n as i64) << Self::FRAC_BITS)
    }

    #[inline]
    pub fn sqr(self) -> Self {
        self * self
    }

    #[inline]
    pub fn mul_i32(self, k: i32) -> Self {
        let prod = (self.0 as i128) * (k as i128);
        Q32_32(prod as i64)
    }

    #[inline]
    pub fn from_num_scale10(num: i64, scale10: u32) -> Self {
        // value = num / 10^scale10
        // Q = value * 2^FRAC_BITS = num * 2^FRAC_BITS / 10^scale10
        let pow10 = 10_i64.pow(scale10);
        let scaled = (num as i128) << Self::FRAC_BITS;
        let val = scaled / (pow10 as i128);
        Q32_32(val as i64)
    }
}

impl Add for Q32_32 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Q32_32(((self.0 as i128) + (rhs.0 as i128)) as i64)
    }
}

impl Sub for Q32_32 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Q32_32(((self.0 as i128) - (rhs.0 as i128)) as i64)
    }
}

impl Mul for Q32_32 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        // (a / 2^F) * (b / 2^F) = (a*b) / 2^(2F) → shift FRAC_BITS
        let prod = (self.0 as i128) * (rhs.0 as i128);
        Q32_32((prod >> Self::FRAC_BITS) as i64)
    }
}

impl FixedPoint {
    pub fn to_q32(&self) -> Q32_32 {
        Q32_32::from_num_scale10(self.num, self.scale)
    }
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    fn check_mandelbrot_decimal(c_re: Decimal, c_im: Decimal, max_iter: u32) -> u32 {
        let mut z_re = c_re;
        let mut z_im = c_im;

        let threshold = Decimal::from(4);

        for i in 0..max_iter {
            let modulus_squared = z_re * z_re + z_im * z_im;
            if modulus_squared > threshold {
                return i;
            }

            let new_re = z_re * z_re - z_im * z_im + c_re;
            let new_im = Decimal::from(2) * z_re * z_im + c_im;

            z_re = new_re;
            z_im = new_im;
        }

        max_iter
    }

    #[inline]
    fn fp(num: i64, scale: u32) -> FixedPoint {
        FixedPoint { num, scale }
    }

    #[test]
    fn sample_points_match_decimal() {
        let max_iter = 1000_u32;
        let scale10: u32 = 1;

        let cases: &[(i64, i64)] = &[
            (0, 0),   // 0 + 0i
            (-10, 0), // -1.0 + 0i
            (-5, 0),  // -0.5 + 0i
            (5, 5),   // 0.5 + 0.5i
            (15, 0),  // 1.5 + 0i
            (20, 0),  // 2.0 + 0i
            (20, 20), // 2.0 + 2.0i
        ];

        for &(re_num, im_num) in cases {
            let fp_re = fp(re_num, scale10);
            let fp_im = fp(im_num, scale10);

            let iter_q = check_mandelbrot(fp_re.clone(), fp_im.clone(), max_iter);

            let dec_re = Decimal::new(re_num, scale10);
            let dec_im = Decimal::new(im_num, scale10);
            let iter_dec = check_mandelbrot_decimal(dec_re, dec_im, max_iter);

            assert_eq!(
                iter_q, iter_dec,
                "mismatch for c = {} / 10 + i * {} / 10",
                re_num, im_num
            );
        }
    }

    #[test]
    fn grid_step_0_1_matches_decimal() {
        let max_iter = 300_u32;
        // num / 10 → step 0.1
        let scale10: u32 = 1;

        // Re: from -2.0 to 1.0  → [-20..=10]
        // Im: from -1.5 to 1.5  → [-15..=15]
        let re_min = -20;
        let re_max = 10;
        let im_min = -15;
        let im_max = 15;

        let mut mismatches: Vec<(i64, i64, u32, u32)> = Vec::new();

        'outer: for re_num in re_min..=re_max {
            for im_num in im_min..=im_max {
                let fp_re = fp(re_num as i64, scale10);
                let fp_im = fp(im_num as i64, scale10);

                let iter_q = check_mandelbrot(fp_re.clone(), fp_im.clone(), max_iter);

                let dec_re = Decimal::new(re_num as i64, scale10);
                let dec_im = Decimal::new(im_num as i64, scale10);
                let iter_dec = check_mandelbrot_decimal(dec_re, dec_im, max_iter);

                if iter_q != iter_dec {
                    mismatches.push((re_num as i64, im_num as i64, iter_q, iter_dec));
                    if mismatches.len() >= 20 {
                        break 'outer;
                    }
                }
            }
        }

        if !mismatches.is_empty() {
            use std::fmt::Write as _;
            let mut msg = String::from("Found mismatches on 0.1 grid (up to 20 shown):\n");
            for (re_num, im_num, q_it, d_it) in &mismatches {
                let re = *re_num as f64 / 10.0;
                let im = *im_num as f64 / 10.0;
                let _ = writeln!(
                    &mut msg,
                    "  c = ({:.1}) + i*({:.1}): Q32 = {}, Decimal = {}",
                    re, im, q_it, d_it
                );
            }
            panic!("{msg}");
        }
    }

    #[test]
    fn dense_strip_step_0_01_matches_decimal() {
        let max_iter = 300_u32;
        let scale10: u32 = 2; // num / 100 → step 0.01

        let re_min = -150;
        let re_max = -100;
        let im_min = -50;
        let im_max = 50;

        let mut mismatches: Vec<(i64, i64, u32, u32)> = Vec::new();

        'outer: for re_num in re_min..=re_max {
            for im_num in im_min..=im_max {
                let fp_re = fp(re_num as i64, scale10);
                let fp_im = fp(im_num as i64, scale10);

                let iter_q = check_mandelbrot(fp_re.clone(), fp_im.clone(), max_iter);

                let dec_re = Decimal::new(re_num as i64, scale10);
                let dec_im = Decimal::new(im_num as i64, scale10);
                let iter_dec = check_mandelbrot_decimal(dec_re, dec_im, max_iter);

                if iter_q != iter_dec {
                    mismatches.push((re_num as i64, im_num as i64, iter_q, iter_dec));
                    if mismatches.len() >= 50 {
                        break 'outer;
                    }
                }
            }
        }

        if !mismatches.is_empty() {
            use std::fmt::Write as _;
            let mut msg = String::from("Found mismatches on dense 0.001 strip (up to 50 shown):\n");
            for (re_num, im_num, q_it, d_it) in &mismatches {
                let re = *re_num as f64 / 10f64.powi(scale10 as i32);
                let im = *im_num as f64 / 10f64.powi(scale10 as i32);
                let _ = writeln!(
                    &mut msg,
                    "  c = ({:.3}) + i*({:.3}): Q32 = {}, Decimal = {}",
                    re, im, q_it, d_it
                );
            }
            panic!("{msg}");
        }
    }
}
