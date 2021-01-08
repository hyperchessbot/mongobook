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
