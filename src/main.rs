use num::complex::Complex;
use std::fs::create_dir_all;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use mandlebrot::Mandle;

fn set_zoom(sc: f64) -> f64 {
    sc / 150.0
}

fn main() {
    // Resolution
    let samples = (100, 100);

    let mut scale = 0.01;

    let x_center = 0.001643721971153;
    let y_center = -0.822467633298876;

    let center = Complex::new(x_center, y_center);

    let mut zoom_step = set_zoom(scale);

    let max_frames = 31;

    // Create output dir
    create_dir_all("img").unwrap();

    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel::<bool>();

    let mut frame = 0;
    while frame < max_frames {
        if scale < 0.0 {
            println!("Scale going negative at frame {}", frame);
            break;
        }

        let tx = tx.clone();
        pool.execute(move || {
            println!("Render frame {}", frame);
            let mut man = Mandle::new(samples, scale, center, frame);
            man.generate();
            man.draw_image();
            tx.send(true).expect("done channel open");
        });

        if frame % 25 == 0 {
            zoom_step = set_zoom(scale);
        }
        println!("frame {} scale {}", frame, scale);
        scale -= zoom_step;
        frame += 1;
    }

    // Wait for work to complete
    rx.iter().take(frame as usize).for_each(drop);
}
