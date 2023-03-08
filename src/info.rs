use std::path::PathBuf;
use structopt::StructOpt;
use image::{GenericImageView};

#[derive(Debug, StructOpt)]
pub struct Command {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub(crate) input: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,

    #[structopt(
    short,
    long = "--long-option",
    required_if("out", "file"),
    parse(from_os_str)
    )]
    long_option: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Debug, StructOpt)]
enum Cmd {
    #[structopt(
    about = "get image's information"
    )]
    Info {
        path: String,
    },
    #[structopt(
    about = "get image's dimensions"
    )]
    Dimensions {
        file_path: String,
    },
}

impl Command {
    pub(crate) fn info(self) {
        let path = PathBuf::from(self.file);
        let img = image::open(path).unwrap();

        // The color method returns the image's `ColorType`.
        println!("{:?}", img.color());

        // Write the contents of this image to the Writer in PNG format.
        img.save("../test/test.jpg").unwrap();
    }
}

// pub fn info(file_path: String) {
//     let path = PathBuf::from(file_path);
//     let img = image::open(path).unwrap();
//
//     // The color method returns the image's `ColorType`.
//     println!("{:?}", img.color());
//
//     // Write the contents of this image to the Writer in PNG format.
//     img.save("../test/test.jpg").unwrap();
// }

pub fn test(file_path: String) {
    println!("dimensions {:?}", file_path)
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