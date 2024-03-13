//! Live video stream receiver using SRT protocol and
//! Frame extraction from video file using FFmpeg.
use std::process::Command;

/// Receives a live video stream using the SRT protocol and saves it to a file.
///
/// # Arguments
/// * `input_url` - The streaming server URL.
/// * `output_file` - The path to save the received video.
fn receive_srt_stream(input_url: &str, output_file: &str) -> std::io::Result<()> {
    let _output = Command::new("ffmpeg")
    .args([
        "-i", input_url, // Input URL
        "-c", "copy",    // Copy the video and audio codecs, no need to re-encode
        "-f", "mp4",     // Output file format
        output_file      // Output file
        ])
        .status()?;
    
    if _output.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "ffmpeg failed"))
    }
}

/// Converts a video file to frames using FFmpeg.
///
/// # Arguments
/// * `video_file` - The path to input video file.
/// * `output_folder` - The directory to save the extracted frames.
fn convert_video_to_frames(video_file: &str, output_folder: &str) -> std::io::Result<()> {
    let _output = Command::new("ffmpeg")
    .args([
        "-i", video_file, // Input video file
        "-vf", "fps=1", // Extract one frame per second
        &format!("{}/out%d.png", output_folder) // Save frames as PNG images
        ])
        .status()?;

    if _output.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "ffmpeg failed"))
    }
}
    
#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use image::io::Reader as ImageReader;
    use image::{DynamicImage, GenericImageView};

    #[test]
    fn test_receive_srt_stream() {
        /*
         * This test requires a running SRT server to pass!!
         */

        // Define input and output file paths for testing
        let input_url = "srt://127.0.0.1:9999?mode=caller";
        let output_file = "output.mp4";

        // Call the function being tested
        let result = receive_srt_stream(input_url, output_file);

        // Assert that the function returns Ok(())
        assert!(result.is_ok());

        // Clean up the output file
        std::fs::remove_file(output_file).expect("Failed to clean up the output file");
    }

    #[test]
    fn test_convert_video_to_frames() {
        /*
         * This test use a dummy video generated with the stored example images
         * The dummy video is converted to frames
         * And the frames are compared with the original images
         * 
         * TO-DO: ffmpeg only reads the first two images in the filelist.txt, fix this
         */
        // Define input video file and output folder for testing
        let output_folder = "../data_stash/images/frames";
        let dummy_video = "../data_stash/images/dummy_vid.mp4";
        let base_path = "../data_stash/images/169_6300.jpg";

        // Create the output folder for testing
        if fs::metadata(output_folder).is_err() {
            fs::create_dir(output_folder).expect("Failed to create output folder");
        }

        // Generate a dummy video file for testing
        let _output = Command::new("ffmpeg")
            .args([
                "-f", "concat", // Use all images in the specified folder
                "-safe", "0", // Use all images in the specified folder
                "-i", "../data_stash/images/filelist.txt", // Use all images in the specified folder
                "-c:v", "libx264", // Use the H.264 codec for video
                "-crf", "0", // Use the H.264 codec for video
                "-r", "1", // Set the output frame rate to 30 FPS
                "-pix_fmt", "yuv420p", // Set the pixel format
                dummy_video // Output file
            ])
            .status()
            .expect("Failed to generate video file");
        // fs::File::create(video_file).expect("Failed to create video file");

        // Call the function being tested
        let result = convert_video_to_frames(dummy_video, output_folder);

        fn load_image(path: &str) -> DynamicImage {
            ImageReader::open(path)
                .expect("Failed to open image.")
                .decode()
                .expect("Failed to decode image.")
        }

        fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> bool {
            if img1.dimensions() != img2.dimensions() {
                println!("The images have different dimensions.");
                return false;
            }
    
            true
        }

        let img1 = load_image(&base_path);
        let img2 = load_image(&format!("{}/out1.png", output_folder));

        if compare_images(&img1, &img2) {
            println!("The images are identical in size.");
        } else {
            println!("The images are not identical in size.");
        }

        // Assert that the function returns Ok(())
        assert!(result.is_ok());

        // Clean up the video file and output folder
        fs::remove_file(dummy_video).expect("Failed to clean up video file");
        // Mannually check whether the image and the frame are identical
        // fs::remove_dir_all(output_folder).expect("Failed to clean up output folder");
    }

}
