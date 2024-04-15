# memio

Simple project to experiment Memory-Mapped I/O. Memory-Mapped I/O is especially helpful when working with big files, loading only the necessary parts into memory. This makes it simpler to access and change file data as if you were working with regular memory, which can speed things up, especially for apps that need to jump around a lot in a file.

## 

## Objectives

- TBD

## Quick Start

- [Reading from a Memory-Mapped File](./src/bin/rff.rs)

The example shows how to open an existing file, memory-map it and read data from mapped file.
```console
$ cargo run --bin rff
```

- [Using Memory-Mapped File as IPC](./src/bin/ipc.rs)

The example shows how memory-mapped files can be used as data sharing between processes. Includes reading and writing
to memory mapped file. First write to file, than read from it from another process.

For Writing;
```console
$ cargo run --bin wipc
```
For Reading;
```console
$ cargo run --bin ripc
```

- [Mutating Memory-Mapped File](./src/bin/mutate.rs)

The example shows how memory-mapped files can be modified.
```console
$ cargo run --bin mutate 
```

## Dependencies

- [memmap2](https://crates.io/crates/memmap2)

## References

- TBD