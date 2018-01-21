extern crate data_structure;

extern crate  bytebuffer;

// use data_structure::tree;
mod tree;
mod ringqueue;
mod test_some;
mod process_byte;

fn main() {
    // data_structure:: tree::run();
      // tree::run();
      // ringqueue::run();
      // test_some::run();
      // test_some::run_clone_data();
      process_byte::test_cast();
      process_byte::test_pack_unpack();
}