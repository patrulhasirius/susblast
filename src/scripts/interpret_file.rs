use std::{fs::File, io::Write};

pub fn interpret_file(file: Vec<u8>) -> Vec<dbase::Record> {
    let mut saved_file = File::create("./test_files/test.dbf").unwrap();
    saved_file.write_all(&file).expect("problem creeating file");

    let records = dbase::read("tests/data/line.dbf").expect("problem reading file");
    records
}
