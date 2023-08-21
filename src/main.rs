mod storage;
mod execution;

#[cfg(test)]
mod transport;

#[cfg(feature = "python")]
pub mod python_state;


fn main() {
    println!("Hello, .surml!");
}