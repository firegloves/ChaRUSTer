use std::borrow::BorrowMut;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::marker::PhantomData;
use rand::Rng;
use rand::rngs::ThreadRng;

use crate::character;
use crate::character::{CharacterBuilder, CharacterFeature, Charuster, Level, Property, Stat, Quirk};
use crate::config::Config;
use crate::dictionary;
use crate::dictionary::{Dictionary, SimpleDictionary, TwoLevelsDictionary};

type FnCharFeatPropCreator = Box<dyn Fn(String) -> Option<character::CharacterFeature>>;
type FnCharFeatVecPropCreator = Box<dyn Fn(Vec<String>) -> Option<character::CharacterFeature>>;
type FnCharFeatVecQuirkCreator<T> = Box<dyn Fn(Vec<T>) -> Option<character::CharacterFeature>>;
type FnQuirkCreator<T> = Box<dyn Fn(&mut dyn Dictionary) -> T>;

fn export_to_json(charusters: Vec<Charuster>, filename: &str) {
    let json = serde_json::to_string(&charusters).unwrap();
    let d = "ciao";

    let write = OpenOptions::new().write(true).create(true).open(filename);
    let mut reader = BufReader::new(json.as_bytes());
    let mut writer = BufWriter::new(write.unwrap());

    let mut length = 1;

    while length > 0 {
        let buffer = reader.fill_buf().unwrap();

        writer.write(buffer);

        length = buffer.len();
        reader.consume(length);
    }
}

fn generate_charusters(config: &Config) -> Vec<Charuster> {

    let mut charusters = vec![];
    let mut generators = create_generators(config);
    let char_len = config.execution_conf.char_nums;
    for i in (0..char_len).into_iter() {
        let mut builder = CharacterBuilder::new();
        let gen_len = generators.len();
        for i in (0..gen_len).into_iter() {
            let gen = generators.get_mut(i).unwrap();
            let char_feature = gen.generate().unwrap();
            match char_feature {
                CharacterFeature::NAME(value) => &builder.name(value),
                CharacterFeature::SURNAME(value) => &builder.surname(value),
                CharacterFeature::NICKNAME(value) => &builder.nickname(value),
                CharacterFeature::DESCRIPTION(value) => &builder.description(value),
                CharacterFeature::IMAGE(value) => &builder.image(value),
                CharacterFeature::COLLECTION(value) => &builder.collection(value),
                CharacterFeature::PROFESSION(value) => &builder.profession(value),
                CharacterFeature::HOBBIES(values) => &builder.hobbies(values),
                CharacterFeature::PROPS(values) => &builder.props(values),
                CharacterFeature::LEVELS(values) => &builder.levels(values),
                CharacterFeature::STATS(values) => &builder.stats(values),
            };
        }
        let charuster = builder.build();
        charusters.push(charuster);
    }
    charusters
}


