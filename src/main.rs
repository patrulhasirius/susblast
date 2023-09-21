mod scripts;

use crate::scripts::get_files;

fn main() {
    println!("Now let's see how well you fare against my BIDEN BLAST!!!");
    println!("{:#?}", get_files(vec![1], vec![1]));
}
