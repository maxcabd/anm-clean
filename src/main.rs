use std::{fs::File, path::Path};
use binrw::{BinReaderExt, BinWriterExt};

mod structure;
mod utils;

use crate::structure::anm::NuccAnm;
use crate::structure::clean::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = Path::new(&args[1]);

    let mut anm = File::open(filepath)
        .unwrap()
        .read_be::<NuccAnm>()
        .unwrap();

    for entry in anm.entries.iter_mut() {
        for (curve, curve_header) in entry.curves.iter_mut().zip(&mut entry.curve_headers) {
            clean_curve(curve, curve_header);
            update_header(curve, curve_header);
        }
    }

    let mut file = File::create(filepath).unwrap();
    file.write_be(&anm).unwrap();
}