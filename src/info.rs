use std::path::PathBuf;
use image::{GenericImageView};

pub async fn info(file_path: String) {
    let path = PathBuf::from(file_path);
    let img = image::open(path).unwrap();

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("../test/test.png").unwrap();
}

pub fn dimensions(file_path: String) {
    let path = PathBuf::from(file_path);
    let img = image::open(path).unwrap();

    // let format = ImageFormat::from_path(path);
    // let format_output = ImageOutputFormat::from(format);

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions())
    // println!("extension =  {:?}", format_output)
}