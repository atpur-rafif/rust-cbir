#![allow(unused)]

mod pool;
mod vectors;

use std::{
    fs,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    vec,
};

use image::GenericImageView;
use pool::thread_pool;
use rayon::prelude::*;

use crate::vectors::Vectors;

fn main() {
    let timer = Instant::now();
    let mut a = vec![1, 2, 3];
    let k = thread_pool(
        fs::read_dir("./dataset")
            .unwrap()
            .map(|f| f.unwrap().path().display().to_string())
            .collect::<Vec<String>>(),
        |x| {
            let img = image::open(&x).unwrap();
            Vectors::get_hsv_feature_vector(&img)
        },
    );
    println!("{:.2?}", timer.elapsed());
}
