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
