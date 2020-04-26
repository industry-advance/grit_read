extern crate grit_read;

use grit_read::*;

use std::fs::File;
use std::io::Read;

#[test]
fn read_valid_grf() {
    let mut file = File::open("test_assets/valid.grf").unwrap();
    let mut test_data = Vec::new();
    file.read_to_end(&mut test_data).unwrap();
    let grf = Grf::from_slice(test_data.as_ref()).unwrap();
    assert_eq!(grf.header.tile_width, 8);
}
