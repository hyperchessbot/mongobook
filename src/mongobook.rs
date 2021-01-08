#![allow(unused_imports)]

use log::{log_enabled, error, warn, debug, info, Level};
use mongodb::bson::{doc, Document, Bson};
use ring::{digest};
use pgnparse::parser::*;
use serde::{Serialize, Deserialize};
use mongodb::{Client};

use crate::models::pgnwithdigest::*;
use crate::utils::env::*;
use crate::mongo::operations::*;

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

				let mut process_from = 0;

				let processed_depth = pgn_with_digest.processed_depth as usize;

				if let Ok(Some(doc)) = result {
					let pgn_with_digest_stored:PgnWithDigest = doc.into();

					if log_enabled!(Level::Info) {
						info!("pgn already in db {} processed depth {}",
							pgn_with_digest_stored.sha256_base64, processed_depth)
					}

					process_from = processed_depth;
				}

				let mut moves = parse_pgn_to_rust_struct(old_pgn_str);

				let num = moves.moves.len();
				
				if num <= processed_depth {
					if log_enabled!(Level::Info) {
						info!("pgn has no moves beyond processed depth, skipping")
					}
				}else{
					let mut process_to = self.book_depth;

					if num < process_to {
						process_to = num;
					}

					if log_enabled!(Level::Info) {
						info!("pgn has unprocessed moves from {} to {}\n{} {} - {} {} {}",
							process_from,
							process_to,
							moves.get_header("White".to_string()),
							moves.get_header("WhiteElo".to_string()),
							moves.get_header("Black".to_string()),
							moves.get_header("BlackElo".to_string()),
							moves.get_header("Result".to_string()),
						);

						for i in process_from..process_to {
							let m = moves.moves[i];

							if log_enabled!(Level::Info) {
								info!("adding move {}", m.san);
							}
						}
					}
				}		
				
				/*match result {
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
				}*/
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
