mod scripts;
use crate::scripts::decompress::decompress;
use crate::scripts::download_files::download_files;

fn main() {
    println!("Hello, world!");
    let file = decompress(&download_files(vec!["01".to_string()])[0]);
    println!("{:#?}", file);
}
