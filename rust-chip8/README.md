# rust-chip8

This is a CHIP-8 interpreter written in Rust.

Run the IBM rom:

```
cargo run
```

Or choose another:
```
cargo run src/roms/WIPEOFF
```

## References

- The central reference I followed. Easy to digest, and helped me get situated. https://tobiasvl.github.io/blog/write-a-chip-8-emulator/
- Another Rust-based interpreter. Hope to use as a reference for interfacing with wasm & canvas. Was helpful in debugging some issues with my 0xDxxx handlers. https://colineberhardt.github.io/wasm-rust-chip8/web/
- Really wonderful and precise technical reference to CHIP-8. http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
- Didn't read this one, but seems neat: https://blog.scottlogic.com/2017/12/13/chip8-emulator-webassembly-rust.html
