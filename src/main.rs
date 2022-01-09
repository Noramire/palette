use image::{self, GenericImageView, RgbImage, DynamicImage};
use palette::{MedianCut, Quantization};
use clap::Parser;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long, help = "Source image (e.g. --src assets/img.jpg)")]
    src: String,

    #[clap(short, long, default_value = "8", help = "Number of shades you want to get")]
    colors: usize,

    #[clap(long = "no-export", help = "Doesn't export palette web page")]
    no_export: bool,

    #[clap(short, long, default_value = "", help = "Export an image (e.g --out assets/new_img.jpg)")]
    out: String,

    #[clap(long, help = "Doesn't print an array of palette's shades")]
    silent: bool
}

pub fn export_html(colors: &Vec<String>, path_image: String) {
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
        .map(|&pixel| [pixel[0], pixel[1], pixel[2]])
        .collect();

    let mut quantization = MedianCut::new(&space, args.colors);
    quantization.build();
    let palette = quantization.get_palette();

    if !args.silent {
        println!("{:?}", palette);
    }

    if !args.no_export {
        export_html(&palette, args.src.clone());
    }

    if !args.out.is_empty() {
        export_quantization(width, height, quantization.to_buffer(), args.out.clone());
    }
}
