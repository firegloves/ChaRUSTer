type StringGenerationFn = Box<fn(config: Config) -> &'static String>;
type VecGenerationFn = Box<fn(config: Config) -> &'static Vec<dyn Quirk>>;
type StringSetterFn = Box<fn(value: String, char_builder: CharacterBuilder) -> CharacterBuilder>;
type VecSetterFn = Box<fn(value: Vec<Quirk>, char_builder: CharacterBuilder) -> CharacterBuilder>;

enum Generation {
    NAME(StringGenerationFn, StringSetterFn),
    SURNAME(StringGenerationFn, StringSetterFn),
    NICKNAME(StringGenerationFn, StringSetterFn),
    DESCRIPTION(StringGenerationFn, StringSetterFn),
    IMAGE(StringGenerationFn, StringSetterFn),
    COLLECTION(StringGenerationFn, StringSetterFn),
    PROFESSION(StringGenerationFn, StringSetterFn),
    HOBBIES(VecGenerationFn, VecSetterFn),
    PROPS(VecGenerationFn, VecSetterFn),
    LEVELS(VecGenerationFn, VecSetterFn),
    STATS(VecGenerationFn, VecSetterFn),
    QUIRKS(VecGenerationFn, VecSetterFn)
}
