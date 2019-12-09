use image::ImageBuffer;
use num::complex::Complex;

const MAX_ITER: u32 = 255;
const MAX_DISTANCE: u32 = 4;

pub struct Mandel {
    samples: (usize, usize),
    scale: f64,
    start_point: Complex<f64>,
    data: Vec<Vec<i32>>,
    pub seq: u32,
}

impl Mandel {
    pub fn new(samples: (u32, u32), scale_factor: f64, center: (f64, f64), seq: u32) -> Mandel {
        let scale = if scale_factor < 0.0 {
            0.0000000000000000000001
        } else {
            scale_factor
        };

        let center = Complex::new(center.0, center.1);
        let samples = (samples.0 as usize, samples.1 as usize);

        let samps = Complex::new(samples.0 as f64, samples.1 as f64);
        let start_point = center - samps.scale(scale * 0.5);
        let data = vec![vec![0; samples.1]; samples.0];

        Mandel {
            samples,
            scale,
            start_point,
            seq,
            data,
        }
    }

    pub fn generate(&mut self) {
        for x in 0..self.samples.0 {
            for y in 0..self.samples.1 {
                let x0 = x as f64 * self.scale + self.start_point.re;
                let y0 = y as f64 * self.scale + self.start_point.im;
                let res = Mandel::diverge_count(Complex::new(x0, y0), MAX_ITER, MAX_DISTANCE);
                self.data[x][y] = res;
            }
        }
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
        let pct = iter as f64 / MAX_ITER as f64;
        let r = (255.0 * pct) as u8;
        image::Rgb([r, 0, 0])
    }

    fn diverge_count(c1: Complex<f64>, max_iter: u32, max_dist: u32) -> i32 {
        let mut c: Complex<f64> = Complex::new(0.0, 0.0);
        for i in 0..max_iter {
            c = c * c + c1;
            if c.norm_sqr() > max_dist as f64 {
                return i as i32;
            }
        }
        -1
    }
}
