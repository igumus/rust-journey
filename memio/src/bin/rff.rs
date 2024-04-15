use std::fs::File;

use memmap2::Mmap;

fn main() {
    let file_path = "./resources/logo.png";
    let file = File::open(file_path).unwrap();
    let mmio = unsafe { Mmap::map(&file).unwrap() };
    let data = &mmio[0..=8];
    println!("First 8 bytes: {:X?}", data);
}
