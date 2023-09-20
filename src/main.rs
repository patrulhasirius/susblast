mod scripts;

use crate::scripts::get_files;

fn main() {
    println!("Hello, world!");
    println!("{:#?}", get_files(vec![1], vec![1]));
}
