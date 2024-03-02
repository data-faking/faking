use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn seinfeld_characters() -> String {
	let mut rng = rand::thread_rng();
	CHARACTERS[rng.gen_range(0..CHARACTERS_LEN)].to_string()
}

#[wasm_bindgen]
pub fn seinfeld_quotes() -> String {
	let mut rng = rand::thread_rng();
	QUOTES[rng.gen_range(0..QUOTES_LEN)].to_string()
}

#[wasm_bindgen]
pub fn seinfeld_businesses() -> String {
	let mut rng = rand::thread_rng();
	BUSINESSES[rng.gen_range(0..BUSINESSES_LEN)].to_string()
}

static CHARACTERS: [&'static str; 29] = [
  "George Costanza",
  "Kramer",
  "Elaine Benes",
  "Newman",
  "Jerry Seinfeld",
  "Frank Costanza",
  "Morty Seinfeld",
  "Estelle Costanza",
  "Susan Ross",
  "Helen Seinfeld",
  "J Peterman",
  "Uncle Leo",
  "David Puddy",
  "Justin Pitt",
  "Kenny Bania",
  "Crazy Joe Davola",
  "Jackie Chiles",
  "Jack Klompus",
  "Ruthie Cohen",
  "Tim Whatley",
  "Sue Ellen Mischke",
  "Bob Sacamano",
  "Babs Kramer",
  "Babu Bhatt",
  "George Steinbrenner",
  "Mickey Abbott",
  "Mr. Lippman",
  "Mr. Wilhelm",
  "Russell Dalrymple"
];
static CHARACTERS_LEN: usize = CHARACTERS.len();

static QUOTES: [&'static str; 55] = [
  "I'm not a lesbian. I hate men, but I'm not a lesbian.",
  "You're gonna over-dry your laundry.",
  "This isn't a good time.",
  "That’s the true spirit of Christmas; people being helped by people other than me.",
  "You’re becoming one of the glitterati.",
  "Father, I’ve never done this before, so I’m not sure about what I’m supposed to do.",
  "She’s one of those low-talkers. You can’t hear a word she’s saying!",
  "Why do they make the condom packets so hard to open?",
  "This woman hates me so much, I’m starting to like her.",
  "I've driven women to lesbianism before, but never a mental institution.",
  "You know I always wanted to pretend I was an architect",
  "Borrowing money from a friend is like having sex. It just completely changes the relationship.",
  "When you look annoyed all the time, people think that you're busy.",
  "I spend so much time trying to get their clothes off, I never thought of taking mine off.",
  "If you can't say something bad about a relationship, you shouldn't say anything at all.",
  "I need the secure packaging of Jockeys. My boys needs a house!",
  "The sea was angry that day, my friends, like an old man trying to send back soup in a deli...",
  "Elaine, breaking up is like knocking over a Coke machine. You can’t do it in one push; you gotta rock it back and forth a few times and then it goes over.",
  "Looking at cleavage is like looking at the sun. You don't stare at it. It's too risky. Ya get a sense of it and then you look away.",
  "You have the chicken, the hen, and the rooster. The chicken goes with the hen... So who is having sex with the rooster?",
  "I lie every second of the day. My whole life is a sham.",
  "Just remember, when you control the mail, you control... information.",
  "I don't think I've ever been to an appointment in my life where I wanted the other guy to show up.",
  "You, my friend, have crossed the line between man and bum.",
  "You should've seen her face. It was the exact same look my father gave me when I told him I wanted to be a ventriloquist.",
  "Did you know that the original title for War and Peace was War, What Is It Good For?",
  "Sex, that’s meaningless, I can understand that, but dinner; that’s heavy. That’s like an hour.",
  "Jerry, just remember, it's not a lie if you believe it.",
  "These pretzels are makin' me thirsty.",
  "It became very clear to me sitting out there today that every decision I've made in my entire life has been wrong. My life is the complete opposite of everything I want it to be. Every instinct I have, in every aspect of life, be it something to wear, something to eat - it's all been wrong.",
  "I had a dream last night that a hamburger was eating me.",
  "I have been performing feats of strength all morning.",
  "Hi, my name is George, I'm unemployed and I live with my parents.",
  "I don't trust the guy. I think he regifted, then he degifted, and now he's using an upstairs invite as a springboard to a Super bowl sex romp.",
  "Yes, I hope my parents die long before I do.",
  "See, this is what the holidays are all about. Three buddies sitting around chewing gum.",
  "Dolores!",
  "I'll be back. We'll make out.",
  "I'm sorry to bother you, but I'm a US postal worker and my mail truck was just ambushed by a band of backwoods mail-hating survivalists.",
  "You very bad man, Jerry. Very bad man.",
  "No soup for you!",
  "Serenity now!",
  "I'm out there Jerry, and I'm loving every minute of it!",
  "I'm out of the contest!",
  "You're killing independent George!",
  "Not that there's anything wrong with that.",
  "Yadda, yadda, yadda.",
  "They're real, and they're spectacular.",
  "She has man hands.",
  "And you want to be my latex salesman.",
  "He's a close talker.",
  "It's a Festivus for the rest of us.",
  "I want to be the one person who doesn't die with dignity.",
  "You, my friend, have crossed the line between man and bum.",
  "You were necking during Schindler's List?"
];
static QUOTES_LEN: usize = QUOTES.len();

static BUSINESSES: [&'static str; 23] = [
  "Champagne Video",
  "Joe's Fruit Shop",
  "Kruger Industrial Smoothing",
  "Vandelay Industries",
  "Kramerica Industries",
  "J. Peterman Catalog",
  "Tom's Restaurant",
  "Reggies",
  "Mendy's",
  "Sunshine Carpet Cleaners",
  "Brandt-Leland",
  "H and H Bagels",
  "Play Now",
  "Top of the Muffin to You!",
  "Oh Henry!",
  "P B and J's",
  "Poppie's",
  "Sagman, Bennett, Robbins, Oppenheim and Taft",
  "Pendant Publishing",
  "Dream Cafe",
  "Doubleday",
  "Tyler Chicken",
  "Royal Bakery"
];
static BUSINESSES_LEN: usize = BUSINESSES.len();