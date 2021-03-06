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

	let mut mongobook = MongoBook::new().book_depth(10);

	mongobook.connect().await;

	println!("{}", mongobook);

	let pgn = std::fs::read_to_string("test.pgn").unwrap();

	//mongobook.drop_coll("pgns").await;
	//mongobook.drop_coll("moves").await;

	mongobook.add_pgn_to_book(pgn).await;

	let start_epd = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -";

	println!("{:?}", mongobook.get_moves("standard", start_epd).await);
	
	Ok(())
}

```

# Logging

```bash
export RUST_LOG=info
# or 
export RUST_LOG=debug
```
