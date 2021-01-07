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