fn create_generators(config: &Config) -> Vec<Box<dyn FeatureGenerator>> {
    let mut generators: Vec<Box<dyn FeatureGenerator>> = vec![];

    if config.char_conf.gen_name && !config.values_conf.names_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.names_file.as_str());
        let mut generator = ChooseGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::NAME(v.clone()))));
        let boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_surname && !config.values_conf.surnames_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.surnames_file.as_str());
        let mut generator = ChooseGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::SURNAME(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_nickname && !config.values_conf.nicknames_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.nicknames_file.as_str());
        let mut generator = ChooseAndRemoveGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::NICKNAME(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_description && !config.values_conf.description_files.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.description_files.as_str());
        let mut generator = ChooseAndRemoveGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::DESCRIPTION(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_image && !config.values_conf.images_folder.is_empty() {
        let dict = SimpleDictionary::new_from_folder(config.values_conf.images_folder.as_str());
        let mut generator = ChooseAndRemoveGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::IMAGE(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_collection {
        let dict = SimpleDictionary::new_with_single_term(String::from("collection"), config.values_conf.collection_name.clone());
        let mut generator = ChooseGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::COLLECTION(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_profession && !config.values_conf.professions_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.professions_file.as_str());
        let mut generator = ChooseGenerator::new(dict, Box::new(|v: String| Some(CharacterFeature::PROFESSION(v.clone()))));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    // TODO add num of desired item in vecs
    if config.char_conf.gen_hobbies && !config.values_conf.hobbies_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.hobbies_file.as_str());
        let mut generator = ChooseVecGenerator::new(dict, Box::new(|v: Vec<String>| Some(CharacterFeature::HOBBIES(v))), 3);
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_props && !config.values_conf.props_file.is_empty() {
        let dict = TwoLevelsDictionary::new(config.values_conf.props_file.as_str());
        let mut generator = ChooseVecQuirkGenerator::new(Box::new(dict), Box::new(|v: Vec<Property>| Some(CharacterFeature::PROPS(v))), 3,
                                                     Box::new(|dict: &mut dyn Dictionary| {
                                                         let term = dict.choose_and_remove().unwrap();
                                                         let taxonomy : Vec<&str> = term.split("+").collect();
                                                         character::Property {
                                                             prop_type: String::from(taxonomy.get(0).unwrap().to_owned()),
                                                             name: String::from(taxonomy.get(1).unwrap().to_owned())
                                                         }
                                                     }));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_levels && !config.values_conf.levels_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.levels_file.as_str());
        let mut generator = ChooseVecQuirkGenerator::new(Box::new(dict), Box::new(|v: Vec<Level>| Some(CharacterFeature::LEVELS(v))), 3,
                                                     Box::new(|dict: &mut dyn Dictionary| {
                                                         character::Level {
                                                             name: dict.choose_and_remove().unwrap(),
                                                             value: rand::thread_rng().gen_range(1..=100),
                                                             max_value: 100
                                                         }
                                                     }));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }
    if config.char_conf.gen_stats && !config.values_conf.stats_file.is_empty() {
        let dict = SimpleDictionary::new(config.values_conf.stats_file.as_str());
        let mut generator = ChooseVecQuirkGenerator::new(Box::new(dict), Box::new(|v: Vec<Stat>| Some(CharacterFeature::STATS(v))), 3,
                                                     Box::new(|dict: &mut dyn Dictionary| {
                                                         character::Stat {
                                                             name: dict.choose_and_remove().unwrap(),
                                                             value: rand::thread_rng().gen_range(1..=100),
                                                             max_value: 100
                                                         }
                                                     }));
        let mut boxxx = Box::new(generator);
        generators.push(boxxx);
    }

    generators
}

/** GENERATORS **/
trait FeatureGenerator {
    fn generate(&mut self) -> Option<character::CharacterFeature>;
}

// ChooseGenerator
struct ChooseGenerator {
    dict: SimpleDictionary,
    fn_char_feat_creator: FnCharFeatPropCreator,
}

impl ChooseGenerator {
    fn new(dict: SimpleDictionary, fn_char_feat_creator: FnCharFeatPropCreator) -> ChooseGenerator {
        ChooseGenerator { dict, fn_char_feat_creator }
    }
}

impl FeatureGenerator for ChooseGenerator {
    fn generate(&mut self) -> Option<character::CharacterFeature> {
        let value = self.dict.choose()?;
        (self.fn_char_feat_creator)(value.clone())
    }
}

// ChooseAndRemoveGenerator
struct ChooseAndRemoveGenerator {
    dict: SimpleDictionary,
    fn_char_feat_creator: FnCharFeatPropCreator,
}

impl ChooseAndRemoveGenerator {
    fn new(dict: SimpleDictionary, fn_char_feat_creator: FnCharFeatPropCreator) -> ChooseAndRemoveGenerator {
        ChooseAndRemoveGenerator { dict, fn_char_feat_creator }
    }
}

impl FeatureGenerator for ChooseAndRemoveGenerator {
    fn generate(&mut self) -> Option<character::CharacterFeature> {
        let value = self.dict.choose_and_remove()?;
        (self.fn_char_feat_creator)(value.clone())
    }
}

// ChooseVecGenerator
struct ChooseVecGenerator {
    dict: SimpleDictionary,
    fn_char_feat_vec_creator: FnCharFeatVecPropCreator,
    vec_size: u8,
}

impl ChooseVecGenerator {
    fn new(dict: SimpleDictionary, fn_char_feat_vec_creator: FnCharFeatVecPropCreator, vec_size: u8) -> ChooseVecGenerator {
        ChooseVecGenerator { dict, fn_char_feat_vec_creator, vec_size }
    }
}

impl FeatureGenerator for ChooseVecGenerator {
    fn generate(&mut self) -> Option<character::CharacterFeature> {
        let mut feat_vec = vec![];
        for _ in (0..self.vec_size).into_iter() {
            let value = self.dict.choose()?;
            feat_vec.push(value.clone());
        }
        (self.fn_char_feat_vec_creator)(feat_vec)
    }
}

// ChooseVecQuirkGenerator
struct ChooseVecQuirkGenerator<T: character::Quirk> {
    dict: Box<dyn Dictionary>,
    fn_char_feat_vec_creator: FnCharFeatVecQuirkCreator<T>,
    fn_quirk_creator: FnQuirkCreator<T>,
    vec_size: u8,
}

impl<T: character::Quirk> ChooseVecQuirkGenerator<T> {
    fn new(dict: Box<dyn Dictionary>, fn_char_feat_vec_creator: FnCharFeatVecQuirkCreator<T>, vec_size: u8, fn_quirk_creator: FnQuirkCreator<T>) -> ChooseVecQuirkGenerator<T> {
        ChooseVecQuirkGenerator { dict, fn_char_feat_vec_creator, vec_size, fn_quirk_creator }
    }
}

impl<T> FeatureGenerator for ChooseVecQuirkGenerator<T>
where T: character::Quirk {
    fn generate(&mut self) -> Option<character::CharacterFeature> {
        let mut feat_vec = vec![];
        let mut cloned_dict = dyn_clone::clone_box(&*self.dict);
        for _ in (0..self.vec_size).into_iter() {
            let quirk = (self.fn_quirk_creator)(cloned_dict.as_mut());
            feat_vec.push(quirk);
        }
        (self.fn_char_feat_vec_creator)(feat_vec)
    }
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::character::CharacterFeature::NAME;
    use crate::character::CharacterPropTypes;
    use crate::config::parse_config;

    use super::*;

    fn get_test_dictionary_filename() -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/test_dictionary");
        d.into_os_string().into_string().unwrap()
    }

    #[test]
    fn test() {
        let config = parse_config();
        let charusters = generate_charusters(&config);
        export_to_json(charusters, "/Users/firegloves/Desktop/churusters.json")
    }
}
