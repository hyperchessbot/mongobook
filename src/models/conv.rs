/// get variant key for variant name
pub fn get_variant<T>(variant_name: T) -> String
where T: core::fmt::Display {
	let variant_name = format!("{}", variant_name);

	match variant_name.to_lowercase().as_str() {
		"antichess" | "anti chess" | "giveaway" => "antichess",
		"atomic" => "atomic",
		"chess960" | "chess 960" => "chess960",
		"crazyhouse" | "crazy house" => "crazyhouse",
		"fromposition" | "from position" => "fromposition",
		"horde" => "horde",
		"kingofthehill" | "king of the hill" | "koth" => "kingofthehill",		
		"racingkings" | "racing kings" => "racingkings",
		"threecheck" | "three check" | "3check" | "3 check" => "threecheck",
		_ => "standard",
	}.to_string()
}

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
