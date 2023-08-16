mod storage;
mod execution;

use crate::storage::surml_file::SurMlFile;
use tch::{CModule, Tensor};
use crate::execution::compute::ModelComputation;
use std::collections::HashMap;

#[cfg(feature = "python")]
pub mod python_state;


fn main() {
    println!("Hello, .surml!");
    let mut file = SurMlFile::from_file("./test.surml").unwrap();
    println!("{:?}", file.header);
    file.model.set_eval();
    let x = Tensor::f_from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0]).unwrap().reshape(&[2, 2]);
    let outcome = file.model.forward_ts(&[x]);
    println!("{:?}", outcome);

    let computert_unit = ModelComputation {
        surml_file: &mut file
    };

    let mut input_values = HashMap::new();
    input_values.insert(String::from("squarefoot"), 1.0);
    input_values.insert(String::from("num_floors"), 2.0);

    let outcome = computert_unit.buffered_compute(&mut input_values);
    println!("{:?}", outcome);

    // let mut new_model = read_file::read().unwrap();
    // new_model.set_eval();
    // println!("{:?}", new_model);
    // let x = Tensor::f_from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0]).unwrap().reshape(&[2, 2]);
    // let outcome = new_model.forward_ts(&[x]);
    // println!("{:?}", outcome);

    // header.add_normaliser(String::from("squarefoot"), normaliser)


    // let mut model = CModule::load("./stash/test.surml").unwrap();

    // let new_model = CModule::load_data(&mut test).unwrap();
}