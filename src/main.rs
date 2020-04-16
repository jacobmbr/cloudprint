use dither;
use dither::prelude::{Dither, Ditherer, Img, RGB};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let img: Img<RGB<f64>> = Img::load("/home/jacob/Desktop/anja.png").unwrap();
    let bw_img = img.convert_with(|rgb| rgb.to_chroma_corrected_black_and_white());
    for d in &["sierra", "stucki", "atkinson", "burkes", "jarvis", "floyd"] {
        let tmp = bw_img.clone();
        let output = do_dither(d, tmp);
        let p = &format!("/home/jacob/Desktop/otto-{}.bmp", d);
        let output_path = Path::new(p);
        output.save(output_path).unwrap();
        println!("{:?}", output_path);
    }
}

fn do_dither(d: &str, img: Img<f64>) -> Img<RGB<u8>> {
    let ddd = Ditherer::from_str(d).unwrap();
    let quantize = dither::create_quantize_n_bits_func(1).unwrap();
    let output = ddd
        .dither(img, quantize)
        .convert_with(RGB::from_chroma_corrected_black_and_white);
    output
}
