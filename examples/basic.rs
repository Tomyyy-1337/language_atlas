use language_atlas::generate_language_functions;

enum Language {
    English,
    Spanish,
    French,
}

generate_language_functions! {
    LanguageEnum: Language
    greeting {
        English: "Hello"
        Spanish: "Hola"
        French:  "Bonjour"
    }
    farewell(name) {
        English: "Goodbye, {name}"
        Spanish: "Adiós, {name}"
        French:  "Au revoir, {name}"
    }
    calculate(a, b, c) {
        English: "{a} + {b} = {c}"
    }
}

fn main() {
    let lang = Language::English;
    assert_eq!(lang.greeting(), "Hello");
    assert_eq!(lang.farewell("John"), "Goodbye, John");
    assert_eq!(lang.calculate(1, 2, 3), "1 + 2 = 3");

    let lang = Language::Spanish;
    assert_eq!(lang.greeting(), "Hola");
    assert_eq!(lang.farewell("Juan"), "Adiós, Juan");
    assert_eq!(lang.calculate(1, 2, 3), "1 + 2 = 3");

    let lang = Language::French;
    assert_eq!(lang.greeting(), "Bonjour");
    assert_eq!(lang.farewell("Jean"), "Au revoir, Jean");
    assert_eq!(lang.calculate(1, 2, 3), "1 + 2 = 3");
}
