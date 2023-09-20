pub fn decompress(file: &[u8]) -> Vec<u8> {
    //TODO deal with header weirness
    explode::explode(file).expect("error in file exploding")
}
