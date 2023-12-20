mod pool;
mod vectors;

use pool::thread_pool;
use std::{fs, time::Instant};

use crate::vectors::Vectors;

fn main() {
    let timer = Instant::now();
    thread_pool(
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
