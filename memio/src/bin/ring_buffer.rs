use memmap2::MmapMut;
use std::{fs::OpenOptions, io};

struct RingBuffer {
    backed: MmapMut,
    capacity: usize,
    head: usize,
    tail: usize,
}

impl RingBuffer {
    fn new(file_path: &str, size: usize) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        file.set_len(size as u64)?;
        let mmap = unsafe { memmap2::MmapOptions::new().map_mut(&file)? };

        Ok(Self {
            backed: mmap,
            capacity: size,
            head: 0,
            tail: 0,
        })
    }

    fn write(&mut self, data: &[u8]) {
        for byte in data {
            self.backed[self.head] = *byte;
            self.head = (self.head + 1) % self.capacity;
            if self.head == self.tail {
                self.tail = (self.tail + 1) % self.capacity;
            }
        }
    }
}

fn main() {
    let mut buff =
        RingBuffer::new("./resources/rbuf.data", 1024).expect("failed to create ring buffer");
    buff.write(b"selam");
    println!("hello from ring buffer");
}
