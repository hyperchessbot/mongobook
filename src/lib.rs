//!
//! # Examples
//!
//!
//!```
//!use dotenv::dotenv;
//!use std::env;
//!
//!fn main(){
//!	dotenv().ok();
//!	
//!	println!("mongobook, under construction");
//!	
//!	println!("mongodb uri {}", env::var("MONGODB_URI").unwrap());
//!}
//!```


// lib
pub mod mongobook;
