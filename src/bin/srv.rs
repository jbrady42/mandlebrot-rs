use actix_web::get;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use rug::{Complex, Float};
use serde::Deserialize;

use image_base64;

use mandelbrot::Mandel;

#[derive(Deserialize, Debug)]
struct Info {
    cx: f64,
    cy: f64,
    scale: String,
    width: u32,
    height: u32,
}

#[get("/")]
fn serve_mandelbrot(info: web::Query<Info>) -> impl Responder {
    println!("Info {:?}", info);
    let center = (info.cx, info.cy);
    let dims = (info.width, info.height);

    let scale = Float::from_str(&info.scale, 128).unwrap();

    let mut man = Mandel::new(dims, scale, center, 0);
    man.generate();
    man.draw_image();

    // Get base64
    let base64 = image_base64::to_base64(&Mandel::image_path(man.seq));
    HttpResponse::Ok().body(base64)
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "*"))
            .service(serve_mandelbrot)
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}
