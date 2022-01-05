pub enum CharacterPropTypes {
    String,
    VecString,
    VecProps,
    VecLevels,
    VecStats,
}

pub enum CharacterFeature {
    NAME(String),
    SURNAME(String),
    NICKNAME(String),
    DESCRIPTION(String),
    IMAGE(String),
    COLLECTION(String),
    PROFESSION(String),
    HOBBIES(Vec<String>),
    PROPS(Vec<Property>),
    LEVELS(Vec<Level>),
    STATS(Vec<Stat>),
}

pub struct Charuster {
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
}

impl Charuster {

    // This method will help users to discover the builder
    pub fn builder() -> CharacterBuilder {
        CharacterBuilder::default()
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn surname(&self) -> &String {
        &self.surname
    }
    pub fn nickname(&self) -> &String {
        &self.nickname
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn hobbies(&self) -> &Vec<String> {
        &self.hobbies
    }
    pub fn image(&self) -> &String {
        &self.image
    }
    pub fn collection(&self) -> &String {
        &self.collection
    }
    pub fn profession(&self) -> &String {
        &self.profession
    }
    pub fn props(&self) -> &Vec<Property> {
        &self.props
    }
    pub fn levels(&self) -> &Vec<Level> {
        &self.levels
    }
    pub fn stats(&self) -> &Vec<Stat> {
        &self.stats
    }
}

#[derive(Default)]
pub struct CharacterBuilder {
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
}

impl CharacterBuilder {
    pub fn new() -> Self {
        CharacterBuilder {
            name: "".to_string(),
            surname: "".to_string(),
            nickname: "".to_string(),
            description: "".to_string(),
            hobbies: vec![],
            image: "".to_string(),
            collection: "".to_string(),
            profession: "".to_string(),
            props: vec![],
            levels: vec![],
            stats: vec![],
        }
    }

    pub fn name(& mut self, name: String) -> &Self {
        self.name = name.to_string();
        self
    }

    pub fn surname(& mut self, surname: String) -> &Self {
        self.surname = surname.to_string();
        self
    }

    pub fn nickname(& mut self, nickname: String) -> &Self {
        self.nickname = nickname.to_string();
        self
    }

    pub fn description(& mut self, description: String) -> &Self {
        self.description = description.to_string();
        self
    }

    pub fn hobbies(& mut self, hobbies: Vec<String>) -> &Self {
        self.hobbies = hobbies;
        self
    }

    pub fn image(& mut self, image: String) -> &Self {
        self.image = image.to_string();
        self
    }

    pub fn collection(& mut self, collection: String) -> &Self {
        self.collection = collection.to_string();
        self
    }

    pub fn profession(& mut self, profession: String) -> &Self {
        self.profession = profession.to_string();
        self
    }

    pub fn props(& mut self, props: Vec<Property>) -> &Self {
        self.props = props;
        self
    }

    pub fn levels(& mut self, levels: Vec<Level>) -> &Self {
        self.levels = levels;
        self
    }

    pub fn stats(& mut self, stats: Vec<Stat>) -> &Self {
        self.stats = stats;
        self
    }



    pub fn build(self) -> Charuster {
        Charuster {
            name: self.name,
            surname: self.surname,
            nickname: self.nickname,
            description: self.description,
            hobbies: self.hobbies,
            image: self.image,
            collection: self.collection,
            profession: self.profession,
            props: self.props,
            levels: self.levels,
            stats: self.stats,
        }
    }
}

pub trait Quirk {}

// Property
pub struct Property {
    // TODO creare new fn e togliere pub
    pub prop_type: String,
    pub name: String
}
impl Quirk for Property {}

// Level
pub struct Level {
    // TODO creare new fn e togliere pub
    pub name: String,
    pub value: i32,
    pub max_value: i32
}
impl Quirk for Level {}

// Stat
pub struct Stat {
    pub name: String,
    pub value: i32,
    pub max_value: i32,
}
impl Quirk for Stat {}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::character::CharacterPropTypes;
    use super::*;

    #[test]
    fn test() {
        CharacterFeature::NAME(String::from("CIA"));
    }
}