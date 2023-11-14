mod args;
mod imgproc;

/// Example showcasing a generic implementation of image concatenation.
///
/// The example images are coming from https://placeholder.com/
///
/// Run from the root of the repository with:
/// cargo run --release --example concat
fn main() {
    let opts = args::config_load();
    let mut imgs = Vec::new();
    let new_y: u32 = opts.resize.unwrap();
    match opts.flagos {
        Some(flags) => {
            for img in flags {
                imgs.push(image::open(&img).unwrap());
            }
        }
        None => println!("No images to concat"),
    }
    if opts.no_resize {
        imgs.push(image::open(opts.input.unwrap()).unwrap());
    } else {
        let rescaled = imgproc::scale_image(opts.input.unwrap(), new_y);
        imgs.push(image::open(rescaled).unwrap());
    }
    let result_path = &opts.output.unwrap();

    imgproc::h_concat_vec(imgs).save(result_path).unwrap();
}
