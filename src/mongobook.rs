use log::{log_enabled, info, Level};

//////////////////////////////////////////////////////////////////
// MongoDb
use mongodb::{Client, options::ClientOptions};
//use mongodb::bson::{doc, Document, Bson};

pub async fn connect() -> Result<Client, Box<dyn std::error::Error>>{
	// parse a connection string into an options struct
	let client_options = ClientOptions::parse(&std::env::var("MONGODB_URI").unwrap()).await?;

	// get a handle to the deployment
	let client = Client::with_options(client_options)?;

	// list the names of the databases in that deployment
	/*for db_name in client.list_database_names(None, None).await? {
		println!("db {}", db_name);
	}*/

	if log_enabled!(Level::Debug) {
		info!("mongodb connected");
	}		

	Ok(client)
}
//////////////////////////////////////////////////////////////////
