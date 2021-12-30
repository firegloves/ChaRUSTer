// Character
struct Character {
    name: String,
    surname: String,
    nickname: String,
    description: String,
    hobbies: Vec<String>,
    image: String,
    collection: String,
    profession: String,
    props: Vec<Property>,
    levels: Vec<Level>,
    stats: Vec<Stat>,
    quirks: Vec<String>
}

// Property
struct Property {
    prop_type: String,
    name: String
}

// Level
struct Level {
    name: String,
    value: i32,
    max_value: i32
}

// Stat
struct Stat {
    name: String,
    value: i32,
    max_value: i32
}
