use std::{fs, io};
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, Read};
use std::path::Path;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

const ACCEPTED_IMAGE_FORMATS: [&'static str; 3] = ["jpg", "jpeg", "png"];

pub trait Dictionary {
    fn choose_and_remove(&mut self) -> Option<String>;
    fn choose(&self) -> Option<String>;
}

// SimpleDictionary
pub struct SimpleDictionary {
    name: String,
    terms: Vec<String>,
}

impl SimpleDictionary {
    /// create a new dictionary from file
    pub fn new(filename: &str) -> Self {
        let name = get_name_from_file(&filename);
        let terms = read_dictionary(filename).expect(format!("Dictionary not found: {}", filename).as_str());
        SimpleDictionary { name: name.unwrap().to_owned(), terms }
    }

    /// create a new dictionary with a single term
    pub fn new_with_single_term(dict_name: String, term: String) -> Self {
        let terms = vec!(term);
        SimpleDictionary { name: dict_name, terms }
    }

    /// create a new dictionary from directory listing
    pub fn new_from_folder(dir: &str) -> Self {
        let name = get_name_from_file(&dir);
        let paths = fs::read_dir(dir).unwrap();

        let terms: Vec<String> = paths
            .map(|dir_entry| dir_entry.unwrap().path())
            .filter(|path| path.extension().is_some() &&
                ACCEPTED_IMAGE_FORMATS.contains(&(path.extension().unwrap().to_str().unwrap())))
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_owned())
            .collect();

        SimpleDictionary { name: name.unwrap().to_owned(), terms }
    }
}

impl Dictionary for SimpleDictionary {
    /// choose a term of the dictionary, remove it from the vector and return it
    fn choose_and_remove(&mut self) -> Option<String> {
        if self.terms.len() <= 0 {
            ()
        }

        let i = (0..self.terms.len()).choose(&mut thread_rng())?;
        Some(self.terms.swap_remove(i))
    }

    /// choose a term of the dictionary, remove it from the vector and return it
    fn choose(&self) -> Option<String> {
        if self.terms.len() <= 0 {
            ()
        }

        let value = self.terms.choose(&mut thread_rng())?;
        Some(value.clone())
    }
}

// TwoLevelsDictionary
pub struct TwoLevelsDictionary {
    name: String,
    taxonomies: Vec<Taxonomy>,
}

#[derive(Debug, Deserialize)]
struct Taxonomy {
    kind: String,
    terms: Vec<String>
}

impl TwoLevelsDictionary {
    /// create a new dictionary from file
    pub fn new(filename: &str) -> Self {
        let name = get_name_from_file(&filename);
        let terms = parse_two_levels_dictionary(filename).expect(format!("Dictionary not found: {}", filename).as_str());
        TwoLevelsDictionary { name: name.unwrap().to_owned(), taxonomies: terms }
    }
}

impl Dictionary for TwoLevelsDictionary {
    /// choose a term of the dictionary (term and subterm separated by a +), remove it from the vector and return it
    fn choose_and_remove(&mut self) -> Option<String> {
        if self.taxonomies.len() <= 0 {
            ()
        }

        let kind_ind = (0..self.taxonomies.len()).choose(&mut thread_rng())?;
        let mut taxonomy = self.taxonomies.swap_remove(kind_ind);

        let term_ind = (0..taxonomy.terms.len()).choose(&mut thread_rng())?;
        let term = taxonomy.terms.swap_remove(term_ind);

        Some(format!("{}+{}", taxonomy.kind, term))
    }

    /// choose a term of the dictionary (term and subterm separated by a +), remove it from the vector and return it
    fn choose(&self) -> Option<String> {
        if self.taxonomies.len() <= 0 {
            ()
        }

        let taxonomy = self.taxonomies.choose(&mut thread_rng())?;
        let term = taxonomy.terms.choose(&mut thread_rng())?;

        Some(format!("{}+{}", taxonomy.kind, term))
    }
}


fn read_dictionary(filename: &str) -> Option<Vec<String>> {
    let lines = read_lines(filename);
    match lines {
        Ok(lines) => {
            let terms = lines.map(|l| l.expect("Could not parse line"))
                .collect();
            Some(terms)
        },
        Err(_) => {
            None
        },
    }
}

