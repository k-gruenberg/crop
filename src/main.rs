use std::ffi::OsStr;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};
use clap::{Parser, ArgGroup};
use image::{GenericImageView, ImageError, imageops};

/// Simple command line tool to crop one or multiple images at once
#[derive(Parser, Debug)]
#[clap(about, version, author)]
#[clap(group(
    ArgGroup::new("w")
    .required(true)
    .args(&["width", "x2"]),
))]
#[clap(group(
    ArgGroup::new("h")
    .required(true)
    .args(&["height", "y2"]),
))]
struct Args {
    /// Path to an image file or to a directory containing image files
    #[clap(parse(from_os_str))]
    path: PathBuf,

    /// X position of the top-left crop point
    #[clap(long)]
    x1: f64,

    /// Y position of the top-left crop point
    #[clap(long)]
    y1: f64,

    /// X position of the bottom-right crop point
    #[clap(long)]
    x2: Option<f64>,

    /// Y position of the bottom-right crop point
    #[clap(long)]
    y2: Option<f64>,

    /// Width of the cropped section
    #[clap(long)]
    width: Option<f64>,

    /// Height of the cropped section
    #[clap(long)]
    height: Option<f64>,

    /// With this flag, the x1, y1, x2, y2, width, height arguments are treated as relative values between 0.0 and 1.0 instead of absolute pixel values.
    #[clap(long)]
    relative: bool,

    /// Deactivates output to stdout, errors are still printed to stderr
    #[clap(long)]
    silent: bool,
}

/*
Possible future features/flags:
  --recursive (also crop images in subfolders)
  --nopng, --nojpg, ... (do not crop images of certain file types)
  --fliphorizontal
  --flipvertical
  --rotate90left
  --rotate90right
  --rotate180
  --grayscale, --invertcolors, --blur, --brighten, ... (https://docs.rs/crate/image/0.23.14)
  --keeporiginals
  --regexfilter (only crop image files whose name matches a certain regex pattern)
  --destination (put the cropped images into a separate folder instead of replacing them)
  --minheight, --minwidth (only crop images with a certain minimum size, in pixels)
  --maxheight, --maxwidth (only crop images up to a certain maximum size, in pixels)
  --shrink (shrink image instead of cropping it; x1/y2 attributes are ignored when width/height attributes are given)
  --outformat (also convert all images into a certain format in addition to cropping them)
  --nocrop (do not crop; only useful when an argument for one of the other tools is given)
 */

/// For a full list see `https://github.com/image-rs/image/blob/master/README.md`.
/// However, according to `https://docs.rs/image/0.23.14/image/fn.save_buffer.html`
/// "Currently only jpeg, png, ico, pnm, bmp and tiff files are supported"
/// for the `save_buffer` function!
static SUPPORTED_IMAGE_FORMATS: [&str; 8] =
    ["JPEG", "JPG", "PNG", "ICO", "PNM", "BMP", "TIFF", "TIF"];


fn main() {
    let args = Args::parse();

    let images: Vec<PathBuf>;
    if args.path.is_file() {
        images = vec![args.path];
    } else if args.path.is_dir() {
        images = args.path.read_dir().unwrap()
            .filter(|dir_entry: &io::Result<DirEntry>| dir_entry.is_ok())
            .map(|dir_entry: io::Result<DirEntry>| dir_entry.unwrap().path())
            .filter(|path: &PathBuf| path.extension().map(|extension: &OsStr| SUPPORTED_IMAGE_FORMATS.contains(&extension.to_str().unwrap())).unwrap_or(false))
            .collect();
    } else {
        eprintln!("Path {} does not exist!", args.path.display());
        return;
    }

    for image in images {
        let path_string = image.display().to_string();
        if let Err(err) = crop_image(&image, args.x1, args.y1,
                                 args.width.unwrap_or_else(|| args.x2.unwrap() - args.x1),
                                 args.height.unwrap_or_else(|| args.y2.unwrap() - args.y1),
                                 args.relative) {
            eprintln!("Cropping image {} failed: {}", path_string, err);
        } else if !args.silent {
            println!("Cropped {}", path_string);
        };
    }
}

fn crop_image(image_file: &Path, x1: f64, y1: f64, width: f64, height: f64, relative: bool) -> Result<(), ImageError> {
    let mut img = image::open(image_file)?;

    let (horizontal_multiplier, vertical_multiplier) = if relative {
        img.dimensions()
    } else {
        (1, 1)
    };
    let (horizontal_multiplier, vertical_multiplier) = (horizontal_multiplier as f64, vertical_multiplier as f64);

    let cropped_img = imageops::crop(&mut img,
                                     (x1 * horizontal_multiplier) as u32, (y1 * vertical_multiplier) as u32,
                                     (width * horizontal_multiplier) as u32, (height * vertical_multiplier) as u32);
    cropped_img.to_image().save(image_file)?;

    return Result::Ok(());
}