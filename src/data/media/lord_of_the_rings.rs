use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lord_of_the_rings_characters() -> String {
  let mut rng = rand::thread_rng();
  CHARACTERS[rng.gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lord_of_the_rings_locations() -> String {
  let mut rng = rand::thread_rng();
  LOCATIONS[rng.gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn lord_of_the_rings_quotes() -> String {
  let mut rng = rand::thread_rng();
  QUOTES[rng.gen_range(0..QUOTES_LEN)].to_string()
}

static CHARACTERS: [&'static str; 30] = [
  "Frodo Baggins",
  "Gandalf the Grey",
  "Samwise Gamgee",
  "Meriadoc Brandybuck",
  "Peregrin Took",
  "Aragorn",
  "Legolas",
  "Gimli",
  "Boromir",
  "Sauron",
  "Gollum",
  "Bilbo Baggins",
  "Tom Bombadil",
  "Glorfindel",
  "Elrond",
  "Arwen Evenstar",
  "Galadriel",
  "Saruman the White",
  "Éomer",
  "Théoden",
  "Éowyn",
  "Grìma Wormtongue",
  "Shadowfax",
  "Treebeard",
  "Quickbeam",
  "Shelob",
  "Faramir",
  "Denethor",
  "Beregond",
  "Barliman Butterbur"
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static LOCATIONS: [&'static str; 85] = [
  "Aglarond",
  "Aldburg",
  "Andustar",
  "Angband",
  "Argonath",
  "Bag End",
  "Barad-dûr",
  "Black Gate",
  "Bridge of Khazad-dûm",
  "Carchost",
  "Cirith Ungol",
  "Coldfells",
  "Crack of Doom",
  "Dark Land",
  "Dol Guldur",
  "Dome of Stars",
  "Doors of Durin",
  "Doriath",
  "East Beleriand",
  "Eastfarthing",
  "East Road",
  "Eithel Sirion",
  "Elostirion",
  "Enchanted Isles",
  "Endless Stair",
  "Eä",
  "Falls of Rauros",
  "Fens of Serech",
  "Field of Celebrant",
  "Fords of Isen",
  "The Forsaken Inn",
  "Gap of Rohan",
  "Gladden Fields",
  "Gorgoroth",
  "Greenway",
  "Haudh-en-Nirnaeth",
  "Haven of the Eldar",
  "Helm's Deep",
  "Henneth Annûn",
  "Hobbit-hole",
  "Houses of Healing",
  "Hyarnustar",
  "Ilmen",
  "Inn of the Prancing Pony",
  "Isengard",
  "Isenmouthe",
  "Isle of Balar",
  "Land of the Sun",
  "Losgar",
  "Luthany",
  "Lothlorièn",
  "Maglor's Gap",
  "Marish",
  "Meduseld",
  "Minas Tirith",
  "Minhiriath",
  "Máhanaxar",
  "Narchost",
  "Nargothrond",
  "Núath",
  "Old Ford",
  "Old Forest",
  "Old Forest Road",
  "Orthanc",
  "Parth Galen",
  "Paths of the Dead",
  "Pelennor Fields",
  "Rath Dínen",
  "Regions of the Shire",
  "Rivendell",
  "The Rivers and Beacon-Hills of Gondor",
  "Sarn Ford",
  "Taur-en-Faroth",
  "Taur-im-Duinath",
  "Timeless Halls",
  "Tol Brandir",
  "Tol Galen",
  "Tol Morwen",
  "Tol-in-Gaurhoth",
  "Tumladen",
  "Utumno",
  "Vaiya",
  "Vista",
  "The Void",
  "Warning beacons of Gondor"
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 54] = [
  "Often does hatred hurt itself!",
  "Go now, and die in what way seems best to you.",
  "Memory is not what the heart desires. That is only a mirror....",
  "Dwarves’ tongues run on when speaking of their handiwork, they say.",
  "But all’s well as ends well",
  "Sing, all ye children of the West",
  "\"...and all the stars flowered in the sky.\"",
  "Clothes are but little loss, if you escape from drowning.",
  "The wolf that one hears is worse than the orc that one fears. - Boromir",
  "I don't keep water in my pockets.",
  "\"For he gave it up in the end of his own accord: an important point. No,\"",
  "I could not \"make' you--except by force, which would break your mind.",
  "Their faces were as a rule good-natured rather than beautiful.",
  "The Lord of the Ringwraiths had met his doom.",
  "Many that live deserve death. And some that die deserve life",
  "Short cuts make delays, but inns make longer ones.",
  "But where the warg howls, there also the orc prowls. - Aragorn",
  "In winter here no heart could mourn for summer or for spring.",
  "his old life lay behind in the mists, dark adventure lay in front.",
  "I sang of leaves, of leaves of gold, and leaves of gold there grew",
  "Someone else always has to carry on the story.",
  "It's the job that's never started as takes longest to finish.",
  "Moonlight drowns out all but the brightest stars.",
  "Fear nothing! Have peace until the morning! Heed no nightly noises!",
  "I’ll get there, if I leave everything but my bones behind,",
  "NO ADMITTANCE EXCEPT ON PARTY BUSINESS.",
  "Real names tell you the story of the things they belong to",
  "Deeds will not be less valiant because they are unpraised.",
  "Even the smallest person can change the course of the future.",
  "Who knows? Have patience. Go where you must go, and hope!",
  "Yet dawn is ever the hope of men,’ said Aragorn.",
  "You will soon be well, if I do not talk you to death.",
  "Memory is not what the heart desires. That is only a mirror,",
  "You have shown your usual cunning in getting up just in time for a meal.",
  "The great storm is coming, but the tide has turned.",
  "\"...as young and as ancient as Spring....\"",
  "Why couldn’t he stop talking and let them drink his health?",
  "Not all those who wander are lost",
  "\"But in this at least thou shalt not defy my will: to rule my own end.\"",
  "I look foul and feel fair.",
  "It's the job that's never started as takes longest to finish.",
  "Moonlight drowns out all but the brightest stars.",
  "I didn't think it would end this way.",
  "Home is behind, the world ahead",
  "All's well that ends better.",
  "I want to be a healer, and love all things that grow and are not barren.",
  "Where there's life there's hope, and need of vittles.",
  "A hunted man sometimes wearies of distrust and longs for friendship.",
  "The leaves were long, the grass was green",
  "Begone, foul dwimmerlaik, lord of carrion! Leave the dead in peace!",
  "The world is indeed full of peril and in it there are many dark places.",
  "To the sea, to the sea! The white gulls are crying",
  "Do not scorn pity that is the gift of a gentle heart, Éowyn!",
  "I wish the Ring had never come to me. I wish none of this had happened."
];
static QUOTES_LEN: usize = QUOTES.len();