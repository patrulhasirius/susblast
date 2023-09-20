mod decompress;
mod download_files;
mod interpret_file;

use decompress::decompress;
use download_files::download_files;
use interpret_file::interpret_file;

pub fn get_files(_years: Vec<u8>, _months: Vec<u8>) {
    interpret_file(decompress(&download_files(vec!["01".to_string()])[0]));
}
