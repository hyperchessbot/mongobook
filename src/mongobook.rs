#![allow(unused_imports)]

use log::{log_enabled, debug, info, Level};
use mongodb::bson::{doc, Document, Bson};
use ring::{digest};

/// get environment variable as string with default
pub fn env_string_or<T, D>(key: T, default: D) -> String
where T: core::fmt::Display, D: core::fmt::Display {
	let key = format!("{}", key);
	let default = format!("{}", default);
	match std::env::var(&key) {
		Ok(value) => value,
		_ => default
	}
}

/// get environment variable as usize with default
pub fn env_usize_or<T>(key: T, default: usize) -> usize
where T: core::fmt::Display {
	let key = format!("{}", key);	
	match std::env::var(&key) {
		Ok(value) => value.parse::<usize>().unwrap_or(default),
		_ => default
	}
}

/// pgn with digest
#[derive(Debug)]
struct PgnWithDigest {	
	pgn_str: String,
	sha256_base64: String,
}

/// convert pgn with digest to bson
impl From<PgnWithDigest> for Document {
	fn from(pgn_with_digest: PgnWithDigest) -> Self {
        doc!{"_id": pgn_with_digest.sha256_base64, "pgn": pgn_with_digest.pgn_str}
    }
}

/// convert bson to pgn with digest
impl From<Document> for PgnWithDigest {
	fn from(document: Document) -> Self {
        PgnWithDigest{
			pgn_str: document.get("pgn").and_then(Bson::as_str).unwrap_or("").to_string(),
			sha256_base64: document.get("_id").and_then(Bson::as_str).unwrap_or("").to_string(),
		}
    }
}

/// display pgn with digest
impl std::fmt::Display for PgnWithDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("pgn = {}\nsha256(base64) = {}", self.pgn_str, self.sha256_base64))
    }
}

/// pgn with digest from display
impl From<&str> for PgnWithDigest {
	fn from(pgn_str: &str) -> Self {
		PgnWithDigest {
			pgn_str: pgn_str.to_string(),
			sha256_base64: base64::encode(digest::digest(&digest::SHA256, pgn_str.as_bytes()).as_ref()),
		}
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
	/// mongodb uri
	mongodb_uri: String,
	/// client
	client: Option<Client>,
	/// max book depth in plies
	book_depth: usize,
}

impl MongoBook {
	/// create new mongo book
	pub fn new() -> MongoBook {
		MongoBook {
			mongodb_uri: env_string_or("MONGODB_URI", "mongodb://localhost:27017"),
			client: None,
			book_depth: env_usize_or("BOOK_DEPTH", 40),
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

/// display for MongoBook
impl std::fmt::Display for MongoBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("MongoBook\n-> uri = {}\n-> book depth = {}", self.mongodb_uri, self.book_depth))
    }
}
