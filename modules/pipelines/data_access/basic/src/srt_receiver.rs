// Pipe frames to model for inference
use std::process::{Command, Stdio};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Define ffmpeg command to capture video and pipe it
    let mut ffmpeg_output = Command::new("ffmpeg")
        .args([
            "-i", "srt://127.0.0.1:2000?mode=caller",
            "-f", "image2pipe",
            "-vcodec", "mjpeg",
            "-"
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    // Get the output of ffmpeg
    let mut frames = ffmpeg_output.stdout.take().unwrap();

    // Build buffer for the frames
    // TO-DO: automatically obtain frame_size from the first frame
    let frame_size = 853 * 480;
    let mut buffer = vec![Ou8; frame_size]; 
    
    // Put frames into buffer
    while let Ok(_) = frames.read_exact(&mut buffer) {
        // Process the frame for inference
    }

    Ok(())
}
