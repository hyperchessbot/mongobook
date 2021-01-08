#![allow(unused_imports)]

use log::{log_enabled, error, warn, debug, info, Level};
use mongodb::bson::{doc, Document, Bson};
use ring::{digest};
use pgnparse::parser::*;
use serde::{Serialize, Deserialize};

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
#[derive(Debug, Serialize, Deserialize)]
struct PgnWithDigest {	
	/// pgn as string
	pgn_str: String,
	/// sha256 of pgn as base64
	#[serde(rename(serialize = "_id", deserialize = "_id"))]
	sha256_base64: String,
	/// processed depth
	processed_depth: i32,
}

/// convert pgn with digest to bson
impl From<PgnWithDigest> for Document {
	fn from(pgn_with_digest: PgnWithDigest) -> Self {
        match bson::to_bson(&pgn_with_digest) {
        	Ok(bson) => {
        		match bson {
        			bson::Bson::Document(doc) => doc,
        			_ => {
		        		if log_enabled!(Level::Error) {
							error!("could not convert pgn with digest to bson ( conversion result was not a document )");
						}		

		        		doc!()
		        	}
        		}
        	},
        	Err(err) => {
        		if log_enabled!(Level::Error) {
					error!("could not convert pgn with digest to bson ( fatal ) {:?}", err);
				}		

        		doc!()
        	}
        }
    }
}

/// convert bson to pgn with digest
impl From<Document> for PgnWithDigest {
	fn from(document: Document) -> Self {
        match bson::from_bson(bson::Bson::Document(document)){
        	Ok(result) => result,
        	Err(err) => {
				panic!("could not deserialize to pgn with digest {:?}", err)
        	}
        }
    }
}

/// display pgn with digest
impl std::fmt::Display for PgnWithDigest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("pgn = {}\nsha256(base64) = {}\nprocessed depth = {}",
        	self.pgn_str, self.sha256_base64, self.processed_depth
        ))
    }
}

/// pgn with digest from display
impl From<&str> for PgnWithDigest {
	fn from(pgn_str: &str) -> Self {
		PgnWithDigest {
			pgn_str: pgn_str.to_string(),
			sha256_base64: base64::encode(digest::digest(&digest::SHA256, pgn_str.as_bytes()).as_ref()),
			processed_depth: 0,
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
	/// book db
	book_db: String
}

impl MongoBook {
	/// create new mongo book
	pub fn new() -> MongoBook {
		MongoBook {
			mongodb_uri: env_string_or("MONGODB_URI", "mongodb://localhost:27017"),
			client: None,
			book_depth: env_usize_or("BOOK_DEPTH", 40),
			book_db: env_string_or("BOOK_DB", "rustbook"),
		}
	}

	/// set book depth and return self
	pub fn book_depth(mut self, book_depth: usize) -> MongoBook {
		self.book_depth = book_depth;

		self
	}

	/// connect
	pub async fn connect(&mut self) {
		match connect(&self.mongodb_uri).await {
			Ok(client) => self.client = Some(client),
			_ => self.client = None
		}		
	}

	/// drop pgns
	pub async fn drop_coll<T>(&mut self, coll: T)
	where T: core::fmt::Display {
		let coll = format!("{}", coll);

		if let Some(client) = &self.client {
			if log_enabled!(Level::Info) {
				info!("dropping {}", coll);
			}

			match client.database(&self.book_db).collection(&coll).drop(None).await {
				Ok(_) => {
					if log_enabled!(Level::Info) {
						info!("{} dropped ok", coll);
					}
				},
				Err(err) => {
					if log_enabled!(Level::Error) {
						error!("dropping {} failed {:?}", coll, err);
					}
				}
			}
		}
	}

	/// add pgn to book
	pub async fn add_pgn_to_book<T>(&mut self, all_pgn: T)
	where T: core::fmt::Display {
		let all_pgn = format!("{}", all_pgn);

		if let Some(client) = &self.client {
			if log_enabled!(Level::Info) {
				info!("adding pgn of size {} to book", all_pgn.len());
			}

			let db = client.database(&self.book_db);
			let pgns = db.collection("pgns");

			let mut items:Vec<&str> = all_pgn.split("\r\n\r\n\r\n").collect();	
			let _ = items.pop();

			if log_enabled!(Level::Info) {
				info!("number of games {}", items.len());
			}
			
			for pgn_str in items {
				let old_pgn_str = pgn_str.to_owned();
				
				let pgn_with_digest:PgnWithDigest = pgn_str.into();
				
				if log_enabled!(Level::Info) {
					info!("processing pgn with sha {}", pgn_with_digest.sha256_base64);
				}
				
				let result = pgns.find_one(doc!{"_id": pgn_with_digest.sha256_base64.to_owned()}, None).await;
				
				match result {
					Ok(Some(doc)) => {
						let pgn_with_digest_stored:PgnWithDigest = doc.into();

						if log_enabled!(Level::Info) {
							info!("pgn already in db {}", pgn_with_digest_stored.sha256_base64)
						}
					},
					_ => {
						let mut moves = parse_pgn_to_rust_struct(old_pgn_str);
						
						if moves.moves.len() > 0 {
							if log_enabled!(Level::Info) {
								info!("{} {} - {} {} {}",
									moves.get_header("White".to_string()),
									moves.get_header("WhiteElo".to_string()),
									moves.get_header("Black".to_string()),
									moves.get_header("BlackElo".to_string()),
									moves.get_header("Result".to_string()),
								);
							}

							let doc:Document = pgn_with_digest.into();
							
							if log_enabled!(Level::Info) {								
								info!("pgn not in db, inserting {:?}", &doc);
							}
							
							let result = pgns.insert_one(doc, None).await;
				
							match result {
								Ok(_) => {
									if log_enabled!(Level::Info) {
										info!("pgn inserted ok")
									}
								},
								Err(err) => {
									if log_enabled!(Level::Error) {
										error!("inserting pgn failed {:?}", err)
									}
								}
							}
						}
					}
				}
			}
		}
	}
}

/// display for MongoBook
impl std::fmt::Display for MongoBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("MongoBook\n-> uri = {}\n-> book depth = {}",
        	self.mongodb_uri, self.book_depth
        ))
    }
}
