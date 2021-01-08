//!
//! # Examples
//!
//!
//!```
//!#![allow(unused_imports)]
//!
//!#[macro_use]
//!extern crate log;
//!
//!extern crate env_logger;
//!
//!use dotenv::dotenv;
//!use std::env;
//!
//!use pgnparse::parser::*;
//!
//!use mongobook::mongobook::*;
//!
//!#[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>>{
//!	dotenv().ok();
//!	env_logger::init();
//!	
//!	println!("mongobook, under construction");
//!	
//!	//println!("mongodb uri {}", env::var("MONGODB_URI").unwrap());
//!	
//!	//let client = connect(env_string_or("MONGODB_URI", "")).await?;
//!	
//!	//println!("\nmongo client {:?}\n", client);
//!	
//!	//let pgn = std::fs::read_to_string("test.pgn")?;
//!	
//!	//println!("\n{:?}\n", parse_pgn_to_rust_struct(pgn));
//!
//!	let mut mongobook = MongoBook::new();
//!
//!	mongobook.connect().await;
//!
//!	println!("{}", mongobook);
//!	
//!	Ok(())
//!}
//!
//!```


// lib
pub mod mongobook;
