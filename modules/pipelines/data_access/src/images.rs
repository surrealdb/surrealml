//! # Image Buffer
//! In this module we are reading the image to a buffer and calculate the RGB indexes for each pixel.
use crate::tags::SurgeryStep;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, Rgb};
use std::io::{self, Write};

/// Represents the indexes of the red, green and blue components of a pixel.
///
/// # Fields
/// * `red` - The index of the red component in relation to the (x, y) coordinates of the pixel.
/// * `green` - The index of the green component in relation to the (x, y) coordinates of the pixel.
/// * `blue` - The index of the blue component in relation to the (x, y) coordinates of the pixel.
#[derive(Debug)]
struct RgbIndexes {
    red: usize,
    green: usize,
    blue: usize,
}

/// Calculates the RGB indexes for a given pixel in relation to the (x, y) coordinates of the pixel.
///
/// # Arguments
/// * `x` - The x coordinate of the pixel.
/// * `y` - The y coordinate of the pixel.
/// * `total_width` - The total width of the image frame.
/// * `total_height` - The total height of the image frame.
///
/// # Returns
/// A RgbIndexes struct containing the indexes of the red, green and blue components of the pixel.
fn calculate_rgb_index(x: usize, y: usize, total_width: usize, total_height: usize) -> RgbIndexes {
    RgbIndexes {
        red: 0 * total_height * total_width + y * total_width + x,
        green: 1 * total_height * total_width + y * total_width + x,
        blue: 2 * total_height * total_width + y * total_width + x,
    }
}

/// Reads an RGB image from a file and returns the raw data in 1D form that can be mapped as a 3D
/// array by using the `calculate_rgb_index` function.
///
/// # Arguments
/// * `path` - The path to the image file.
/// * `height` - The total height of the image.
/// * `width` - The total width of the image.
///
/// # Returns
/// A 1D array containing the raw RGB data of the image (flatterned).
pub fn read_rgb_image(path: String, height: usize, width: usize) -> Vec<u8> {
    // let height: usize = 480;
    // let width: usize = 853;
    let depth: usize = 3;

    let img: DynamicImage = ImageReader::open(path).unwrap().decode().unwrap();
    let resized_img: DynamicImage = img.resize_exact(
        width as u32,
        height as u32,
        image::imageops::FilterType::Nearest,
    );

    // Convert to RGB and flatten to array if necessary
    let rgb_img: ImageBuffer<Rgb<u8>, Vec<u8>> = resized_img.to_rgb8();

    let mut raw_data = vec![0u8; depth * height * width]; // 3 channels, 480 height, 853 width

    for chunk in rgb_img.enumerate_pixels() {
        let x: u32 = chunk.0;
        let y: u32 = chunk.1;
        let pixel: &Rgb<u8> = chunk.2; // [u8, u8, u8]

        let indexes = calculate_rgb_index(x as usize, y as usize, width, height);

        raw_data[indexes.red as usize] = pixel[0]; // store red component
        raw_data[indexes.green as usize] = pixel[1]; // store green component
        raw_data[indexes.blue as usize] = pixel[2]; // store blue component
    }
    raw_data
}

/// Writes a frame to the standard output.
///
/// # Arguments
/// * `data` - The raw data of the frame.
/// * `tag` - The tag associated with the frame.
pub fn write_frame_to_std_out(data: Vec<u8>, tag: SurgeryStep) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write the tag as a 2-byte integer
    handle
        .write_all(&(tag.to_u8() as u16).to_le_bytes())
        .unwrap();

    // Write the len as a 4-byte integer
    handle
        .write_all(&(data.len() as u32).to_le_bytes())
        .unwrap();

    // Write each byte in data as a 2-byte integer
    for byte in data {
        handle.write_all(&(byte as u16).to_le_bytes()).unwrap();
    }

    handle.flush().unwrap();
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct DummyJson {
        data: Vec<u8>,
    }

    #[test]
    fn test_read_image() {
        let _data = read_rgb_image("./data_stash/images/169_6300.jpg".to_string(), 480, 853);
    }

    #[test]
    fn test_calculate_rgb_index() {
        // This will give x y chunks of 50 and an entire rgb image of 150
        let total_height = 5;
        let total_width = 10;

        let indexes = calculate_rgb_index(0, 0, total_width, total_height);
        assert_eq!(&0, &indexes.red);
        assert_eq!(&50, &indexes.green);
        assert_eq!(&100, &indexes.blue);

        let indexes = calculate_rgb_index(1, 0, total_width, total_height);
        assert_eq!(&1, &indexes.red);
        assert_eq!(&51, &indexes.green);
        assert_eq!(&101, &indexes.blue);

        let indexes = calculate_rgb_index(2, 0, total_width, total_height);
        assert_eq!(&2, &indexes.red);
        assert_eq!(&52, &indexes.green);
        assert_eq!(&102, &indexes.blue);

        let indexes = calculate_rgb_index(0, 1, total_width, total_height);
        assert_eq!(&10, &indexes.red);
        assert_eq!(&60, &indexes.green);
        assert_eq!(&110, &indexes.blue);

        let indexes = calculate_rgb_index(0, 2, total_width, total_height);
        assert_eq!(&20, &indexes.red);
        assert_eq!(&70, &indexes.green);
        assert_eq!(&120, &indexes.blue);
    }

    #[test]
    fn test_test_calculate_rgb_index_quality_control() {
        let raw_data = std::fs::read_to_string("./data_stash/images/dummy_rgb_data.json").unwrap();
        let data: DummyJson = serde_json::from_str(&raw_data).unwrap();

        // This will give x y chunks of 50 and an entire rgb image of 150
        let total_height = 5;
        let total_width = 10;

        let index = calculate_rgb_index(0, 0, total_width, total_height);
        assert_eq!(&data.data[index.red], &111); // z = 0
        assert_eq!(&data.data[index.green], &208); // z = 1
        assert_eq!(&data.data[index.blue], &12); // z = 2

        let index = calculate_rgb_index(5, 3, total_width, total_height);
        assert_eq!(&data.data[index.red], &65);
        assert_eq!(&data.data[index.green], &7);
        assert_eq!(&data.data[index.blue], &193);

        let index = calculate_rgb_index(8, 2, total_width, total_height);
        assert_eq!(&data.data[index.red], &253);
        assert_eq!(&data.data[index.green], &133);
        assert_eq!(&data.data[index.blue], &115);
    }
}
