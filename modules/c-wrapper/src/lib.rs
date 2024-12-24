// use surrealml_core::
mod state;
mod api;
mod utils;


#[no_mangle]
pub extern "C" fn pass_raw_model(data: *const f32, length: usize) -> f32 {
    if data.is_null() {
        // Handle null pointer case
        return 0.0;
    }

    // Convert the raw pointer and length into a slice
    let slice = unsafe { std::slice::from_raw_parts(data, length) };

    // Example: Compute the sum of the elements in the array
    slice.iter().copied().sum()
}
