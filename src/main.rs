struct Vectors {
    size: usize,
    dimension: usize,
    buffer: Vec<f64>,
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
        res /= size as f64;

        res
    }
}

fn main() {
    // let img = image::open("./dataset/0.jpg").unwrap();
    // println!("{} {}", img.width(), img.height());
    let a = Vectors::new(3, 10);
    let b = Vectors::new(3, 10);
    dbg!(Vectors::get_average_cosine_similarity(&a, &b));
}
