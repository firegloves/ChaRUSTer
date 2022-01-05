use charuster::config;

fn main() {

    // la generazione avviene in una funzione che riceve 2 funzioni generics:
    // - funzione di generazione del valore da settare
    // - set del valore (setter o simile del Character)
    // la configurazione del tutto avviene tramite una enum che contiene le 2 fn (NAME, SURNAME, PROPS, etc)
    // poi iterando sul vec di quanto ottenuto al punto precedente eseguiamo la generazione dei vari componenti del Char

    println!("Hello, world!");

    config::parse_config();

    // let empty_builder = Character::builder();
    // let builder = FeatureGenerator::generate(empty_builder);
    // let character = builder.build();
    // println!("{}", Character::name(&character));
    // println!("{}", character.name());
    // println!("{}", character.surname());
}

//use config;

//mod generator {
// Character
// struct Character {
//     name: String,
//     surname: String,
//     nickname: String,
//     description: String,
//     hobbies: Vec<String>,
//     image: String,
//     collection: String,
//     profession: String,
//     props: Vec<Property>,
//     levels: Vec<Level>,
//     stats: Vec<Stat>,
//     quirks: Vec<String>
// }
//
// impl Character {
//     fn name(&mut self, name: &str) {
//         self.name = name.to_string();
//     }
// }
//
// struct FeatureGenerator {}
//
// impl FeatureGenerator {
//     pub fn generate() -> fn(mut character: &Character) -> Character {
//         let my_name: &str = String::from("Aldino").as_str();
//         |character| {
//             Character::name(character, my_name);
//             character
//         }
//     }
// }
//
//
// // Property
// struct Property {
//     prop_type: String,
//     name: String
// }
//
// // Level
// struct Level {
//     name: String,
//     value: i32,
//     max_value: i32
// }
//
// // Stat
// struct Stat {
//     name: String,
//     value: i32,
//     max_value: i32,
// }

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

//fn read_dictionary(dict_file: &str) {
//    read_lines(dict_file)
//}
