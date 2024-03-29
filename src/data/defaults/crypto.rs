use crate::locales::en::misc::lorem_ipsum::lorem_ipsum_sentence;
use bitcoin::hashes::{sha1, sha256, sha512, Hash};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn md5() -> String {
	let chars = lorem_ipsum_sentence();
	let digest = md5::compute(chars);
	format!("{:x}", digest)
}

#[wasm_bindgen]
pub fn sha1() -> String {
	let chars = lorem_ipsum_sentence();
	let h = sha1::Hash::hash(chars.as_bytes());
	h.to_string()
}

#[wasm_bindgen]
pub fn sha256() -> String {
	let chars = lorem_ipsum_sentence();
	let h = sha256::Hash::hash(chars.as_bytes());
	h.to_string()
}

#[wasm_bindgen]
pub fn sha512() -> String {
	let chars = lorem_ipsum_sentence();
	let h = sha512::Hash::hash(chars.as_bytes());
	h.to_string()
}
