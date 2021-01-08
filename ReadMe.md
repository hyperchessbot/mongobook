# mongobook

[![documentation](https://docs.rs/mongobook/badge.svg)](https://docs.rs/mongobook) [![Crates.io](https://img.shields.io/crates/v/mongobook.svg)](https://crates.io/crates/mongobook) [![Crates.io (recent)](https://img.shields.io/crates/dr/mongobook)](https://crates.io/crates/mongobook)

Mongodb hosted chess opening book. Under construction.

# Usage

```rust
extern crate env_logger;

use dotenv::dotenv;

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
