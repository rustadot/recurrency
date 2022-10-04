use sp_std::vec::Vec;

/// Handle serializing and deserializing from `Vec<u8>` to hexadecimal
#[cfg(feature = "std")]
pub mod as_hex {
	use serde::{Deserializer, Serializer};

	/// Serializes a `Vec<u8>` into a hexadecimal string
	pub fn serialize<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		impl_serde::serialize::serialize(bytes.as_slice(), serializer)
	}

	/// Deserializes a hexadecimal string into a `Vec<u8>`
	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
		impl_serde::serialize::deserialize(deserializer)
	}
}

/// Handle serializing and deserializing from `Option<Vec<u8>>` to hexadecimal
#[cfg(feature = "std")]
pub mod as_hex_option {
	use serde::{Deserializer, Serializer};

	/// Serializes a `Vec<u8>` into a hexadecimal string
	pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match bytes {
			Some(bytes) => impl_serde::serialize::serialize(bytes.as_slice(), serializer),
			None => serializer.serialize_none(),
		}
	}

	/// Deserializes a hexadecimal string into a `Vec<u8>`
	pub fn deserialize<'de, D: Deserializer<'de>>(
		deserializer: D,
	) -> Result<Option<Vec<u8>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		impl_serde::serialize::deserialize(deserializer).map(|r| Some(r))
	}
}
/// Handle serializing and deserializing from `Vec<u8>` to a UTF-8 string
#[cfg(feature = "std")]
pub mod as_string {
	use super::*;
	use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};

	/// Serializes a `Vec<u8>` into a UTF-8 string
	pub fn serialize<S: Serializer>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
		std::str::from_utf8(bytes)
			.map_err(|e| S::Error::custom(format!("Debug buffer contains invalid UTF8: {}", e)))?
			.serialize(serializer)
	}

	/// Serializes a UTF-8 string into a `Vec<u8>`
	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
		Ok(String::deserialize(deserializer)?.into_bytes())
	}
}

const PREFIX: &'static str = "<Bytes>";
const POSTFIX: &'static str = "</Bytes>";

/// Wraps `PREFIX` and `POSTFIX` around a `Vec<u8>`
/// Returns `PREFIX` ++ `data` ++ `POSTFIX`
pub fn wrap_binary_data(data: Vec<u8>) -> Vec<u8> {
	let mut encapsuled = PREFIX.as_bytes().to_vec();
	encapsuled.append(&mut data.clone());
	encapsuled.append(&mut POSTFIX.as_bytes().to_vec());
	encapsuled
}
