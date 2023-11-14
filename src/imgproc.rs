use image::imageops::FilterType;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, Pixel, Primitive};
use std::fs::File;

pub fn h_concat_vec<I, P, S>(images: Vec<I>) -> ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let img_width_out: u32 = images.iter().map(|im| im.width()).sum();
    let img_height_out: u32 = images.iter().map(|im| im.height()).max().unwrap_or(0);

    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_width = 0;

    for img in images {
        let y = img_height_out - img.height();
        imgbuf.copy_from(&img, accumulated_width, y).unwrap();
        accumulated_width += img.width();
    }

    imgbuf
}

pub fn scale_image(img: String, new_y: u32) -> String {
    let file_name = "new_input.png";
    let img_view = image::open(&img).unwrap();
    let x: u32 = img_view.width();
    let y: u32 = img_view.height();
    if y != new_y {
        let new_x: u32 = (x / y) * new_y;
        let scaled = img_view.resize(new_x, new_y, FilterType::Lanczos3);
        let mut output = File::create(file_name).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
        file_name.to_string()
    } else {
        img
    }
}
