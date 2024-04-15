use std::fs::File;

use memmap2::Mmap;

fn main() {
    let file_path = "./resources/shared.dat";
    let file = File::open(file_path).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap() };
    let content = &mmap[..];
    println!("New content: {:?}", content);
}
