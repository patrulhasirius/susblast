use std::{
    fs::File,
    io::{BufReader, Write},
};

use dbase::Reader;

pub fn interpret_file(path: std::path::PathBuf, file: Vec<u8>) -> Reader<BufReader<File>> {
    let mut saved_file =
        File::create(format!("{}temp.dbf", path.clone().display())).expect("error on temp_path");
    saved_file.write_all(&file).expect("problem creating file");

    dbase::Reader::from_path(format!("{}temp.dbf", path.display())).expect("error reading dbf")
}
