use log::{log_enabled, debug, info, Level};

/// get environment variable with default
pub fn env_string_or<T, D>(key: T, default: D) -> String
where T: core::fmt::Display, D: core::fmt::Display {
	let key = format!("{}", key);
	let default = format!("{}", default);
	match std::env::var(&key) {
		Ok(value) => value,
		_ => default
	}
}

//////////////////////////////////////////////////////////////////
// MongoDb
use mongodb::{Client, options::ClientOptions};
//use mongodb::bson::{doc, Document, Bson};

/// connect to mongodb
pub async fn connect<T>(mongodb_uri: T) -> Result<Client, Box<dyn std::error::Error>>
where T: core::fmt::Display {
	let mongodb_uri = format!("{}", mongodb_uri);

	// parse a connection string into an options struct
	let client_options = ClientOptions::parse(&mongodb_uri).await?;

	// get a handle to the deployment
	let client = Client::with_options(client_options)?;

	// list the names of the databases in that deployment
	/*for db_name in client.list_database_names(None, None).await? {
		println!("db {}", db_name);
	}*/

	if log_enabled!(Level::Info) {
		info!("mongodb connected");
	}		

	Ok(client)
}
//////////////////////////////////////////////////////////////////

pub struct MongoBook {
	mongodb_uri: String,
	client: Option<Client>,
}

impl MongoBook {
	/// create new mongo book
	pub fn new() -> MongoBook {
		MongoBook {
			mongodb_uri: env_string_or("MONGODB_URI", "mongodb://localhost:27017"),
			client: None,
		}
	}

	/// connect
	pub async fn connect(&mut self) {
		match connect(&self.mongodb_uri).await {
			Ok(client) => self.client = Some(client),
			_ => self.client = None
		}		
	}
}
