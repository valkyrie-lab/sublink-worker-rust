# Build Instructions

## Cross-compile for mipsel (MIPS32EL) with musl libc

### Prerequisites

- Nix or nix-shell
- Rust nightly toolchain with rust-src component

### Build mipsel-linux-musl Target

```bash
RUSTFLAGS="-C target-feature=+soft-float"
nix-shell --run "cargo build --target mipsel-unknown-linux-musl --release -Zbuild-std=std,panic_abort"
```

The binary will be at: `target/mipsel-unknown-linux-musl/release/sublink-worker-rust`

### Verify Binary

```bash
file target/mipsel-unknown-linux-musl/release/sublink-worker-rust
```

Expected output:
```
ELF 32-bit LSB pie executable, MIPS, MIPS32 rel2 version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-mipsel.so.1, stripped
```

### Notes

- The mipsel-linux-musl cross toolchain is automatically fetched from musl.cc via nix
- `-Zbuild-std=std,panic_abort` is required because the target is not fully supported by rustup
- `panic = "abort"` in Cargo.toml is required for this build configuration
