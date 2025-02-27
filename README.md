# Language Atlas
This macro generates functions for a given enum that return language variants of a `String` or `&'static str` type. 

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
language_atlas = "0.1.0"
```

## Notes

- The first language variant is considered the default value.
- If a language variant is not provided for a field, the default value is used.
- If no language string is provided for a field, a deprecated function returning “ToDo!” is generated. The function signature stays the same.
- Parameter functions return a `String` type, while non-parameter functions return a `&'static str` type.
- The types of parameters are optional. If no types are provided, the parameters have to implement the `Display` trait.

## Example Usage 

```rust
use language_atlas::generate_language_functions;

enum Language {
    English,
    Spanish,
    French,
}

generate_language_functions! {
    LanguageEnum: Language
    // returns a `&'static str`
    greeting { 
        English: "Hello" 
        Spanish: "Hola"
        French:  "Bonjour" 
    }
    // Returns a `String` 
    // name is a parameter that implements `std::fmt::Display`. Parameter types are optional.
    farewell(name) {
        English: "Goodbye, {name}"
        Spanish: "Adios, {name}"
        French:  "Au revoir, {name}"
    }
    // Returns a `String`. English is the default value for all languages.
    // day, month, and year have types u8, u8, and u16 respectively
    date(day: u8, month: u8, year: u16) {
         French:  "{day}/{month}/{year}"
         English: "{month}/{day}/{year}"
     }
    // Generates a deprecated placeholder function  returning `ToDo!`
    dummy { }
}

fn main() {
    let mut lang = Language::English;
    assert_eq!(lang.greeting(), "Hello");
    assert_eq!(lang.farewell("John"), "Goodbye, John");
    assert_eq!(lang.date(1, 2, 2021), "2/1/2021");
    assert_eq!(lang.dummy(), "ToDo!");

    lang = Language::Spanish;
    assert_eq!(lang.greeting(), "Hola");
    assert_eq!(lang.farewell("Juan"), "Adios, Juan");
    assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
    assert_eq!(lang.dummy(), "ToDo!");

    lang = Language::French;
    assert_eq!(lang.greeting(), "Bonjour");
    assert_eq!(lang.farewell("Jean"), "Au revoir, Jean");
    assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
    assert_eq!(lang.dummy(), "ToDo!");
}
```

## Licence

This project is licensed under the MIT License.