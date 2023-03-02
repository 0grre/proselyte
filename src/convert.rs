use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use image::EncodableLayout;
use webp::{Encoder, WebPMemory};

pub async fn to_webp(file_path: String) {
    let p = file_path.as_str();
    let path_buff = PathBuf::from(p);
    let img = image::open(path_buff).unwrap();

    let encoder: Encoder = Encoder::from_image(&img).unwrap();

    let encoded_webp: WebPMemory = encoder.encode(65f32);

    let path: &Path = Path::new(p);
    let filename_original_image = path.file_stem().unwrap().to_str().unwrap();

    let webp_image_path = format!(
        "{}.webp",
        filename_original_image
    );

    let mut webp_image = File::create(webp_image_path.to_string()).unwrap();
    webp_image.write_all(encoded_webp.as_bytes()).expect("TODO: panic message");
}
