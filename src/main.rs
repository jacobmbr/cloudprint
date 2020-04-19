use dither;
use dither::prelude::{Dither, Ditherer, Img, RGB};
use image;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = "/home/jacob/Desktop/dither/bulia/";
    let filename = "bulia2.jpeg";
    let tmpfile = "tmpfile.jpeg";

    let img = image::open(format!("{}{}", path, filename)).unwrap();
    img.resize(384, 800000, image::imageops::Gaussian)
        .to_rgb()
        .save(format!("{}{}", path, tmpfile))
        .unwrap();

    let img: Img<RGB<f64>> = Img::load(format!("{}{}", path, tmpfile)).unwrap();
    let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());

    // for d in &["sierra", "stucki", "atkinson", "burkes", "jarvis", "floyd"] {
    for d in &["sierra"] {
        let tmp = bw_img.clone();
        let output = dither_image(d, tmp);
        let pixels: Vec<u8> = output.clone().into_iter().map(|el| el.0).collect();
        let width = output.width();
        let height = output.height();
        let bitmap = bitmap_from_pixels(pixels, width, height);
        let out_str = bitmap_to_python_str(bitmap, output.width(), output.height());
        let mut file = File::create(format!("{}{}", path, "res.py")).unwrap();
        file.write_all(out_str.as_bytes()).unwrap();

        output
            .save(Path::new(&format!("{}result-{}.bmp", path, d)))
            .unwrap();
        println!("Done: {}", d)
    }
    remove_file(Path::new(&format!("{}{}", path, tmpfile))).unwrap();
}

fn bitmap_from_pixels(pixels: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let row_bytes = width / 8;
    let mut bitmap: Vec<u8> = vec![0; (row_bytes * height) as usize];
    for (i, chunk) in pixels.chunks(8).enumerate() {
        let mut chunk_byte = std::string::String::new();
        for pixel in chunk {
            match pixel {
                0 => chunk_byte.push_str("1"),
                255 => chunk_byte.push_str("0"),
                _ => panic!("derp"),
            }
        }
        let chunk_byte = u8::from_str_radix(&chunk_byte, 2).unwrap();
        bitmap[i] = chunk_byte;
    }
    bitmap
}
fn bitmap_to_python_str(bitmap: Vec<u8>, width: u32, height: u32) -> String {
    let mut array_string = String::new();
    for (i, p) in bitmap.iter().enumerate() {
        array_string.push_str(&p.to_string());
        if i != bitmap.len() - 1 {
            array_string.push_str(", ");
        }
        if i % 10 == 0 {
            array_string.push_str("\n    ");
        }
    }
    let out_str = format!(
        "width = {}\nheight = {}\ndata = [\n    {}\n]",
        width, height, array_string
    );
    out_str
}

fn dither_image(d: &str, img: Img<f64>) -> Img<RGB<u8>> {
    let ditherer = Ditherer::from_str(d).unwrap();
    let quantize = dither::create_quantize_n_bits_func(1).unwrap();
    let output = ditherer
        .dither(img, quantize)
        .convert_with(RGB::from_chroma_corrected_black_and_white);
    output
}

// for y in { 0..height } {
//     let n = y * row_bytes;
//     let mut x = 0;
//     for b in { 0..row_bytes } {
//         let mut sum = 0;
//         let mut bit = 128;
//         while bit > 0 {
//             if x >= width {
//                 break;
//             }
//             if pixels[(x * y) as usize] == 0 {
//                 sum |= bit;
//             }
//             x += 1;
//             bit >>= 1;
//         }
//         println!("{}, {}", bitmap.len(), n + b);
//         bitmap[(n + b) as usize] = sum;
//     }
// }
// // println!("{:?}", bitmap);
