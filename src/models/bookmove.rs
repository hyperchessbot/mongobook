use serde::{Serialize, Deserialize};
use mongodb::bson::{doc, Document};
use log::{log_enabled, error, Level};

/// book move
#[derive(Debug, Serialize, Deserialize)]
pub struct BookMove {	
	/// sha
	pub sha: String,
	/// epd
	pub epd: String,
	/// san
	pub san: String,
	/// uci
	pub uci: String,	
	/// result wrt
	pub result_wrt: i32,
}

/// display book move
impl std::fmt::Display for BookMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("epd = {} san = {} uci = {} sha = {}",
        	self.epd, self.san, self.uci, self.sha
        ))
    }
}

// generate to bson conversion
convert_to_bson!(BookMove, "BookMove");

// generate from bson conversion
convert_from_bson!(BookMove, "BookMove");