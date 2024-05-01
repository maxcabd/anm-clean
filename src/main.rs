mod clean;

use std::path::Path;

use xfbin::{read_xfbin, write_xfbin};
use xfbin::nucc::{NuccAnm, NuccStruct};

use clean::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath = Path::new(&args[1]);

    if filepath.extension().unwrap() != "xfbin" {
        panic!("File does not end with .xfbin");
    }

    let mut xfbin = read_xfbin(&Path::new(filepath)).unwrap();

    for nucc_page in xfbin.pages.iter_mut() {
        for nucc_struct in nucc_page.structs.iter_mut() {
            if let Some(nucc_anm) = nucc_struct.downcast_mut::<NuccAnm>() {
                clean_anm(nucc_anm);
                *nucc_struct = Box::new(nucc_anm.clone()) as Box<dyn NuccStruct>;
            } 
        }
    }
    
    write_xfbin(xfbin, &Path::new(filepath)).unwrap();
    
}