use std::{
    fs::File,
    io::{BufReader, Write},
};

use dbase::Reader;

pub fn interpret_file(file: Vec<u8>) -> Reader<BufReader<File>> {
    let mut saved_file = File::create("./test_files/test.dbf").unwrap();
    saved_file.write_all(&file).expect("problem creeating file");

    dbase::Reader::from_path("./test_files/test.dbf").expect("error reading dbf")
}
