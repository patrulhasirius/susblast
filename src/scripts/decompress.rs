use explode;

pub fn decompress(file: &Vec<u8>) -> Vec<u8> {
    //TODO deal with header weirness
    let result = explode::explode(file).expect("error in file exploding");
    result
}
