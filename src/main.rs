extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate rsglare;
extern crate image;

use clap::{Arg, App};
use std::fs::{File, OpenOptions};
use rsglare::scene::*;
use image::ImageFormat;

fn main() {
    let app = App::new("rsglare")
        .version("0.1")
        .author("hossein dindar <hosseind2017@gmail.com>")
        .arg(Arg::with_name("scene")
            .help("Sets the scene file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("image")
            .help("Sets the output image file")
            .required(true)
            .index(2));
    let matches = app.get_matches();

    let scene_path = matches.value_of("scene").unwrap();
    let scene_file = File::open(scene_path).expect("File not found");

    let image_path = matches.value_of("image").unwrap();

    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let block = rsglare::ViewBlock {
        x: 0,
        y: 0,
        width: scene.width,
        height: scene.height,
    };

    let image = rsglare::render(&block, &scene);

    let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}