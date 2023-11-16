use crate::LesFiltres;
use image::imageops::colorops::{contrast_in_place, huerotate_in_place, invert};
use image::imageops::FilterType;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat, Pixel, Primitive};
use std::fs::File;
pub fn h_concat_vec<I, P, S>(images: Vec<I>, image_last: I, f: LesFiltres) -> ImageBuffer<P, Vec<S>>
where
    I: GenericImageView<Pixel = P> + image::GenericImage,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let img_width_out: u32 = images.iter().map(|im| im.width()).sum();
    let img_height_out: u32 = images.iter().map(|im| im.height()).max().unwrap_or(0);
    let img_height_out: u32 = std::cmp::max(image_last.height(), img_height_out);
    let img_width_out: u32 = img_width_out + image_last.width();

    let mut imgbuf = image::ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_width = 0;

    for mut img in images {
        let y = img_height_out - img.height();
        let x = img.width();
        apply_filter(&mut img, &f);
        // huerotate_in_place(&mut img, f.hue.unwrap());
        imgbuf.copy_from(&img, accumulated_width, y).unwrap();
        accumulated_width += x;
    }
    imgbuf
        .copy_from(
            &image_last,
            accumulated_width,
            img_height_out - image_last.height(),
        )
        .unwrap();

    imgbuf
}

fn apply_filter<I>(img: &mut I, f: &LesFiltres)
where
    I: GenericImage,
{
    match f.contrasty {
        Some(a) => contrast_in_place(img, a),
        None => huerotate_in_place(img, 0),
    };
    match f.hue {
        Some(a) => huerotate_in_place(img, a),
        None => huerotate_in_place(img, 0),
    };

    // if f.gray {
    //     grayscale_with_type_alpha(img);
    // };
    if f.invert {
        invert(img);
    };
}

pub fn scale_image(img: &str, new_y: u32) -> String {
    let file_name = "new_input.png";
    let img_view = image::open(img).unwrap();
    let x: u32 = img_view.width();
    let y: u32 = img_view.height();
    if y != new_y {
        let new_x: u32 = (x / y) * new_y;
        let scaled = img_view.resize(new_x, new_y, FilterType::Triangle);
        let mut output = File::create(file_name).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
        file_name.to_string()
    } else {
        img.to_owned()
    }
}
