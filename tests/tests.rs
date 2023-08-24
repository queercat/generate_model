extern crate this_returns;

use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use this_returns::generate_model;

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
