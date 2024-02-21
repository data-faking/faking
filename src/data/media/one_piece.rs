use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn one_piece_characters() -> String {
	let mut rng = rand::thread_rng();
	CHARACTER_NAMES[rng.gen_range(0..CHARACTER_NAMES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn one_piece_seas() -> String {
	let mut rng = rand::thread_rng();
	SEAS[rng.gen_range(0..SEAS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn one_piece_islands() -> String {
	let mut rng = rand::thread_rng();
	ISLANDS[rng.gen_range(0..ISLANDS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn one_piece_locations() -> String {
	let mut rng = rand::thread_rng();
	LOCATIONS[rng.gen_range(0..LOCATIONS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn one_piece_quotes() -> String {
	let mut rng = rand::thread_rng();
	QUOTES[rng.gen_range(0..QUOTES_LEN)].to_string()
}

static CHARACTER_NAMES: [&'static str; 111] = [
	"Monkey D. Luffy",
	"Roronoa Zoro",
	"Nami",
	"Ussop",
	"Vinsmoke Sanji",
	"Tony Tony Chopper",
	"Nico Robin",
	"Franky",
	"Brook",
	"Akainu",
	"Aokiji",
	"Arlong",
	"Bartholomew Kuma",
	"Boa Hancock",
	"Caesar Clown",
	"Coby",
	"Crocodile",
	"Kuroken Mr. 1",
	"Dracule Mihawk",
	"Edward Newgate",
	"Emporio Ivankov",
	"Gecko Moriah",
	"Jinbe",
	"Kaido",
	"Kalifa",
	"Kizaru",
	"Marshall D. Teach",
	"Mokey D. Dragon",
	"Monkey D. Garp",
	"Portgas D. Ace",
	"Rob Lucci",
	"Sengoku",
	"Shanks",
	"Smoker",
	"Tashigi",
	"Trafalgar D. Water Law",
	"Alvida",
	"Baby Five",
	"Bartolomeo",
	"Basil Hawkins",
	"Bastille",
	"Bellamy",
	"Ben Beckmann",
	"Bepo",
	"Blueno",
	"Bon Clay Mr. 2",
	"Brook",
	"Bufalo",
	"Buggy",
	"Capone Bege",
	"Cavendish",
	"Hakuba",
	"Dellinger",
	"Diamante",
	"Doc Q",
	"Don Chinjao",
	"Don Krieg",
	"Donquixote Doflamingo",
	"Enel",
	"Eustass Kid",
	"Fujitora",
	"Fukuro",
	"Fullbody",
	"Gladius",
	"Gold D. Roger",
	"Hacchi",
	"Hannyabal",
	"Hody Jones",
	"Jyabura",
	"Jesus Burgess",
	"Jewelry Bonney",
	"Jozu",
	"Kaku",
	"Kaime",
	"Killer",
	"Kinemon",
	"Koala",
	"Kumadori",
	"Kyros",
	"Laboon",
	"Laffitte",
	"Lao G",
	"Leo",
	"Lucy",
	"Magellan",
	"Marco",
	"Miss Valentine",
	"Momonosuke",
	"Money",
	"Nojiko",
	"Perona",
	"Rebecca",
	"Ryuma",
	"Sabo",
	"Sadi",
	"Scratchmen Apoo",
	"Sengoku",
	"Señor Pink",
	"Sentoumaru",
	"Shirahoshi",
	"Silvers Rayleigh",
	"Sogeking",
	"Sugar",
	"Spandam",
	"Van Auger",
	"Vergo",
	"Vista",
	"Vivi",
	"X Drake",
	"Corazon",
	"Pika"
];
static CHARACTER_NAMES_LEN: usize = CHARACTER_NAMES.len();

static SEAS: [&'static str; 6] = [
	"East Blue",
	"West Blue",
	"North Blue",
	"South Blue",
	"Grand Line",
	"All Blue"
];
static SEAS_LEN: usize = SEAS.len();

static ISLANDS: [&'static str; 95] = [
	"Dawn Island",
	"Goat Island",
	"Shells Town",
	"Organ Islands",
	"Island of Rare Animals",
	"Gecko Islands",
	"Conomi Islands",
	"Loguetown",
	"Kumate Island",
	"Mirrorball Island",
	"Tequila Wolf",
	"Cozia",
	"Ohara",
	"Ilusia",
	"Thriller Bark",
	"Toroa",
	"Las Camp",
	"Kano Country",
	"Germa Kingdom",
	"Lvneel Kingdom",
	"Micqueot",
	"Spider Miles",
	"Flevance",
	"Rubeck Island",
	"Swallow Island",
	"Minion Island",
	"Rakesh",
	"Notice",
	"Briss Kingdom",
	"Karate Island",
	"Centaurea",
	"Torino Kingdom",
	"Baterilla",
	"Black Drum Kingdom",
	"Fishman Island",
	"Amazon Lily",
	"Impel Down",
	"Rasukaina",
	"Cactus Island",
	"Little Garden",
	"Holliday Island",
	"Drum Island",
	"Alabasta",
	"Nanimonai Island",
	"Jaya",
	"Long Ring Long Land",
	"Water 7",
	"Enies Lobby",
	"San Faldo",
	"Pucci",
	"St. Poplar",
	"Florian Triangle",
	"Sabaody Archipelago",
	"Marineford",
	"Vira",
	"Banaro Island",
	"Yuki's Island",
	"Buggy's Treasure Island",
	"G-2",
	"Karakuri Island",
	"Mamoiro Island",
	"Boin Archipelago",
	"Namakura Island",
	"Kuraigana Island",
	"Merveille",
	"G-1",
	"Yukiryu Island",
	"Baltigo",
	"Wano Country",
	"Edd War",
	"Floodvalter",
	"G-5",
	"Laftel",
	"Whole Cake Island",
	"Cacao Island",
	"Jam Island",
	"Nuts Island",
	"Cheese Island",
	"Biscuits Island",
	"Candy Island",
	"Milk Island",
	"Punk Hazard",
	"Raijin Island",
	"Risky Red Island",
	"Mystoria Island",
	"Dressrosa",
	"Green Bit",
	"Zou",
	"Prodence Kingdom",
	"Applenine Island",
	"Karai Bari Island",
	"Broc Coli Island",
	"Elbaf",
	"Skypiea",
	"Weatheria"
];
static ISLANDS_LEN: usize = ISLANDS.len();

static LOCATIONS: [&'static str; 51] = [
	"Foosha Village",
	"Mt. Colubo",
	"Gray Terminal",
	"Midway Forest",
	"Goa Kingdom",
	"Orange Town",
	"Syrup Village",
	"Shimotsuki Village",
	"Baratie",
	"Gosa Village",
	"Cocoyashi Village",
	"Arlong Park",
	"Ryugu Kingdom",
	"Reverse Mountain",
	"Twin Cape",
	"Mariejois",
	"Whiskey Peak",
	"Bighorn",
	"Drum Rockies",
	"Cocoa Weed",
	"Gyasta",
	"Robelie",
	"Sandora Desert",
	"Sandora River",
	"Rainbase",
	"Yuba",
	"Erumalu",
	"Nanohana",
	"Katorea",
	"Spiders Cafe",
	"Alubarna",
	"Tamarisk",
	"Suiren",
	"Mock Town",
	"Sea Train Area",
	"Totto Land",
	"Acacia",
	"Sebio",
	"Moon",
	"Birka",
	"Angel Island",
	"Upper Yard",
	"Shandia Village",
	"Heaven's Gate",
	"Clouds End",
	"Rommel Kingdom",
	"Eight Nine Island",
	"High Mountain",
	"Nakrowa",
	"Land of Ice",
	"Great Kingdom"
];
static LOCATIONS_LEN: usize = LOCATIONS.len();

static QUOTES: [&'static str; 99] = [
	"\"This thing called Justice…changes its shape depending on where you stand.\" - Kuzan (Aokiji)",
	"\"I love heroes, but I don't want to be one. Do you even know what a hero is!? For example, you have some meat. Pirates will feast on the meat, but the hero will distribute it among the people! I want to eat the meat!\" - Monkey D. Luffy",
	"\"\"Loneliness\" is no longer part of my vocabulary.\" - Brook",
	"\"Find yourself and your true world. Become stronger! No matter how long it takes, I will await you in my strongest form. Surpass this sword! Surpass me!\" - Dracule Mihawk to Roronoa Zoro",
	"\"The government says your existence is a crime, but no matter what kind of weapons you may hold, just being alive isn't a sin! There's no crime in living!\" - Franky",
	"\"You can't bring back what you've lost, think about what you have now!\" - Jinbe",
	"\"Don't ever think there's any perfect society made by humans!! If you think that way you'll overlook the enemy!! Don't be fooled by appearances!!\" - Smoker",
	"\"It may be hard right now… But you must silence those thoughts! Stop counting only those things that you have lost! What is gone, is gone!\" - Jinbei",
	"\"I don't care now. I wanted to look like a human because I wanted friends. Now I want to be a monster who's helpful to Luffy!\" - Chopper",
	"\"People don't fear god, fear itself is god\" - Enel",
	"\"If I can't even protect my captain's dream, then whatever ambition I have is nothing but talk! Luffy must be the man who becomes the Pirate King!\" - Roronoa Zoro",
	"\"Death is never an apology.\" - Brook",
	"\"There comes a time when a man has to stand and fight! That time is when his friends' dreams are being laughed at! And I won't let you laugh at that!\" - Usopp",
	"\"Don't ever think there's any perfect society made by humans!! If you think that way you'll overlook the enemy!! Don't be fooled by appearances!!\" - Smoker",
	"\"Old man, everyone!! And you.. Luffy. Even though I've been good for nothing my whole life, even though I have the blood of a demon within me… You guys still loved me! Thank you so much!!\" - Portgas D. Ace",
	"\"If you lose credibility by just admitting fault, then you didn't have any in the first place.\" - Fujitora",
	"\"Man or Child, Strong or Weak, None of those matters once you are out at sea!\" - Usopp",
	"\"Compared to the \"righteous\" greed of the rulers, the criminals of the world seem much more honorable. When scum rules the world, only more scum is born.\" - Eustass Kid",
	"\"What do you know of death? Have you ever died? You think death will preserve your cause forever? Ridiculous! Death leaves nothing behind! Once a person passes on, nothing remains but dead bones. If there is one thing I can't stand, it is a person with no respect for life.\" - Brook",
	"\"Then stand up right away! And don't act like you're about to die! It's not like you!\" - Usopp",
	"\"The weak don't get to decide anything, not even how they die.\" - Trafalgar D. Water Law",
	"\"Stop counting only those things you have lost! What is gone, is gone! So ask yourself this. What is there… that still remains to you?!\" - Jinbe",
	"\"Protecting what we cherish most as men is the reason why we formed this pirate crew!\" - Usopp",
	"\"Miracles only happen to those who never give up.\" - Emporio Ivankov",
	"\"You sure can talk the talk, but you're not quite ready to walk the walk. Time's up, it's my turn.\" - Roronoa Zoro",
	"\"When do you think people die? When they are shot through the heart by the bullet of a pistol? No. When they are ravaged by an incurable disease? No… It's when they're forgotten!\" - Hiluluk Doctor",
	"\"You need to accept the fact that you're not the best and have all the will to strive to be better than anyone you face.\" - Roronoa Zoro",
	"\"It's not some sort of special power. He has the ability to make allies of everyone he meets. And that is the most fearsome ability on the high seas.\" - Dracule Mihawk (Hawk-Eyes)",
	"\"Life is like a pencil that will surely run out, but will leave the beautiful writing of life.\" - Nami",
	"\"Men who can't wipe away the tears from a women's eyes, aren't real men.\" - Sanji Vinsmoke",
	"\"You can spill drinks on me, even spit on me. I'll just laugh about it. But If you dare to hurt my friends… I won't forgive you!\" - Shanks",
	"\"The time is almost here. Go and lay the groundwork. A world where only true pirates can survive will soon be upon us. Those without power, flee while you can.\" - Donquixote Doflamingo",
	"\"Only those who have suffered long, can see the light within the shadows.\" - Roronoa Zoro",
	"\"What good is a treasure if I am alone?\" - Nami",
	"\"Maybe nothing in this world happens by accident. As everything happens for a reason, our destiny slowly takes form.\" - Silvers Rayleigh",
	"\"It is a sad truth that greater the authority a person possesses, the more he tends to fear change.\" - Jinbe",
	"\"Being strong isn't just about having power or move, it about one's spirit.\" - Roronoa Zoro",
	"\"Bring on the hardship. It's preferred in a path of carnage.\" - Roronoa Zoro",
	"\"Sometimes the only thing you have to doubt is your own common sense.\" - Nico Robin",
	"\"ONE PIECE IS REAL!\" - Edward \"Whitebeard\" Newgate",
	"\"Either in belief or doubt, if I lean to one of these sides, my reaction time will be dulled if my heart thinks the opposite of what I choose.\" - Roronoa Zoro",
	"\"If I lose to someone as pitiful as you, with such a small injury… my fate is already sealed.\" - Roronoa Zoro",
	"\"Fools who don't respect the past are likely to repeat it.\" - Nico Robin",
	"\"We have to live a life of no regrets.\" - Portgas D. Ace",
	"\"When the world shoves you around, you just gotta stand up and shove back. It's not like somebody's gonna save you if you start babbling excuses.\" - Roronoa Zoro",
	"\"Before the heart of truth, there is no need for words.\" - Nico Robin",
	"\"I want to live!\" - Robin Nico",
	"\"When you decided to go to the sea, it was your own decision. Whatever happens to you on the sea, it depends on what you've done! Don't blame others!\" - Roronoa Zoro",
	"\"By experiencing both victory and defeat, running away and shedding tears, a man will become a man. It's okay to cry, but you have to move on.\" - Shanks",
	"\"Once a person passes on, nothing remains but dead bones! If there is one thing I can't stand, it is a person with no respect for life!\" - Brook",
	"\"When I decided to follow my dream, I had already discarded my life.\" - Roronoa Zoro",
	"\"What keeps me alive in this world is neither bodily organs nor muscles, it's my soul.\" - Brook",
	"\"No one is born into this world to be alone\" - Jaguar D. Saul",
	"\"Food is a gift from god. Spices are a gift from the devil. It looks like it was a bit too spicy for you.\" - Sanji",
	"\"My wealth and treasures? If you want it, I'll let you have it. Look for it, I left all of it at that place!\" - Gol D. Roger",
	"\"I do things my own way! So don't give me any lip about it!\" - Roronoa Zoro",
	"\"There is someone that I must meet again. And until that day…not even Death itself can take my life away!\" - Roronoa Zoro",
	"\"Neither God nor the Devil can give aid to those without the will to fight…\" - Brook",
	"\"If you don't take risks, you can't create a future.\" - Monkey D. Luffy",
	"\"Seek freedom, and it will stretched out before your eyes. If the endless dream guides your restless spirit, seize it! Raise your flag, and stand tall!\" - Gol D. Roger",
	"\"Don't try to find a reason for somebody's love.\" - Sengoku The Buddha",
	"\"All for one, one for all! There are times when a man has no choice but to act!\" - Brook",
	"\"It doesn't matter who your parents were. Everyone is a child of the sea.\" - Edward Newgate",
	"\"To true friendship, how long you've known each other means nothing.\" - Bentham aka Bon Clay (One Piece)",
	"\"No matter how deep the night, it always turns to day, eventually.\" - Brook",
	"\"Someday, I promise you, they will all watch as I change the world.\" - Monkey D. Dragon",
	"\"Pirates are evil? The Marines are righteous?… Justice will prevail, you say? But of course it will! Whoever wins this war becomes justice!\" - Donquixote Doflamingo",
	"\"There is a difference in the look of the ordinary vagabond and that of a determined man.\" - Doflamingo",
	"\"I'm going to be the world's greatest swordsman! All I have left is my destiny! My name may be infamous…but it's gonna shake the world!\" - Roronoa Zoro",
	"\"There can be no happiness in a world where the undesirables are thrown away.\" - Monkey D. Dragon",
	"\"If you want to protect something, do it right! Don't let them get their way anymore!\" - Crocodile",
	"\"I lost my mother when I was 8…and killed my father when I was 10. Those with the title of Executive\" or above…are my \"family\", with whom I've shared my joys and sorrows. They're the only thing I have! I won't forgive anyone that laughs at my family. Understood?\" - Doflamingo",
	"\"Even in the depths of Hell… blooms a beautiful flower of friendship… leaving its petals as mementos… bobbing back and forth on the waves… may it one day bloom once more…the Okama Way.\" - Bentham aka Bon Clay",
	"\"A true pirate should fear nothing, not even death!\" - Gecko Moria",
	"\"The royalty and nobles are behind the fire… Believe me… This town smells worse than Gray Terminal. It smells like rotten people! If I stay here… I'll never be free! I'm… ashamed to be born a noble!\" - Sabo",
	"\"Believe me… This town smells worse than Gray Terminal. It smells like rotten people! If I stay here… I'll never be free! I'm… ashamed to be born a noble!\" - Sabo",
	"\"Destiny. Fate. Dreams. These unstoppable ideas are held deep in the heart of man. As long as there are people who seek freedom in this life, these things shall not vanish from the Earth.\" - Gold D. Rodger",
	"\"No matter what happens, don't be sorry you were born, don't forget to smile in any situation. As long as you're alive there will be better things later…and there will be many.\" - Bellemere",
	"\"Any person who hurts Nami-san's feelings! I won't let them live peacefully!\" - Sanji",
	"\"A scar on the back is a shame for a swordsman.\" - Roronoa Zoro",
	"\"Fine! Go see for yourself, Straw Hat… What a real nightmare is… In the New World!\" - Gecko Moria",
	"\"Inherited Will, the Destiny of the Age, and the Dreams of its People. These are things that will not be stopped. As long as people continue to pursue the meaning of freedom, these things will never cease to be!\" - Gol D. Roger",
	"\"There are things you can't see unless you change your standing.\" - Trafalgar Law",
	"\"This pain is nothing when compared to the pain I've received in the past 12 years.\" - Kyros",
	"\"I don't care if you're a god. If you lay even one finger on Nami-san, I'll become the devil of the blue sea!\" - Sanji",
	"\"You can't see the whole picture until you look at it from the outside.\" - Trafalgar Law",
	"\"Ideals can only be spoken by those powerful enough to carry them out.\" - Crocodile",
	"\"People's dreams… Never end!\" - Teach Marshall D. (Blackbeard)",
	"\"I'll die smiling! Because if you ever think of me in the future, I want you to remember me smiling.\" - Donquixote Rosinante",
	"\"Wake up princess, I'm tired of your useless ideals. It's gotten pathetic. What good are your happy ideals if you can't do anything to make them a reality? They're nothing but dreams, and your dreams don't stand a chance.\" - Sir Crocodile",
	"\"I couldn't confess my feelings for you, so I watched you from afar, being happy with someone else.\" - Sanji",
	"\"If you kill yourself, I'll kill you!\" - Zoro Roronoa",
	"\"Don't start a fight if you can't end it.\" - Sanji",
	"\"I don't care what the society says. I've never regretted doing anything. I will survive and do what I want to.\" - Roronoa Zoro",
	"\"I don't wanna live a thousand years. If I just live through today, that'll be enough.\" - Ace Portgas D.",
	"\"A real man is someone who forgives a woman for her lies!\" - Sanji",
	"\"The world has too many heroes. It needs a monster.\" - Dracule Mihawk",
	"\"When a guy gets flustered, because someone calls him weak, it's proof that he recognizes he is weak. Let the outcome determine the weak and the strong.\" - Don Krieg",
	"\"I don't remember the name of every weakling I crush.\" - Dracule Mihawk",
];
static QUOTES_LEN: usize = QUOTES.len();
