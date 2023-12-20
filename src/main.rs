#![allow(unused)]
use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    vec,
};

use image::{DynamicImage, GenericImageView, Rgba};

struct Vectors {
    size: usize,
    dimension: usize,
    buffer: Vec<f64>,
}

fn bin_h(h: f64) -> usize {
    let mut r = 1;

    if h > 25.0 {
        r = 2
    }

    if h > 40.0 {
        r = 3
    }

    if h > 120.0 {
        r = 4
    }

    if h > 190.0 {
        r = 5
    }

    if h > 270.0 {
        r = 6
    }
    if h > 295.0 {
        r = 7
    }
    if h > 315.0 {
        r = 0
    }

    return r;
}

fn bin_s(s: f64) -> usize {
    let mut r = 0;

    if s > 0.2 {
        r = 1
    }
    if s > 0.7 {
        r = 2
    }

    r
}

fn bin_v(v: f64) -> usize {
    let mut r = 0;

    if v > 0.2 {
        r = 1
    }
    if v > 0.7 {
        r = 2
    }

    r
}

impl Vectors {
    fn new(size: usize, dimension: usize) -> Vectors {
        Vectors {
            size,
            dimension,
            buffer: vec![0.0; size * dimension],
        }
    }

    fn get_average_cosine_similarity(a: &Vectors, b: &Vectors) -> f64 {
        let size = a.size;
        let dimension = a.dimension;
        if size != b.size || dimension != b.dimension {
            panic!("Can't operate on different size or dimension of vectors");
        }

        let mut res = 0.0;
        for i in 0..size {
            let mut sum = 0.0;
            for j in 0..dimension {
                let idx = i * dimension + j;
                let va = a.buffer[idx];
                let vb = b.buffer[idx];
                let diff = va - vb;
                sum += diff * diff;
            }
            res += f64::sqrt(sum);
        }
        res / size as f64
    }

    fn pixel_to_bin(p: Rgba<u8>) -> usize {
        let [r, g, b, _] = p.0;

        let r = (r as f64) / 255.0;
        let g = (g as f64) / 255.0;
        let b = (b as f64) / 255.0;

        let cmax = f64::max(f64::max(r, g), b);
        let cmin = f64::min(f64::min(r, g), b);
        let delta = cmax - cmin;

        let h = if cmax == r {
            (60.0 * ((g - b) / delta) + 360.0) % 360.0
        } else if cmax == g {
            (60.0 * ((b - r) / delta) + 360.0) % 360.0
        } else if cmax == b {
            (60.0 * ((r - g) / delta) + 360.0) % 360.0
        } else {
            0.0
        };

        let s = if cmax == 0.0 { 0.0 } else { delta / cmax };
        let v = cmax;

        24 * bin_v(v) + 8 * bin_s(s) + bin_h(h)
    }

    fn get_hsv_feature_vector(img: &DynamicImage) -> Vectors {
        let mut vecs = Vectors::new(1, 72);

        for i in 0..img.height() {
            for j in 0..img.width() {
                let bin = Self::pixel_to_bin(img.get_pixel(j, i));
                vecs.buffer[bin] += 1.0;
            }
        }

        vecs
    }
}

fn main() {
    let all_timer = Instant::now();

    let io_timer = Instant::now();
    let dir = std::fs::read_dir("./dataset").unwrap();
    let mut imgs = Vec::new() as Vec<(String, DynamicImage)>;
    for img in dir {
        let now = Instant::now();
        let img_path = img.unwrap().path().display().to_string();
        let img = image::open(&img_path).unwrap();
        println!("Loaded Image: {:.2?} {}", now.elapsed(), img_path);
        imgs.push((img_path, img));
    }
    println!("All io: {:.2?}", io_timer.elapsed());

    let process_timer = Instant::now();
    let mut threads = vec![] as Vec<JoinHandle<()>>;
    for (img_path, img) in imgs {
        threads.push(thread::spawn(move || {
            let now = Instant::now();
            Vectors::get_hsv_feature_vector(&img);
            let elapsed = now.elapsed();
            println!("Processed Image: {:.2?} {}", now.elapsed(), img_path);
        }));
    }

    let res = Vec::new() as Vec<(String, Vectors)>;
    for thread in threads {
        let a = thread.join().unwrap();
    }
    println!("All process: {:.2?}", process_timer.elapsed());

    println!("Total time elapsed: {:.2?}", all_timer.elapsed());
}
