[![Test](https://github.com/malwaredb/murmurhash3-rs/actions/workflows/test.yml/badge.svg)](https://github.com/malwaredb/murmurhash3-rs/actions/workflows/test.yml)[![Lint](https://github.com/malwaredb/murmurhash3-rs/actions/workflows/lint.yml/badge.svg)](https://github.com/malwaredb/murmurhash3-rs/actions/workflows/lint.yml)[![Crates.io Version](https://img.shields.io/crates/v/malwaredb-murmurhash3)](https://crates.io/crates/malwaredb-murmurhash3)

## MurmurHash3.rs

A rust implementation of the [MurmurHash3](https://code.google.com/p/smhasher/wiki/MurmurHash3) algorithm. Both 32 bit and 128 bit versions are included. The 128 bit version is implemented with 64 bit datatypes, making it most suitable for x86_64 or other 64 bit architectures.

This is a minor change from the upstream version. Modifications here remove dependencies on crates which prevented compilation on non-x86 hardware, and updated to Rust edition 2021.

### Usage

In your `Cargo.toml`:

```toml
[dependencies]
malwaredb-murmurhash3 = "0.1"
```

Then you can start to use either `murmurhash3_x86_32` or `murmurhash3_x64_128`:

```rust
use malwaredb_murmurhash3::murmurhash3_x64_128;

fn hash_value() {
    let data = "test data";
    let seed = 48221234;

    let hash = murmurhash3_x64_128(data.as_bytes(), seed);
}
```

Unfortunately, there is a bug in the ``HashState`` library implementation which prevents
implementation of new ``Hasher`` implementations for use in for example ``HashMap``. Additionally,
only the 32 bit hasher can be used there since ``HashMap`` uses a 64 bit hash internally.

### Tests
Run `cargo test`
