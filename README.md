# smplx

[![crates.io](https://img.shields.io/crates/v/concision.svg)](https://crates.io/crates/smplx)
[![docs.rs](https://docs.rs/concision/badge.svg)](https://docs.rs/smplx)

[![clippy](https://github.com/FL03/smplx/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/smplx/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/smplx/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/smplx/actions/workflows/rust.yml)

***

### _The library is currently in the early stages of development and is not yet ready for production use._

smplx is a research project supporting the Flow protocol 

## Features



## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/smplx.git
cd smplx
```

```bash
cargo build --features full -r --workspace
```

## Usage

### Example: Linear Model (biased)

```rust
    extern crate smplx;

    fn main() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        tracing::info!("Starting linear model example");


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)
