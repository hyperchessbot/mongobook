# mongobook

[![documentation](https://docs.rs/mongobook/badge.svg)](https://docs.rs/mongobook) [![Crates.io](https://img.shields.io/crates/v/mongobook.svg)](https://crates.io/crates/mongobook) [![Crates.io (recent)](https://img.shields.io/crates/dr/mongobook)](https://crates.io/crates/mongobook)

Mongodb hosted chess opening book. Under construction.

# Usage

```rust
use dotenv::dotenv;
use std::env;

use mongobook::mongobook::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	dotenv().ok();
	
	println!("mongobook, under construction");
	
	println!("mongodb uri {}", env::var("MONGODB_URI").unwrap());
	
	let client = connect().await?;
	
	println!("mongo client {:?}", client);
	
	Ok(())
}

```

