use image::{self, GenericImageView, RgbImage, DynamicImage};
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use palette::traits::Quantization;
use palette::mediancut::MedianCut;
use palette::meet::Meet;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, help = "Source image (e.g. assets/img.jpg)")]
    src: String,

    #[clap(short, long, default_value = "8", help = "Number of shades you want to get")]
    colors: usize,

    #[clap(long = "no-export", help = "Don't export a web page")]
    no_export: bool,

    #[clap(short, long, default_value = "", help = "Export an image (e.g assets/new_img.jpg)")]
    out: String,

    #[clap(long, help = "Don't print an array of palette's shades")]
    silent: bool,

    #[clap(long, default_value = "0", help = "Quantization method
0. Median Cut Algorithm
1. Common Pixels with a dictionary
    ")]
    method: usize
}

pub fn export_html(colors: &[String], path_image: String) {
    let path = Path::new(path_image.as_str());
    let image_name = path.file_name().unwrap().to_str().unwrap();
    
    let index_html = String::from(include_str!("assets\\index.html"));
    let formatted = index_html
    .replace("{% image %}", image_name)
    .replace("{% colors %}", format!("{:?}", colors).as_str());
    
    let path_html = match path.parent() {
        Some(n) => n.join("index.html"),
        None => Path::new("index.html").to_path_buf()
    };

    let mut file = File::create(path_html).unwrap();
    file.write_all(formatted.as_bytes()).unwrap();
}

pub fn export_quantization(width: u32, height: u32, buf: Vec<u8>, path_out: String) {
    let buffer = RgbImage::from_raw(width, height, buf).unwrap();
    buffer.save(path_out).unwrap();
}

fn main() {
    let args = Args::parse();

    let img: DynamicImage = image::open(args.src.clone())
        .unwrap();

    let width = img.width();
    let height = img.height();
    let buffer = img.to_rgb8();

    let space: Vec<[u8; 3]> = buffer.pixels()
        .map(|pixel| pixel.0)
        .collect();

    let quantization: Option<Box<dyn Quantization>> = match args.method {
        0 => Some(Box::new(MedianCut::new(space, args.colors))),
        1 => Some(Box::new(Meet::new(space, args.colors))),
        _ => None
    };

    let mut quantization = quantization.expect(
        format!("\"{}\" is a wrong method!", args.method).as_str()
    );

    quantization.build();
    let palette = quantization.get_palette();

    if !args.silent {
        println!("{:?}", palette);
    }

    if !args.no_export {
        export_html(&palette, args.src.clone());
    }

    if !args.out.is_empty() {
        export_quantization(width, height, quantization.to_buffer(), args.out);
    }
}