fn parse_two_levels_dictionary(filename: &str) -> Option<Vec<Taxonomy>> {
    let mut file = File::open(filename).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();
    let terms: Vec<Taxonomy> = serde_json::from_str(json.as_str()).expect("JSON was not well-formatted");
    Some(terms)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_name_from_file(filename: &str) -> Option<&str> {
    Path::new(filename).file_name()
        .expect(&format!("Trying to create dictionary from non existing file: {}", filename))
        .to_str()
}


#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    const EXPECTED_TERMS: [&'static str; 3] = ["term_1", "term_2", "term_3"];

    fn get_test_dictionary_filename() -> String {
        get_dictionary_filename("test_dictionary")
    }

    fn get_dictionary_filename(dict_name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/".to_owned() + dict_name);
        d.into_os_string().into_string().unwrap()
    }

    fn get_not_existing_dictionary_filename() -> String {
        let mut d = PathBuf::from("resources/test/not_existing");
        d.into_os_string().into_string().unwrap()
    }

    #[test]
    fn should_read_lines_from_file() {
        let result = read_lines(get_test_dictionary_filename()).unwrap();
        for (i, res_line) in result.enumerate() {
            assert_eq!(res_line.unwrap(), EXPECTED_TERMS[i]);
        }
    }

    #[test]
    #[should_panic]
    fn should_panic_while_reading_lines_from_not_existing_file() {
        read_lines(get_not_existing_dictionary_filename()).unwrap();
    }

    #[test]
    fn should_read_terms_from_dictionary_file() {
        let terms = read_dictionary(get_test_dictionary_filename().as_str()).unwrap();
        for (i, term) in terms.iter().enumerate() {
            assert_eq!(term, &EXPECTED_TERMS[i]);
        }
    }

    #[test]
    fn should_panic_while_reading_terms_from_not_existing_file() {
        let terms = read_dictionary(get_not_existing_dictionary_filename().as_str());
        assert!(terms.is_none());
    }

    #[test]
    fn should_create_dictionary_file() {
        let dict = SimpleDictionary::new(get_test_dictionary_filename().as_str());
        assert_eq!(dict.name, "test_dictionary");
        for (i, term) in dict.terms.iter().enumerate() {
            assert_eq!(term, &EXPECTED_TERMS[i]);
        }
    }

    #[test]
    fn should_create_dictionary_with_single_term() {
        let dict_name = String::from("Zombies");
        let term = String::from("term_1");
        let dict = SimpleDictionary::new_with_single_term(dict_name.clone(), term.clone());

        assert_eq!(dict.name, dict_name);

        let term1 = dict.choose().unwrap();
        assert_eq!(term, term1);
        assert_eq!(dict.terms.len(), 1);

        let term2 = dict.choose().unwrap();
        assert_eq!(term, term1);
        assert_eq!(dict.terms.len(), 1);
    }

    // #[test]
    // fn should_create_dictionary_from_folder() {
    //
    //     let expected_images = vec![String::from("image_1.jpg"), String::from("image_2.png"), String::from("image_1.jpeg")];
    //
    //     let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     d.push("resources/test/image_dir");
    //     let dir = d.into_os_string().into_string().unwrap();
    //     let mut dict = Dictionary::new_from_folder(dir.as_str());
    //
    //     assert_eq!(dict.name, "image_dir");
    //     assert_eq!(dict.terms.len(), 3);
    //     assert!(expected_images.contains(&String::from(dict.terms.pop().unwrap())));
    //     // assert!(expected_images.contains(&dict.terms.pop().unwrap().into_string()));
    //     // assert!(expected_images.contains(&dict.terms.pop().unwrap().into_string()));
    // }

    #[test]
    #[should_panic]
    fn should_panic_while_creating_dictionary_from_non_existing_file() {
        SimpleDictionary::new(get_not_existing_dictionary_filename().as_str());
    }

    #[test]
    #[should_panic]
    fn should_panic_while_creating_dictionary_from_empty_filename() {
        SimpleDictionary::new("");
    }

    #[test]
    fn should_choose_a_random_term_and_remove_it_from_the_dictionary() {
        let mut dict = SimpleDictionary::new(get_test_dictionary_filename().as_str());

        let term1 = dict.choose_and_remove().unwrap();
        assert!(EXPECTED_TERMS.contains(&term1.as_str()));
        assert_eq!(dict.terms.len(), 2);

        let term2 = dict.choose_and_remove().unwrap();
        assert!(EXPECTED_TERMS.contains(&term2.as_str()));
        assert_eq!(dict.terms.len(), 1);
        assert_ne!(term1, term2);

        let term3 = dict.choose_and_remove().unwrap();
        assert!(EXPECTED_TERMS.contains(&term3.as_str()));
        assert_eq!(dict.terms.len(), 0);
        assert_ne!(term1, term3);
    }

    #[test]
    fn should_return_none_when_choosing_from_empty_dictionary() {
        let mut dict = SimpleDictionary{
            name: String::from("My dic"),
            terms: vec![]
        };
        assert!(dict.choose_and_remove().is_none());
    }

    #[test]
    fn should_choose_a_random_term_from_the_dictionary() {
        let mut dict = SimpleDictionary::new(get_test_dictionary_filename().as_str());

        let term1 = dict.choose().unwrap();
        assert!(EXPECTED_TERMS.contains(&term1.as_str()));
        assert_eq!(dict.terms.len(), 3);

        let term2 = dict.choose().unwrap();
        assert!(EXPECTED_TERMS.contains(&term2.as_str()));
        assert_eq!(dict.terms.len(), 3);
    }

    #[test]
    fn should_parse_two_levels_dictionary() {
        let taxonomies = parse_two_levels_dictionary(get_dictionary_filename("test_two_levels_dictionary.json").as_str()).unwrap();
        assert_eq!(taxonomies.len(), 3);

        assert_eq!(taxonomies[0].kind, "term_1");
        assert_eq!(taxonomies[0].terms.len(), 3);
        assert_eq!(taxonomies[0].terms[0], "sub_term_1_1");
        assert_eq!(taxonomies[0].terms[1], "sub_term_1_2");
        assert_eq!(taxonomies[0].terms[2], "sub_term_1_3");

        assert_eq!(taxonomies[1].kind, "term_2");
        assert_eq!(taxonomies[1].terms.len(), 3);
        assert_eq!(taxonomies[1].terms[0], "sub_term_2_1");
        assert_eq!(taxonomies[1].terms[1], "sub_term_2_2");
        assert_eq!(taxonomies[1].terms[2], "sub_term_2_3");

        assert_eq!(taxonomies[2].kind, "term_3");
        assert_eq!(taxonomies[2].terms.len(), 3);
        assert_eq!(taxonomies[2].terms[0], "sub_term_3_1");
        assert_eq!(taxonomies[2].terms[1], "sub_term_3_2");
        assert_eq!(taxonomies[2].terms[2], "sub_term_3_3");
    }
}
