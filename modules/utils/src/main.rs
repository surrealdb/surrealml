use surrealml_core::storage::surml_file::SurMlFile;
use surrealml_core::execution::compute::ModelComputation;
use std::collections::HashMap;
use std::io::Write;


fn main() {
    let mut file = SurMlFile::from_file("./stash/test.surml").unwrap();

    let compute_unit = ModelComputation {
        surml_file: &mut file,
    };

    // automatically map inputs and apply normalisers to the compute if this data was put in the header
    let mut input_values = HashMap::new();
    input_values.insert(String::from("squarefoot"), 1000.0);
    input_values.insert(String::from("num_floors"), 2.0);

    let output = compute_unit.buffered_compute(&mut input_values).unwrap();

    // write output to txt file
    let mut file = std::fs::File::create(format!("./output/output.txt")).unwrap();
    file.write_all(format!("{}", output[0]).as_bytes()).unwrap();
    println!("output: {:?}", output);
}