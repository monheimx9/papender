mod args;
mod imgproc;
use anyhow::Result;

/// Example showcasing a generic implementation of image concatenation.
///
/// The example images are coming from https://placeholder.com/
///
/// Run from the root of the repository with:
/// cargo run --release --example concat

#[derive(Clone)]
pub struct LesFiltres {
    pub hue: Option<i32>,
    pub contrasty: Option<f32>,
    pub gray: bool,
    pub invert: bool,
}

fn main() -> Result<()> {
    let opts = &args::config_load().unwrap().clone();
    let mut imgs = Vec::new();
    let new_y: &u32 = &opts.resize.unwrap();
    match &opts.flagos {
        Some(flags) => {
            for img in flags {
                imgs.push(image::open(img)?);
            }
        }
        None => println!("No images to concat"),
    }
    let image_last = {
        if opts.no_resize {
            image::open(opts.input.clone().unwrap())?
        } else {
            let rescaled = imgproc::scale_image(opts.input.clone().unwrap(), new_y)?;
            image::open(rescaled)?
        }
    };
    let result_path = &opts.output.clone().unwrap();

    imgproc::h_concat_vec(imgs, image_last, opts.les_filtres.clone())
        .save(result_path)
        .unwrap();
    Ok(())
}
