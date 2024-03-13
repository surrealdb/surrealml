# Data Access

Here is where we house libraries that handle the reading and writing of data. For now we are merely just reading from jpeg files. However,
we will moved onto support for networking interfaces.

# Basic

Basic is a library that handles the reading of jpeg files. It is a simple library that is used to read in the jpeg files and convert them
to a stream of bytes. The loading and conversion of the jpeg files is done in the `data_access/basic/src/images.rs`.
For our images we are handling data in the following outline: 

<p align="center">
  <img src="static/Image_plane.jpg" alt="Alt text" style="width:480px; height:400px;">
</p>

Here we can see that we have three layers of a frame. Each layer of the `Z` axis corresponds to the RGB values of the 
pixel in the `X, Y` coordinate. We package the image frame as a `1D` array of `u8` values. The `u8` values and calculate
the index of the pixel in the `1D` array with the following formula example where the maximum width is `10`, the maximum
height is `5`, and there are `3` layers for the RGB values:

```bash
(3, 480, 853) => (channels, height, width) => (z, y, x)
```

<p align="center">
  <img src="static/coordinates.jpg" alt="Alt text" style="width:480px; height:600px;">
</p>

Here we can see that we can map the `X, Y, Z` coordinates to the `1D` array. To see the sequence of this mapping we can
look at the following testing code:

```rust
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
```

We can see that our mapping function follows the exact same pattern as the reshape function that `numpy` has and this
can be seen in the file `engines/pytorch_train/tests/test_numpy_quality_control.py`.

# Networking

At this point in time we are just handling image files in the `basic` module in rust, and piping this data into the
python pytorch engine as seen in the following example:

```bash
./data_access_rust_bin | python pytorch_engine.py
```

This means we can chunk the data into the stream and thus the ML model to be trained further. We are doing this to
give users flexibility on the size of the RAM memory needed to train the model. For instance, if the user has a
`60GB` folder of images, it is unlikely that they will be able to load all of these images into memory at once 
as depicted in the following:

```bash
[rust (basic)] ===> [1, 0, 0, 1, 1, 0, 1] ===> [1, 0, 0, 1, 1, 0, 1] ===> [python (ML)]
```

This also gives us a lot of flexibility in the future. For instance, if we need to send the training data over
a network we can easily swap out the `std::io::stdin` with a networking layer like the following:

```bash
[rust (basic)] ===> [TCP (packet)] ===> [TCP (packet)] ===> [python (ML)]
```

We can use the `Command` in a program to coordinate pipes over multiple cores and manage the flow of data.
We can also pipe in the `ffmpeg` command as seen in the following example:

```bash
ffmpeg -i 'srt://192.168.1.345:40052?mode=caller' | ./data_access_rust_bin | python pytorch_engine.py
```

We can map this with the following Rust code:

```rust
use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    // Start the ffmpeg process
    let ffmpeg_output = Command::new("ffmpeg")
        .args(["-i", "srt://192.168.1.345:40052?mode=caller"])
        .stdout(Stdio::piped())
        .spawn()?;

    // Assuming `data_access_rust_bin` is the compiled binary you want to run next
    let rust_binary_output = Command::new("./data_access_rust_bin")
        .stdin(ffmpeg_output.stdout.unwrap()) // Use the output of ffmpeg as input
        .stdout(Stdio::piped())
        .spawn()?;

    // Finally, pass the output of your Rust binary to the Python script
    let python_output = Command::new("python")
        .arg("pytorch_engine.py")
        .stdin(rust_binary_output.stdout.unwrap()) // Use the output of the Rust binary as input
        .output()?;

    // Here you can handle the final output, for example, print it
    println!("Python script output: {}", String::from_utf8_lossy(&python_output.stdout));

    Ok(())
}

```

# Local Test Setup for FFmpeg

## SRT listener server in OBS Studio

At this stage, we do not have steady access to Panasonic [AW-UE150](https://eu.connect.panasonic.com/gb/en/products/broadcast-proav/aw-ue150) cameras. Hence, for initial testing purposes, we set up a Secure Reliable Transport (SRT) listener server using OBS Studio. The server URL is 

```powershell
srt://127.0.0.1:9999?mode=listener&timeout=500000&transtype=live
```

- **'127.0.0.1':** IP address of the listener server
- **'9999':** Port of the listener server
- **'timeout=500000':** The listener server waits for connection for 500 s before auto-abort
- **'transtype=live':** Optimised for live streaming

Output resolution is set to 1920 X 1080 with an FPS of 1. The images in the [CAMMA-public/cholect50](https://github.com/CAMMA-public/cholect50) are sourced as an Image Slide Show.

The listener listens for connection requests from callers. The callers are implemented in `srt_receiver.rs`.

## Download and Install FFmpeg

Download FFmpeg [here](https://www.ffmpeg.org/download.html). Versions are available for Windows, Linux, and Mac OS.

## FFmpeg Documentation

Official documentation of FFmpeg is [here](https://www.ffmpeg.org/documentation.html).
