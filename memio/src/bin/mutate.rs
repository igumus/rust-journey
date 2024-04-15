use std::fs::OpenOptions;

fn main() {
    let file_path = "./resources/mutate.png";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .expect("Can not open file");
    let fsize = file.metadata().expect("Failed to read file metadata").len();
    file.set_len(fsize + 6).unwrap();
    let mut mmap = unsafe {
        memmap2::MmapOptions::new()
            .map_mut(&file)
            .expect("Failed to mmap file")
    };
    let message = b"123456";
    let mlen = mmap.len();
    mmap[mlen - message.len()..].copy_from_slice(b"167890");
    mmap.flush().expect("Failed to flush mmap");
    println!("Memory-Mapped File updated");
}
