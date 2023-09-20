mod scripts;

use crate::scripts::get_files;

fn main() {
    println!("Hello, world!");
    let file = get_files(vec![1], vec![1]);
    println!("{:#?}", file);
}
