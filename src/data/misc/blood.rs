use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn blood_type() -> String {
	let mut rng = rand::thread_rng();
	BLOOD_TYPES[rng.gen_range(0..BLOOD_TYPES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn rh_factory() -> String {
	let mut rng = rand::thread_rng();
	RH_FACTORS[rng.gen_range(0..RH_FACTORS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn group() -> String {
	let blood_type = blood_type();
	let rh_factor = rh_factory();
	return format!("{}{}", blood_type, rh_factor);
}

static BLOOD_TYPES: [&'static str; 4] = [
	"O", "A", "B", "AB",
];
static BLOOD_TYPES_LEN: usize = BLOOD_TYPES.len();

static RH_FACTORS: [&'static str; 2] = [
	"+", "-",
];
static RH_FACTORS_LEN: usize = RH_FACTORS.len();
