use sparse_merkle_tree::{
  SMTState
};

fn main() {
  println!("Begin");
  unsafe {
    let state = SMTState::new(32);

    let mut key1: [u8; 32] = [0;32];
    key1[0] = 11;
    let value1: [u8; 32] = [0; 32];

    let ins_ref1 = state.insert(key1.as_ptr(), value1.as_ptr());
    println!("insert ref value: {}", ins_ref1);

    let mut key2: [u8; 32] = [0; 32];
    key2[0] = 22;
    let value2: [u8; 32] = [0; 32];

    let ins_ref2 = state.insert(key2.as_ptr(), value2.as_ptr());
    println!("insert ref value: {}", ins_ref2);

    state.normalize();

    let root_hash: [u8; 32] = [
      126, 135, 248, 129, 55,  74, 37, 2,  200, 13,  112, 6,  225, 50, 60, 119,
      17,  26,  212, 53,  176, 39, 79, 18, 40,  147, 224, 67, 94,  50, 10, 130
    ];
    let proof: [u8; 75] = [
      76, 79,  3,   81,  2,   107, 31,  136, 57, 101, 46,  172, 224, 208, 71,
      81, 138, 235, 101, 93,  254, 147, 224, 69, 164, 216, 73,  120, 175, 255,
      83, 63,  60,  252, 171, 50,  90,  0,   0,  0,   0,   0,   0,   0,   0,
      0,  0,   0,   0,   0,   0,   0,   0,   0,  0,   0,   0,   0,   0,   0,
      0,  0,   0,   0,   0,   0,   0,   0,   0,  76,  79,  4,   72,  79,  251];
    let verify_ref = state.verify(root_hash.as_ptr(), proof.as_ptr(), proof.len() as u32);
    println!("verify ref value: {}", verify_ref);

    state.del();
  }

  println!("End");
}