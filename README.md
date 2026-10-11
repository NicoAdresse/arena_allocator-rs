# Arena Allocator In Rust <->

---

## About <->

A simple Arena Memory Allocator written in Rust.

---

## How To Run <->

Make sure you have these core things installed:
- **Rustc**: Rust Compiler (along with `nightly`)
- **Cargo**: Rust Package Manager
- **Bootimage**: Loading our code into a `.bin`
- **QEMU**: The virtual machine that we'll use to load our `.bin`

This assumes that you've already cloned this repository. If not, do it.

Step #1: Override `rustup` with `nightly`. Run this command to do just that.
```bash
rustup override set nightly
```

Step #2: Load the source code into a `.bin`.
```bash
cargo bootimage
```

This will automatically create a new `.bin` file in the `target/x86_64-kernel/debug` directory. \
In our case, the file would be called: `bootimage-arena-allocator-rs.bin`.

Step #3: Load the `.bin` into QEMU.
```bash
qemu-system-x86_64 -drive format=raw,file=bootimage-arena-allocator-rs.bin
# Hint, don't use any whitespaces. QEMU will not parse it.
# If you use qemu-system-i386, that works as well.
```

Step #4: Profit.

---

## File Structure <->

```txt
ArenaAllocator
├── Cargo.toml
├── README.md
├── src
│   └── main.rs
└── x86_64-kernel.json
```

---

## License <->

This project is licensed under the [MIT License](./LICENSE).