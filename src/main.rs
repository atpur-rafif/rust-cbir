#![allow(unused)]

mod pool;
mod vectors;

use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    vec,
};

use image::GenericImageView;
use rayon::prelude::*;

fn main() {
    let all_timer = Instant::now();

    let dir = std::fs::read_dir("./dataset").unwrap();
    let mut threads = vec![] as Vec<JoinHandle<()>>;

    let res = dir
        .map(|file| file.unwrap().path().display().to_string())
        .collect::<Vec<String>>()
        .par_iter()
        .map(|img_path| {
            let now = Instant::now();

            let img = image::open(&img_path).unwrap();
            let res = vectors::Vectors::get_hsv_feature_vector(&img);

            let elapsed = now.elapsed();
            // println!("Processed Image in: {:.2?} {}", now.elapsed(), img_path);
            ()
        })
        .collect::<Vec<()>>();

    println!("Total time elapsed: {:.2?}", all_timer.elapsed());
}
