use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn phone_number_with_country_code() -> String {
	let mut rng = rand::thread_rng();

	format!(
		"+{} {}",
		COUNTRYCODES[rng.gen_range(0..COUNTRYCODES_LEN)].to_string(),
		phone_number()
	)
}

#[wasm_bindgen]
pub fn phone_number() -> String {
	let mut rng = rand::thread_rng();
	let format = rng.gen_range(0..6);
	let exchange_code = format!(
		"{}{}{}",
		rng.gen_range(0..10),
		rng.gen_range(0..10),
		rng.gen_range(0..10)
	);
	let line_code = format!(
		"{}{}{}{}",
		rng.gen_range(0..10),
		rng.gen_range(0..10),
		rng.gen_range(0..10),
		rng.gen_range(0..10)
	);

	match format {
		0 => format!(
			"{}{}{}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		1 => format!(
			"{} {} {}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		2 => format!(
			"{}-{}-{}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		3 => format!(
			"({}) {}-{}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		4 => format!(
			"{}.{}.{}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		5 => format!(
			"({}) {} {}",
			AREACODES[rng.gen_range(0..AREACODES_LEN)].to_string(),
			exchange_code,
			line_code
		),
		_ => "".to_string(),
	}
}

static AREACODES: [&'static str; 290] = [
	"907", "205", "251", "256", "334", "479", "501", "870", "480", "520", "602", "623", "928",
	"209", "213", "310", "323", "408", "415", "510", "530", "559", "562", "619", "626", "650",
	"661", "707", "714", "760", "805", "818", "831", "909", "916", "925", "949", "951", "303",
	"719", "970", "203", "860", "202", "302", "239", "305", "321", "352", "386", "407", "561",
	"727", "772", "813", "850", "863", "904", "941", "954", "229", "404", "478", "706", "770",
	"912", "808", "319", "515", "563", "641", "712", "208", "217", "309", "312", "618", "630",
	"708", "773", "815", "847", "219", "260", "317", "574", "765", "812", "316", "620", "785",
	"913", "270", "502", "606", "859", "225", "318", "337", "504", "985", "413", "508", "617",
	"781", "978", "301", "410", "207", "231", "248", "269", "313", "517", "586", "616", "734",
	"810", "906", "989", "218", "320", "507", "612", "651", "763", "952", "314", "417", "573",
	"636", "660", "816", "228", "601", "662", "406", "252", "336", "704", "828", "910", "919",
	"701", "308", "402", "603", "201", "609", "732", "856", "908", "973", "505", "575", "702",
	"775", "212", "315", "516", "518", "585", "607", "631", "716", "718", "845", "914", "216",
	"330", "419", "440", "513", "614", "740", "937", "405", "580", "918", "503", "541", "215",
	"412", "570", "610", "717", "724", "814", "401", "803", "843", "864", "605", "423", "615",
	"731", "865", "901", "931", "210", "214", "254", "281", "325", "361", "409", "432", "512",
	"713", "806", "817", "830", "903", "915", "936", "940", "956", "972", "979", "435", "801",
	"276", "434", "540", "703", "757", "804", "802", "206", "253", "360", "425", "509", "262",
	"414", "608", "715", "920", "304", "307", "204", "226", "236", "249", "250", "263", "289",
	"306", "343", "354", "365", "367", "368", "403", "416", "418", "431", "437", "438", "450",
	"468", "474", "506", "514", "519", "548", "579", "581", "584", "587", "604", "613", "639",
	"647", "672", "683", "705", "709", "742", "753", "778", "780", "782", "807", "819", "825",
	"867", "873", "902", "905",
];
static AREACODES_LEN: usize = AREACODES.len();

static COUNTRYCODES: [&'static str; 29] = [
	"1", "86", "91", "7", "81", "44", "49", "82", "55", "33", "92", "90", "62", "39", "34", "84",
	"20", "30", "62", "63", "64", "65", "66", "852", "46", "41", "55", "54", "31",
];
static COUNTRYCODES_LEN: usize = COUNTRYCODES.len();
