# Language Atlas
This macro genrerates functions for a given enum that return language variants of a `String`. The generatiated functions can take parameters that implement `std::fmt::Display`.

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
- Parameter funtions return a `String` type, while non-parameter functions return a `&'static str` type.

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
```

## Licence

This project is licensed under the MIT License.
