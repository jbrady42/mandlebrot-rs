use image::ImageBuffer;
use rayon::prelude::*;
use rug::{Complex, Float};

const MAX_ITER: u32 = 100;
const MAX_DISTANCE: u32 = 4;
const NUM_PREC: u32 = 128;

pub struct Mandel {
    samples: (usize, usize),
    scale: Float,
    start_point: Complex,
    data: Vec<Vec<i32>>,
    pub seq: u32,
}

impl Mandel {
    pub fn new(samples: (u32, u32), scale: Float, center: (Float, Float), seq: u32) -> Mandel {
        let center = Complex::with_val(NUM_PREC, (center.0, center.1));
        let samples = (samples.0 as usize, samples.1 as usize);

        let mut samps = Complex::with_val(NUM_PREC, (samples.0 as f64, samples.1 as f64));
        samps *= Float::with_val(128, 0.5);

        let start_point = center - (samps * &scale);
        let data = vec![vec![0; 1]; 1];

        println!("Start point {:?}", start_point);

        Mandel {
            samples,
            scale,
            start_point,
            seq,
            data,
        }
    }

    pub fn generate(&mut self) {
        let scale = self.scale.clone();
        let start = self.start_point.clone();
        let ycount = self.samples.1;

        self.data = (0..self.samples.0)
            .collect::<Vec<usize>>()
            .par_iter()
            .map(move |&x| Mandel::line(x, ycount, &scale, &start))
            .collect::<Vec<Vec<i32>>>();
    }

    fn line(x: usize, count: usize, scale: &Float, start: &Complex) -> Vec<i32> {
        let mut d = vec![0; count];
        for y in 0..count {
            let mut c1 = Complex::with_val(NUM_PREC, (x as f64, y as f64));
            c1 *= scale;
            c1 += start;
            let res = Mandel::diverge_count(c1, MAX_ITER, MAX_DISTANCE);
            d[y] = res;
        }
        d
    }

    pub fn image_path(seq: u32) -> String {
        format!("img/fractal-{:09}.png", seq)
    }

    pub fn render_image(&self) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut imgbuf = image::ImageBuffer::new(self.samples.0 as u32, self.samples.1 as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let div = self.data[x as usize][self.samples.1 - 1 - y as usize];
            let clr = if div < 0 {
                image::Rgb([0, 0, 0])
            } else {
                Mandel::get_color(div)
            };
            *pixel = clr;
        }
        imgbuf
    }

    pub fn draw_image(&self) {
        let imgbuf = self.render_image();
        imgbuf.save(Mandel::image_path(self.seq)).unwrap();
    }

    fn get_color(iter: i32) -> image::Rgb<u8> {
        return Mandel::color_RGB_space(iter);
        let pct = iter as f64 / MAX_ITER as f64;
        let r = (255.0 * pct) as u8;
        image::Rgb([r, 0, 0])
    }

    fn color_RGB_space(iter: i32) -> image::Rgb<u8> {
        let base = 255;
        let r = (iter % base) as u8;
        let mut c = iter / base;
        let g = (c % base) as u8;
        c /= base;
        let b = (c % base) as u8;
        image::Rgb([r, g, b])
    }

    fn diverge_count(c1: Complex, max_iter: u32, max_dist: u32) -> i32 {
        let mut c = Complex::with_val(NUM_PREC, (0.0, 0.0));
        for i in 0..max_iter {
            c.square_mut();
            c += &c1;
            let dist = c.clone().norm();
            if dist > max_dist as f64 {
                return i as i32;
            }
        }
        -1
    }

    pub fn dump(&self) {
        for a in &self.data {
            print!("{:?}", a);
            println!("");
        }
    }
}
