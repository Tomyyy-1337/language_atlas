#[cfg(test)]
mod tests {
    use crate::generate_language_functions;


    #[test]
    fn it_works() {
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
            date(day: u8, month: u8, year: u16) {
                French:  "{day}/{month}/{year}"
                English: "{month}/{day}/{year}"
            }
            dummy {  }
        }

        let mut lang = Language::English;
        assert_eq!(lang.greeting(), "Hello");
        assert_eq!(lang.farewell("John"), "Goodbye, John");
        assert_eq!(lang.date(1, 2, 2021), "2/1/2021");
        assert_eq!(lang.dummy(), "ToDo!");

        lang = Language::Spanish;
        assert_eq!(lang.greeting(), "Hola");
        assert_eq!(lang.farewell("Juan"), "Adiós, Juan");
        assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
        assert_eq!(lang.dummy(), "ToDo!");

        lang = Language::French;
        assert_eq!(lang.greeting(), "Bonjour");
        assert_eq!(lang.farewell("Jean"), "Au revoir, Jean");
        assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
        assert_eq!(lang.dummy(), "ToDo!");
    }

    #[test]
    fn basecase() {
        enum Language {
            German,
            English,
            Spanish,
        }

        generate_language_functions! {
            LanguageEnum: Language
            greeting {
                German:  "Hallo"
            }
            counter(n: u8) {
                German: "#{n}"
            }
            content(content) {
                German: "C: {content}"
            }
        }

        let mut lang = Language::German;
        assert_eq!(lang.greeting(), "Hallo");
        assert_eq!(lang.counter(5), "#5");
        assert_eq!(lang.content("Hello"), "C: Hello");

        lang = Language::English;
        assert_eq!(lang.greeting(), "Hallo");
        assert_eq!(lang.counter(5), "#5");
        assert_eq!(lang.content(5), "C: 5");

        lang = Language::Spanish;
        assert_eq!(lang.greeting(), "Hallo");
        assert_eq!(lang.counter(5), "#5");
        assert_eq!(lang.content(1.1), "C: 1.1");
    }

    #[test]
    fn dummy() {
        enum Language {
            German,
            English,
            Spanish,
            French,
        }

        generate_language_functions! {
            LanguageEnum: Language
            dummy {  }
            dummy_args(a: u8, b: u8) {  }
            dummy_args_general(a, b) {  }
        }

        let mut lang = Language::German;
        assert_eq!(lang.dummy(), "ToDo!");
        assert_eq!(lang.dummy_args(1, 2), "ToDo!");
        assert_eq!(lang.dummy_args_general(1, 2), "ToDo!");
        assert_eq!(lang.dummy_args_general("abs", 1.5), "ToDo!");

        lang = Language::English;
        assert_eq!(lang.dummy(), "ToDo!");

        lang = Language::Spanish;
        assert_eq!(lang.dummy_args(1, 2), "ToDo!");

        lang = Language::French;
        assert_eq!(lang.dummy_args_general(42, "The answer to everything"), "ToDo!");

    }

    #[test]
    fn single_language() {
        enum Language {
            English,
        }

        generate_language_functions! {
            LanguageEnum: Language
            greeting {
                English: "Hello"
            }
            farewell(name) {
                English: "Goodbye, {name}"
            }
            date(day: u8, month: u8, year: u16) {
                English: "{month}/{day}/{year}"
            }
        }

        let lang = Language::English;
        assert_eq!(lang.greeting(), "Hello");
        assert_eq!(lang.farewell("John"), "Goodbye, John");
        assert_eq!(lang.date(1, 2, 2021), "2/1/2021");
    }

    #[test]
    fn all_values_given() {
        enum Variants {
            English,
            Spanish,
            French,
        }
        generate_language_functions! {
            LanguageEnum: Variants
            greeting {
                English: "Hello"
                Spanish: "Hola"
                French:  "Bonjour"
            }
            number(n: u8) {
                English: "number: {n}"
                Spanish: "número: {n}"
                French:  "nombre: {n}"
            }
            content(content) {
                English: "Content: {content}"
                Spanish: "Contenido: {content}"
                French:  "Contenu: {content}"
            }
        }

        let mut lang = Variants::English;
        assert_eq!(lang.greeting(), "Hello");
        assert_eq!(lang.number(5), "number: 5");
        assert_eq!(lang.content("Hello"), "Content: Hello");

        lang = Variants::Spanish;
        assert_eq!(lang.greeting(), "Hola");
        assert_eq!(lang.number(5), "número: 5");
        assert_eq!(lang.content("Hola"), "Contenido: Hola");

        lang = Variants::French;
        assert_eq!(lang.greeting(), "Bonjour");
        assert_eq!(lang.number(5), "nombre: 5");
        assert_eq!(lang.content("Bonjour"), "Contenu: Bonjour");
    }
}