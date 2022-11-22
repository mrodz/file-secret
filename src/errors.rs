use std::fmt::Debug;

#[derive(Debug)]
pub struct FileError {
	pub file_name: String,
	pub message: String
}

impl LockError for FileError {
	fn message(&self) -> Option<String> {
		Some(self.file_name.clone())
	}

	fn name(&self) -> String {
		self.message.clone()
	}
}

pub trait LockError {
	fn name(&self) -> String;
	fn message(&self) -> Option<String>;
}

// impl ToString for dyn LockError {
// 	fn to_string(&self) -> String {
// 		return format!("Error: {}{}", self.name(), if let Some(message) = self.message() {
// 			" (".to_owned() + &message + ")"
// 		} else { String::new() })
// 	}
// }

impl Debug for dyn LockError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl std::fmt::Display for dyn LockError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Error: {}{}", self.name(), if let Some(message) = self.message() {
			" (".to_owned() + &message + ")"
		} else { String::new() })
	}
}

impl std::error::Error for dyn LockError {}

// pub type Result<T> = std::result::Result<T, Box<dyn LockError>>;