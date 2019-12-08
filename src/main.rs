use num::complex::Complex;
use std::fs::create_dir_all;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

const MAX_ITER: u32 = 10;
const MAX_DISTANCE: u32 = 4;

struct Mandle {
    samples_x: usize,
    samples_y: usize,
    scale: f64,
    start_x: f64,
    start_y: f64,
    hist: Vec<Vec<i32>>,
    seq: u32,
}

impl Mandle {
    fn new(scale_factor: f64, center_x: f64, center_y: f64, seq: u32) -> Mandle {
        // Resolution
        let samples_x = 400;
        let samples_y = 400;

        let scale = if scale_factor < 0.0 {
            0.0000000000000000000001
        } else {
            scale_factor
        };

        let start_x = center_x - (samples_x / 2) as f64 * scale;
        let start_y = center_y - (samples_y / 2) as f64 * scale;

        Mandle {
            samples_x,
            samples_y,
            scale,
            start_x,
            start_y,
            seq,
            hist: vec![vec![0; samples_y]; samples_x],
        }
    }

    fn generate(&mut self) {
        for x in 0..self.samples_x {
            for y in 0..self.samples_y {
                let x0 = x as f64 * self.scale + self.start_x;
                let y0 = y as f64 * self.scale + self.start_y;
                let res = mandle_diverge(Complex::new(x0, y0), MAX_ITER, MAX_DISTANCE);
                self.hist[x][y] = res;
            }
        }
    }

    fn draw_image(&self) {
        let mut imgbuf = image::ImageBuffer::new(self.samples_x as u32, self.samples_y as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let div = self.hist[x as usize][self.samples_y - 1 - y as usize];
            let clr = if div < 0 {
                image::Rgb([0, 0, 0])
            } else {
                image::Rgb([200, 0, 0])
            };
            *pixel = clr;
        }
        // Save the image as “fractal.png”, the format is deduced from the path
        let n = format!("img/fractal-{:09}.png", self.seq);
        imgbuf.save(n).unwrap();
    }
}

fn main() {
    let mut scale = 0.0005;

    let x_center = -1.77;
    let y_center = 0.06;

    let zoom_step = 0.000005;

    let max_frames = 500;

    // Create output dir
    create_dir_all("img").unwrap();

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel::<bool>();

    let mut frames = 0;

    for i in 0..max_frames {
        if scale < 0.0 {
            println!("Scale going negativeat frame {}", i);
            break;
        }

        let tx = tx.clone();

        pool.execute(move || {
            println!("frame {} scale {}", i, scale);
            let mut man = Mandle::new(scale, x_center, y_center, i);
            man.generate();
            man.draw_image();
            tx.send(true).expect("done channel open");
        });

        frames += 1;
        scale -= zoom_step;
    }

    // Wait for work to complete
    rx.iter().take(frames).collect::<Vec<bool>>();
}

fn mandle_diverge(c1: Complex<f64>, max_iter: u32, max_dist: u32) -> i32 {
    let mut c: Complex<f64> = num::complex::Complex::new(0.0, 0.0);
    for i in 0..max_iter {
        c = c * c + c1;
        if c.norm_sqr() > max_dist as f64 {
            return i as i32;
        }
    }
    -1
}
