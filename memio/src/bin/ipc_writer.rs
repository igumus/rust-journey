use memmap2::MmapOptions;
use std::fs::OpenOptions;

fn main() {
    let file_path = "./resources/shared.dat";
    let message = b"IPC using mmap in Rust!";

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .expect("failed to create file");

    file.set_len(message.len() as u64)
        .expect("Failed to set_len on file");

    let mut mmap = unsafe {
        MmapOptions::new()
            .map_mut(&file)
            .expect("failed to mmap file")
    };

    mmap[..message.len()].copy_from_slice(message);

    mmap.flush().unwrap();

    println!("Message written to shared memory.");
}
