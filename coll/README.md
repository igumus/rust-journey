# coll  

Simple collections library for educational purposes.

## Objectives

- Pointer types: `&`, `&mut`, `Box`, `Rc` 
- Ownership, borrowing, inherited mutability, interior mutability, Copy
- Pattern matching, generics, destructors
- Testing
- Unsafe Rust: raw pointers, aliasing, stacked borrows, UnsafeCell, variance
- ...

## Quick Start

```console
$ make test
```

## Tips

- To print during tests?
    ```console
    $ cargo test -- --nocapture
    ```
    ```console
    $ rustc --test main.rs; ./main
    ```
    ```console
    $ cargo watch "test -- --nocapture"
    ```

## Collections

Implemented collections are;
    - [Stack](./src/stack.rs)  
    - [Persistent Stack](./src/persistent_stack.rs) 
    - [Vector](./src/vector.rs)

## Notes

- ... TBD ...

## References

- ... TBD ...
