/// This macro genrerates functions for a given enum that return language variants of a String.
/// The generatiated functions can take parameters that implement `std::fmt::Display`.
///
/// # Notes
/// - The first language variant is considered the default value.
/// - If a language variant is not provided for a field, the default value is used.
/// - If no language string is provided for a field, a deprecated function returning "ToDo!" is generated. The function signature stays the same.
/// - Parameter functions return a `String` type, while non-parameter functions return a `&'static str` type.
///
/// # Example
/// ```rust
/// use language_atlas::generate_language_functions;
///
/// enum Language {
///     English,
///     Spanish,
///     French,
/// }
///
/// generate_language_functions! {
///     LanguageEnum: Language
///     // returns a `&'static str`
///     greeting {
///         English: "Hello"
///         Spanish: "Hola"
///         French:  "Bonjour"
///     }
///     // returns a `String`
///     // name is a parameter that implements `std::fmt::Display`. Parameter types are optional.
///     farewell(name) {
///         English: "Goodbye, {name}"
///         Spanish: "Adiós, {name}"
///         French:  "Au revoir, {name}"
///     }
///     // returns a `String`. English is the default value for all languages.
///     // day, month, and year have types u8, u8, and u16 respectively
///     date(day: u8, month: u8, year: u16) {
///         French:  "{day}/{month}/{year}"
///         English: "{month}/{day}/{year}"
///     }
///     // Generates a deprecated placeholder function returning `ToDo!`
///     dummy {  }
/// }
///
/// fn main() {
///     let mut lang = Language::English;
///     assert_eq!(lang.greeting(), "Hello");
///     assert_eq!(lang.farewell("John"), "Goodbye, John");
///     assert_eq!(lang.date(1, 2, 2021), "2/1/2021");
///     assert_eq!(lang.dummy(), "ToDo!");
///
///     lang = Language::Spanish;
///     assert_eq!(lang.greeting(), "Hola");
///     assert_eq!(lang.farewell("Juan"), "Adiós, Juan");
///     assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
///     assert_eq!(lang.dummy(), "ToDo!");
///
///     lang = Language::French;
///     assert_eq!(lang.greeting(), "Bonjour");
///     assert_eq!(lang.farewell("Jean"), "Au revoir, Jean");
///     assert_eq!(lang.date(1, 2, 2021), "1/2/2021");
///     assert_eq!(lang.dummy(), "ToDo!");
/// }
/// ```
/// # Expands to
///
/// ```rust
/// enum Language {
///     English,
///     Spanish,
///     French,
/// }
///
/// #[allow(unreachable_patterns)]
/// #[allow(non_camel_case_types)]
/// impl Language {
///     pub fn greeting(&self) -> &'static str {
///         match self {
///             Language::Spanish => "Hola",
///             Language::French => "Bonjour",
///             Language::English | _ => "Hello",
///         }
///     }
///     pub fn farewell<name: std::fmt::Display>(&self, name: name) -> String {
///         match self {
///             Language::Spanish => format!("Adiós, {name}"),
///             Language::French => format!("Au revoir, {name}"),
///             Language::English | _ => format!("Goodbye, {name}"),
///         }
///     }
///     pub fn date(&self, a: u8, b: u8, c: u16) -> String {
///         match self {
///             Language::English => format!("{b}/{a}/{c}"),
///             Language::French | _ => format!("{a}/{b}/{c}"),
///         }
///     }
///     #[deprecated(note = "No language string provided for this field. Defaulting to 'ToDo!'")]
///     pub fn dummy(&self) -> &'static str {
///        "ToDo!"
///     }
/// }
/// ```
#[macro_export]
macro_rules! generate_language_functions {
    (
        LanguageEnum: $enum_name:ident
        $($field:ident $( ( $($args:ident $(: $args_type:ty )? ),+ ) )? {
            $($lang:ident: $value:expr $(,)? )*
        })*
    ) => {
        #[allow(unreachable_patterns)]
        #[allow(non_camel_case_types)]
        impl $enum_name {
            $(
                generate_language_functions!(@field_impl $enum_name $field $( ( $($args $($args_type)? ),* ) )? { $($lang: $value,)* } );
            )*
        }
    };

    (@field_impl $enum_name:ident $field:ident { } ) => {
        #[deprecated(note = "No language string provided for this field. Defaulting to 'ToDo!'")]
        pub fn $field(&self) -> &'static str {
            "ToDo!"
        }
    };

    (@field_impl $enum_name:ident $field:ident ( $($args:ident ),* ) { } ) => {
        #[deprecated(note = "No language string provided for this field. Defaulting to 'ToDo!'")]
        pub fn $field<$( $args: std::fmt::Display, )*>(
            &self,
            $( $args: $args, )*
        ) -> String {
            String::from("ToDo!")
        }
    };

    (@field_impl $enum_name:ident $field:ident ( $($args:ident $args_type:ty ),+ ) { } ) => {
        #[deprecated(note = "No language string provided for this field. Defaulting to 'ToDo!'")]
        pub fn $field(
            &self,
            $( $args: $args_type, )+
        ) -> String {
            String::from("ToDo!")
        }
    };

    (@field_impl $enum_name:ident $field:ident {
        $first_lang:ident: $first_value:expr,
        $($lang:ident: $value:expr,)*
    }) => {
        pub fn $field(&self) -> &'static str {
            match self {
                $( $enum_name::$lang => $value, )*
                $enum_name::$first_lang | _ => $first_value,
            }
        }
    };

    (@field_impl $enum_name:ident $field:ident ( $($args:ident),+ ) {
        $first_lang:ident: $first_value:expr,
        $($lang:ident: $value:expr,)+
    } ) => {
        pub fn $field<$( $args: std::fmt::Display, )*>(
            &self,
            $( $args: $args, )*
        ) -> String {
            generate_language_functions! { @match_impl_string self $enum_name $first_lang $first_value, { $($lang: $value),* } }
        }
    };

    (@field_impl $enum_name:ident $field:ident ( $($args:ident $args_type:ty ),+ ) {
        $first_lang:ident: $first_value:expr,
        $($lang:ident: $value:expr,)+
    } ) => {
        pub fn $field(
            &self,
            $( $args: $args_type, )+
        ) -> String {
            generate_language_functions! { @match_impl_string self $enum_name $first_lang $first_value, { $($lang: $value),* } }
        }
    };

    (@match_impl_string $self:ident $enum_name:ident $first_lang:ident $first_value:expr, { $($lang:ident: $value:expr),* }) => {
        match $self {
            $( $enum_name::$lang => format!($value), )*
            $enum_name::$first_lang | _ => format!($first_value),
        }
    };

}
