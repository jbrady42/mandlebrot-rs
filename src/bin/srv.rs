use actix_web::get;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use rug::Float;
use serde::Deserialize;
use std::collections::HashMap;

use image_base64;

use mandelbrot::{color_palette, Mandel};

#[derive(Deserialize, Debug)]
struct Info {
    cx: String,
    cy: String,
    scale: String,
    width: u32,
    height: u32,
}

struct AppState {
    palette: HashMap<i32, image::Rgb<u8>>,
}

#[get("/")]
fn serve_mandelbrot(info: web::Query<Info>, state: web::Data<AppState>) -> impl Responder {
    println!("Info {:?}", info);
    let dims = (info.width, info.height);

    let scale = Float::from_str(&info.scale, 128).unwrap();
    let cx = Float::from_str(&info.cx, 128).unwrap();
    let cy = Float::from_str(&info.cy, 128).unwrap();
    let center = (cx, cy);

    let mut man = Mandel::new(dims, scale, center, 0);
    man.generate();
    man.draw_image(&state.palette);

    // Get base64
    let base64 = image_base64::to_base64(&Mandel::image_path(man.seq));
    HttpResponse::Ok().body(base64)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                palette: color_palette(),
            })
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .service(serve_mandelbrot)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
