use rug::{Complex, Float};
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::mpsc::channel;
use structopt::StructOpt;
use threadpool::ThreadPool;

use mandelbrot::Mandel;

#[derive(StructOpt)]
#[structopt(name = "mandlebrot", about = "Generate Mandlebrot zoom images")]
struct Opt {
    #[structopt(
        short = "f",
        long = "frames",
        help = "Set number of frames",
        default_value = "1"
    )]
    frames: u32,

    #[structopt(
        short = "w",
        long = "width",
        help = "Set width of image",
        default_value = "80"
    )]
    width: u32,

    #[structopt(
        short = "h",
        long = "height",
        help = "Set height of image",
        default_value = "80"
    )]
    height: u32,

    #[structopt(
        short = "x",
        long = "center-x",
        help = "Set center-x of image",
        default_value = "0.001643721971153"
    )]
    center_x: f64,

    #[structopt(
        short = "y",
        long = "center-y",
        help = "Set center-y of image",
        default_value = "-0.822467633298876"
    )]
    center_y: f64,

    #[structopt(
        short = "s",
        long = "scale",
        help = "Set start scale",
        default_value = "0.01"
    )]
    start_scale: String,
}

fn set_zoom(sc: &Float) -> Float {
    Float::with_val(128, sc / 150.0)
}

fn main() {
    let opt = Opt::from_args();

    // Resolution
    let samples = (opt.width, opt.height);
    let center = (opt.center_x, opt.center_y);

    let mut scale = Float::from_str(&opt.start_scale, 128).unwrap();

    let mut zoom_step = set_zoom(&scale);

    // Create output dir
    create_dir_all("img").unwrap();

    let n_workers = 2;
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel::<bool>();

    let mut frame = 0;
    let mut skip = 0;
    while frame < opt.frames {
        if scale < 0.0 {
            println!("Scale going negative at frame {}", frame);
            break;
        }

        //Skip frames that have already been generated
        let sc = scale.clone();
        if !Path::new(&Mandel::image_path(frame)).exists() {
            let tx = tx.clone();
            pool.execute(move || {
                println!("Render frame {}", frame);
                let mut man = Mandel::new(samples, sc, center, frame);
                man.generate();
                man.draw_image();
                tx.send(true).expect("done channel open");
            });
        } else {
            skip += 1;
        }

        if frame % 25 == 0 {
            zoom_step = set_zoom(&scale);
        }
        println!("frame {} scale {}", frame, scale);
        scale -= &zoom_step;
        frame += 1;
    }

    // Wait for work to complete
    rx.iter().take((frame - skip) as usize).for_each(drop);
}
