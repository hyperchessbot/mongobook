use log::{log_enabled, error, info, Level};
use mongodb::bson::{doc, Document};
use pgnparse::parser::*;
use mongodb::{Client};
use mongodb::options::{UpdateOptions};
use futures::stream::StreamExt;

use crate::models::pgnwithdigest::*;
use crate::models::bookmove::*;
use crate::utils::env::*;
use crate::mongo::operations::*;
use crate::models::conv::{get_variant};

/// mongo book
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

	/// drop coll
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

	/// upsert one
	pub async fn upsert_one<T>(&mut self, coll: T, filter: Document, doc: Document)
	where T: core::fmt::Display {
		let coll = format!("{}", coll);

		if let Some(client) = &self.client {
			let result = client.database(&self.book_db).collection(&coll)
				.update_one(filter, doc!{"$set": doc}, UpdateOptions::builder().upsert(true).build()).await;

			match result {
				Ok(_) => {
					if log_enabled!(Level::Info) {
						info!("upserted in {} ok", coll);
					}
				},
				Err(err) => {
					if log_enabled!(Level::Error) {
						error!("upsert in {} failed {:?}", coll, err);
					}
				}
			}
		}
	}

	/// get moves for variant and epd
	pub async fn get_moves<T, V>(&mut self, variant: V, epd: T) -> std::collections::HashMap::<String, i32>
	where T: core::fmt::Display, V: core::fmt::Display {
		let variant = format!("{}", variant);
		let epd = format!("{}", epd);

		let mut moves = std::collections::HashMap::<String, i32>::new();

		if let Some(client) = &self.client {
			let result = client.database(&self.book_db).collection("moves")
				.find(doc!{
					"variant": variant.to_owned(),
					"epd": epd.to_owned(),					
				}, None).await;

			match result {
				Ok(cursor) => {
					let mut cursor = cursor;

					if log_enabled!(Level::Info) {
						info!("got cursor for epd {}", &epd);
					}

					while let Some(doc) = cursor.next().await {
						match doc {
							Ok(doc) => {
								let result_wrt = doc.get_i32("result_wrt").unwrap();
								let uci = doc.get("uci").unwrap().to_string();

								*moves.entry(uci).or_insert(0) += result_wrt;
							},
							Err(err) => {
								if log_enabled!(Level::Error) {
									info!("cursor next failed {:?}", err);
								}
							}
						}
					}
				},
				Err(err) => {
					if log_enabled!(Level::Error) {
						error!("getting cursor for {} failed {:?}", &epd, err);
					}
				}
			}
		}

		moves
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
				
				let mut pgn_with_digest:PgnWithDigest = pgn_str.into();
				
				if log_enabled!(Level::Info) {
					info!("processing pgn with sha {}", pgn_with_digest.sha256_base64);
				}
				
				let result = pgns.find_one(doc!{"_id": pgn_with_digest.sha256_base64.to_owned()}, None).await;

				let mut process_from = 0;

				let mut processed_depth = 0;

				if let Ok(Some(doc)) = result {
					let pgn_with_digest_stored:PgnWithDigest = doc.into();

					processed_depth = pgn_with_digest_stored.processed_depth as usize;

					if log_enabled!(Level::Info) {
						info!("pgn already in db {} processed depth {}",
							pgn_with_digest_stored.sha256_base64, processed_depth)
					}

					process_from = processed_depth;
				}

				let mut moves = parse_pgn_to_rust_struct(old_pgn_str);

				let num = moves.moves.len();
				
				if ( num <= processed_depth ) || ( processed_depth >= self.book_depth ) {
					if log_enabled!(Level::Info) {
						info!("pgn has no moves beyond processed depth, skipping")
					}
				}else{
					let mut process_to = self.book_depth;

					if num < process_to {
						process_to = num;
					}

					if log_enabled!(Level::Info) {
						let result = match moves.get_header("Result").as_str() {
							"1-0" => 2,
							"0-1" => 0,
							_ => 1
						};

						info!("pgn of orig variant {} and variant key {} has unprocessed moves from {} to {}\n{} {} - {} {} {}",
							moves.get_header("Variant"),
							get_variant(moves.get_header("Variant")),
							process_from,
							process_to,
							moves.get_header("White"),
							moves.get_header("WhiteElo"),
							moves.get_header("Black"),
							moves.get_header("BlackElo"),
							moves.get_header("Result"),
						);

						for i in process_from..process_to {
							let m = &moves.moves[i];

							let mut result_wrt = result;

							let parts:Vec<&str> = m.epd_before.split(" ").collect();

							if parts[1] == "b" {
								result_wrt = 2 - result_wrt;
							}

							if log_enabled!(Level::Info) {
								let sha = pgn_with_digest.sha256_base64.to_owned();
								let epd = m.epd_before.to_owned();
								let san = m.san.to_owned();
								let uci = m.uci.to_owned();

								let book_move = BookMove{
									variant: get_variant(moves.get_header("Variant")),
									sha: sha,
									epd: epd,
									san: san,
									uci: uci,
									result_wrt: result_wrt,
								};

								info!("adding move {}", book_move);

								let doc:Document = book_move.into();

								let result = db.collection("moves").insert_one(doc, None).await;
				
								match result {
									Ok(_) => {
										if log_enabled!(Level::Info) {
											info!("move inserted ok")
										}
									},
									Err(err) => {
										if log_enabled!(Level::Error) {
											error!("inserting move failed {:?}", err)
										}
									}
								}
							}
						}

						pgn_with_digest.processed_depth = process_to as i32;

						let sha = pgn_with_digest.sha256_base64.to_owned();

						let doc:Document = pgn_with_digest.into();
							
						if log_enabled!(Level::Info) {								
							info!("moves processed ok, updating {:?}", &doc);
						}
						
						let filter = doc!{"_id": sha};
						self.upsert_one("pgns", filter, doc).await;
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
