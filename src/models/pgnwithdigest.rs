use ring::{digest};
use serde::{Serialize, Deserialize};

/// pgn with digest
#[derive(Debug, Serialize, Deserialize)]
pub struct PgnWithDigest {	
	/// pgn as string
	pub pgn_str: String,
	/// sha256 of pgn as base64
	#[serde(rename(serialize = "_id", deserialize = "_id"))]
	pub sha256_base64: String,
	/// processed depth
	pub processed_depth: i32,
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
