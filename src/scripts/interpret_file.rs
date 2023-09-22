use std::{
    fs::File,
    io::{BufReader, Write},
};

use dbase::Reader;

pub fn interpret_file(file: Vec<u8>) -> Reader<BufReader<File>> {
    let path = "./test_files/test.dbf";
    let mut saved_file = File::create(path).unwrap();
    saved_file.write_all(&file).expect("problem creeating file");

    dbase::Reader::from_path(path).expect("error reading dbf")
}
