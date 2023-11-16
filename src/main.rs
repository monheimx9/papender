mod args;
mod imgproc;

/// Example showcasing a generic implementation of image concatenation.
///
/// The example images are coming from https://placeholder.com/
///
/// Run from the root of the repository with:
/// cargo run --release --example concat

#[derive(Clone, Copy)]
pub struct LesFiltres {
    pub hue: Option<i32>,
    pub contrasty: Option<f32>,
    pub gray: bool,
    pub invert: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = args::config_load()?;
    let new_y: u32 = opts.resize.unwrap_or(400);
    let imgs = match &opts.flagos {
        Some(flags) => flags.iter().map(image::open).collect(),
        None => {
            println!("No images to concat");
            Ok(vec![])
        }
    }?;
    let image_last = {
        if opts.no_resize {
            image::open(&opts.input)?
        } else {
            let rescaled = imgproc::scale_image(&opts.input, new_y);
            image::open(rescaled)?
        }
    };
    let result_path = &opts.output;

    Ok(imgproc::h_concat_vec(imgs, image_last, opts.les_filtres).save(result_path)?)
}
