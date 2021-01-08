# mongobook

[![documentation](https://docs.rs/mongobook/badge.svg)](https://docs.rs/mongobook) [![Crates.io](https://img.shields.io/crates/v/mongobook.svg)](https://crates.io/crates/mongobook) [![Crates.io (recent)](https://img.shields.io/crates/dr/mongobook)](https://crates.io/crates/mongobook)

Mongodb hosted chess opening book. Under construction.

# Usage

```rust
#![allow(unused_imports)]

#[macro_use]
extern crate log;

extern crate env_logger;

use dotenv::dotenv;
use std::env;

use mongobook::mongobook::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	dotenv().ok();
	env_logger::init();
	
	println!("mongobook, under construction");

	let mut mongobook = MongoBook::new();

	mongobook.connect().await;

	println!("{}", mongobook);

	let pgn = std::fs::read_to_string("test.pgn").unwrap();

	mongobook.add_pgn_to_book(pgn).await;
	
	Ok(())
}

```

# Logging

```bash
export RUST_LOG=info
# or 
export RUST_LOG=debug
```
