use mongodb::bson::{doc, Document};
use log::{log_enabled, error, Level};

use super::pgnwithdigest::*;

/// conversion macro to bson
macro_rules! convert_to_bson {
	($($type: ty, $typename: tt),*) => {
		$(
		impl From<$type> for Document {
			fn from(item: $type) -> Self {
		        match bson::to_bson(&item) {
		        	Ok(bson) => {
		        		match bson {
		        			bson::Bson::Document(doc) => doc,
		        			_ => {
				        		if log_enabled!(Level::Error) {
									error!("could not convert {} to bson ( conversion result was not a document )", $typename);
								}		

				        		doc!()
				        	}
		        		}
		        	},
		        	Err(err) => {
		        		if log_enabled!(Level::Error) {
							error!("could not convert {} to bson ( fatal ) {:?}", $typename, err);
						}		

		        		doc!()
		        	}
		        }
		    }
		}
		)*
	}
}

// generate to bson conversion
convert_to_bson!(PgnWithDigest, "PgnWithDigest");

/// conversion macro from bson
macro_rules! convert_from_bson {
	($($type: ty, $typename: tt),*) => {
		$(
		impl From<Document> for $type {
			fn from(item: Document) -> Self {
		        match bson::from_bson(bson::Bson::Document(item)){
		        	Ok(result) => result,
		        	Err(err) => {
						panic!("could not deserialize doc to {} {:?}", $typename, err)
		        	}
		        }
		    }
		}
		)*
	}
}

// generate from bson conversion
convert_from_bson!(PgnWithDigest, "PgnWithDigest");