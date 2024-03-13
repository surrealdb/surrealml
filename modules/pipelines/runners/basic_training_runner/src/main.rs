use basic::images::{
    read_rgb_image,
    write_frame_to_std_out,
};
use basic::tags::SurgeryStep;
use data_access_layer::images::{
    read_rgb_image,
    write_frame_to_std_out,
};
use data_access_layer::tags::SurgeryStep;


fn main() {
    let file_path = "./test.jpg";
    let height: usize = 480;
    let width: usize = 853;
    let data = read_rgb_image(file_path.to_string(), height, width);
    write_frame_to_std_out(data, SurgeryStep::ClippingAndCutting);
}
