mod dictionary;
mod generator;
mod character;
mod testo;
pub mod config;

// struct Config {
//
// }

// type StringGenerationFn = Box<fn(config: Config) ->String>;
// type VecGenerationFn = Box<fn(config: Config) -> Vec<dyn Quirk>>;
// type StringSetterFn = Box<fn(value: String, char_builder: CharacterBuilder) -> CharacterBuilder>;
// type VecSetterFn = Box<fn(value: Box<Vec<dyn Quirk>>, char_builder: CharacterBuilder) -> CharacterBuilder>;
//
// enum Generation {
//     NAME(StringGenerationFn, StringSetterFn),
//     SURNAME(StringGenerationFn, StringSetterFn),
//     NICKNAME(StringGenerationFn, StringSetterFn),
//     DESCRIPTION(StringGenerationFn, StringSetterFn),
//     IMAGE(StringGenerationFn, StringSetterFn),
//     COLLECTION(StringGenerationFn, StringSetterFn),
//     PROFESSION(StringGenerationFn, StringSetterFn),
//     HOBBIES(VecGenerationFn, VecSetterFn),
//     PROPS(VecGenerationFn, VecSetterFn),
//     LEVELS(VecGenerationFn, VecSetterFn),
//     STATS(VecGenerationFn, VecSetterFn),
//     QUIRKS(VecGenerationFn, VecSetterFn)
// }

//mod generator {
    // Character


// pub fn read_config() {
//     let settings = config::bui
//         // Add in `./Settings.toml`
//         .add_source(config::File::with_name("resources/config.toml"))
//         // Add in settings from the environment (with a prefix of APP)
//         // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
//         //.add_source(config::Environment::with_prefix("APP"))
//         .build()
//         .unwrap();
//     eprintln!("ciaone");
// }
//}

// fn read_dictionary(dict_file: &str) {
//     read_lines(dict_file)
// }
