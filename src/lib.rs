use num::complex::Complex;

const MAX_ITER: u32 = 255;
const MAX_DISTANCE: u32 = 4;

pub struct Mandle {
    samples_x: usize,
    samples_y: usize,
    scale: f64,
    start_x: f64,
    start_y: f64,
    hist: Vec<Vec<i32>>,
    seq: u32,
}

impl Mandle {
    pub fn new(scale_factor: f64, center_x: f64, center_y: f64, seq: u32) -> Mandle {
        // Resolution
        let samples_x = 800;
        let samples_y = 800;

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

    pub fn generate(&mut self) {
        for x in 0..self.samples_x {
            for y in 0..self.samples_y {
                let x0 = x as f64 * self.scale + self.start_x;
                let y0 = y as f64 * self.scale + self.start_y;
                let res = Mandle::diverge_count(Complex::new(x0, y0), MAX_ITER, MAX_DISTANCE);
                self.hist[x][y] = res;
            }
        }
    }

    pub fn draw_image(&self) {
        let mut imgbuf = image::ImageBuffer::new(self.samples_x as u32, self.samples_y as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let div = self.hist[x as usize][self.samples_y - 1 - y as usize];
            let clr = if div < 0 {
                image::Rgb([0, 0, 0])
            } else {
                Mandle::get_color(div)
            };
            *pixel = clr;
        }
        // Save the image as “fractal.png”, the format is deduced from the path
        let n = format!("img/fractal-{:09}.png", self.seq);
        imgbuf.save(n).unwrap();
    }

    fn get_color(iter: i32) -> image::Rgb<u8> {
        let pct = iter as f64 / MAX_ITER as f64;
        let r = (255.0 * pct) as u8;
        image::Rgb([r, 0, 0])
    }

    fn diverge_count(c1: Complex<f64>, max_iter: u32, max_dist: u32) -> i32 {
        let mut c: Complex<f64> = num::complex::Complex::new(0.0, 0.0);
        for i in 0..max_iter {
            c = c * c + c1;
            if c.norm_sqr() > max_dist as f64 {
                return i as i32;
            }
        }
        -1
    }
}
