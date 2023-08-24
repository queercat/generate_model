extern crate generate_model;

use fake::{Dummy, Fake, Faker};
use generate_model::generate_model;
use serde::Serialize;

#[test]
pub fn test_generate_model() {
  #[generate_model]
  #[derive(Dummy, Serialize)]
  struct TestStruct {
    a: String,
    b: u32,
  }

  #[generate_model]
  #[derive(Dummy, Serialize)]
  struct TestStruct2 {
    a: TestStruct,
    b: u32,
  }
}

#[generate_model]
struct TestStruct {
  a: String,
  b: u32,
}
