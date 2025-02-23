# smplx

[![crates.io](https://img.shields.io/crates/v/smplx.svg)](https://crates.io/crates/smplx)
[![docs.rs](https://docs.rs/smplx/badge.svg)](https://docs.rs/smplx)
[![license](https://img.shields.io/crates/l/smplx.svg)](https://crates.io/crates/smplx)

[![clippy](https://github.com/FL03/smplx/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/smplx/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/smplx/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/smplx/actions/workflows/rust.yml)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

Welcome to smplx, a topologically oriented crate focused on simplexes and their complexes.

## Features

- [x] Simplex
- [x] SimplicialComplex

## Background

A simplex describes the smallest (and simplest) polytope in any dimension. More specifically, $n$-simplex $\Delta^n$ is defined to be the $n$-dimensional polytope formed from the convex hull $C$ of its $n+1$ verices where the vertices are affinely independent points $u_0, u_1, \ldots, u_n \in \mathbb{R}^n$.

$C = \biggl\{ \sum_{i=0}^{k} \lambda_i v_i : \lambda_i\geq{0} \text{ for all } j \text{ and }\sum_{i=0}^{k} \lambda_i = 1 \biggr\}$

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/smplx.git
cd smplx
```

#### _Build the workspace_

```bash
cargo build --all-features -r --workspace
```

#### _Run an exmple_

```bash
cargo run -p smplx --example {basic|complex|...|etc.}
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.smplx]
features = ["full"]
version = "0.0.1"
```

### Examples

#### _Simplex_

```rust
    extern crate smplx;

    use smplx::NdSimplex;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to smplx!");

        let vertices = vec![array![3.0, 2.0], array![5.0, 3.0], array![3.0, 4.0]];
        let simplex = NdSimplex::from_iter(vertices);
        // validate the dimension and the number of nodes
        assert_eq!(simplex.dim(), 2);
        assert_eq!(simplex.nodes().shape(), &[3, 2]);

        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
