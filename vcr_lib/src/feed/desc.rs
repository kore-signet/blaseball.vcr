pub enum EventDescription {
    Constant(&'static str),
    ConstantVariant(Vec<&'static str>),
    Prefix(&'static str),
    Suffix(&'static str),
    Variable,
}

impl EventDescription {
    pub fn from_type(t: i16) -> EventDescription {
        use EventDescription::*;

        match t {
            1 => Constant("Play ball!"),
            25 => Constant("The Electricity zaps a strike away!"),
            73 => ConstantVariant(vec![
                "A desolate peanutty wind blows.",
                "A solitary peanut rolls onto the field. Nobody cares.",
                "Peanut fragments rustle on the infield.",
                "The faint crunch of a shell underfoot",
                "The sour smell of rancid peanuts on the wind",
            ]),
            77 => Constant("The Event Horizon awaits."),
            21 => Suffix("apply Home Field advantage!"),
            33 => Constant("The Birds circle ... but they don't find what they're looking for."),
            45 => Suffix(" swallowed a stray peanut and had a Superallergic reaction!"),
            47 => Suffix(" swallowed a stray peanut and had an allergic reaction!"),
            62 => Prefix(
                "A surge of Immateria rushes up from Under!\nBaserunners are swept from play!",
            ),
            65 => Suffix("enters the Secret Base..."),
            69 => Prefix("The Echo Chamber traps a wave.\n"),
            72 => Prefix("The Peanut Mister activates!\n"),
            76 => Prefix("The Event Horizon activates!\n"),
            78 => Constant("The Solar Panels are angled toward Sun 2."),
            79 => Prefix("The Solar Panels absorb Sun 2's energy!\n"),
            88 => Constant("The Atlantis Georgias go Undersea. They're now Overperforming!"),
            125 => Suffix("entered the Hall of Flame."),
            131 => Suffix("had their lineup shuffled."),
            137 => Suffix("has been hatched from the field of eggs."),
            189 => Prefix("The Community Chest Opens!\n"),
            192 => Prefix("Hotel Motel\nInning "),
            193 => Prefix("Prize Match!\nThe Winner gets "),
            195 => Prefix("Smithy beckons to "),
            206 => Prefix("Hype built in "),
            216 => Constant("Game Over."),
            217 => ConstantVariant(vec!["Sun(Sun)'s Pressure built...", "Sun(Sun) Recharged."]),
            228 => Prefix("Home Team Shutout.\nIncoming Voicemail...\n"),
            _ => Variable,
        }
    }
}
