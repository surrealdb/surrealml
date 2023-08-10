mod storage;
mod execution;


fn main() {
    println!("Hello, .surml!");
    // // let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);
    // let mut model = CModule::load("./tests/linear.pt").unwrap();
    // model.set_eval();
    // println!("{:?}", model);
    // let x = Tensor::f_from_slice::<f32>(&[1.0, 2.0, 3.0, 4.0]).unwrap().reshape(&[2, 2]);
    // let outcome = model.forward_ts(&[x]);
    // println!("{:?}", outcome);

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