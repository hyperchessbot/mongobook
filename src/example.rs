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
