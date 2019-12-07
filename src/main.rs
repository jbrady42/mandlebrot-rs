use num::complex::Complex;

const MAX_ITER: u32 = 10;
const MAX_DISTANCE: u32 = 4;

fn main() {
    // Resolution
    let samples_x = 800;
    let samples_y = 800;

    let scale = 0.003;
    //Starting point
    // 3rd quadrant, bottom left
    let x_start = -2.2;
    let y_start = -1.0;

    let mut hist = vec![vec![0; samples_y]; samples_x];

    // Sample space
    for x in 0..samples_x {
        for y in 0..samples_y {
            let x0 = x as f64 * scale + x_start;
            let y0 = y as f64 * scale + y_start;
            let res = mandle_diverge(Complex::new(x0, y0));
            hist[x][y] = res;
        }
    }
    write_image(samples_x, samples_y, &hist);
}

fn mandle_diverge(c1: Complex<f64>) -> i32 {
    let mut c: Complex<f64> = num::complex::Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        c = c * c + c1;
        if c.norm_sqr() > MAX_DISTANCE as f64 {
            return i as i32;
        }
    }
    -1
}

fn write_image(xc: usize, yc: usize, data: &Vec<Vec<i32>>) {
    let mut imgbuf = image::ImageBuffer::new(xc as u32, yc as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let div = data[x as usize][yc - 1 - y as usize];
        let clr = if div < 0 {
            image::Rgb([0, 0, 0])
        } else {
            image::Rgb([200, 0, 0])
        };
        *pixel = clr;
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

fn display_term(data: &Vec<Vec<i32>>) {
    // This displays the graph translated with x and y swapped
    for a in data.iter() {
        for b in a {
            let out_char = if *b < 0 {
                String::from("D")
            } else {
                b.to_string()
            };
            print!("{}", out_char);
        }
        println!("");
    }
}
