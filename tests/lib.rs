extern crate voca_rs;

#[cfg(test)]
mod tests {
    use voca_rs::*;

    /// voca_rs::utils testing
    #[test]
    fn utils_version() {
        assert_eq!(utils::VERSION, "0.1.0");
    }
    #[test]
    fn utils_ascii_letters() {
        assert_eq!(
            utils::ASCII_LETTERS,
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
    }
    #[test]
    fn utils_ascii_lowercase() {
        assert_eq!(utils::ASCII_LOWERCASE, "abcdefghijklmnopqrstuvwxyz");
    }
    #[test]
    fn utils_ascii_uppercase() {
        assert_eq!(utils::ASCII_UPPERCASE, "ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    #[test]
    fn utils_digits() {
        assert_eq!(utils::DIGITS, "0123456789");
    }
    #[test]
    fn utils_hexdigits() {
        assert_eq!(utils::HEXDIGITS, "0123456789abcdefABCDEF");
    }
    #[test]
    fn utils_octdigits() {
        assert_eq!(utils::OCTDIGITS, "01234567");
    }
    #[test]
    fn utils_punctuation() {
        assert_eq!(utils::PUNCTUATION, "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~");
    }
    #[test]
    fn utils_whitespace() {
        assert_eq!(utils::WHITESPACE, " \t\n\r");
    }

    /// voca_rs::split testing
    #[test]
    fn split_to_chars() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a", "v", "i", "t", "y"]);
        assert_eq!(split::chars("  "), [" ", " "]);
        assert_eq!(split::chars("a b"), ["a", " ", "b"]);
        assert_eq!(split::chars("ÜbER"), ["Ü", "b", "E", "R"]);
        assert_eq!(split::chars("\n\t"), ["\n", "\t"]);
        assert_eq!(split::chars(""), [""]);
    }
    #[test]
    #[should_panic]
    fn split_to_chars_panic() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a"]);
    }

    #[test]
    fn split_by_pattern() {
        assert_eq!(
            split::split("gravity can cross dimensions", " "),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(split::split("*dying*star*", "*"), ["", "dying", "star"]);
        assert_eq!(split::split("dying star", ""), ["dying star"]);
        assert_eq!(split::split("Über Stern", ""), ["Über Stern"]);
        assert_eq!(split::split("", ""), [""]);
    }
    #[test]
    #[should_panic]
    fn split_by_pattern_panic() {
        assert_eq!(split::chars("gravity"), ["g", "r", "a"]);
    }

    #[test]
    fn split_words() {
        assert_eq!(
            split::words("gravity can cross dimensions"),
            ["gravity", "can", "cross", "dimensions"]
        );
        assert_eq!(
            split::words("gravity    dying\r\nstar\tfalling"),
            ["gravity", "dying", "star", "falling"]
        );
        assert_eq!(
            split::words("Zażółć gęślą jaźń"),
            ["Zażółć", "gęślą", "jaźń"]
        );
        assert_eq!(split::words("splitCamelCase"), ["split", "Camel", "Case"]);
        assert_eq!(
            split::words("split-some kind_of_mixed CaseHere"),
            ["split", "some", "kind", "of", "mixed", "Case", "Here"]
        );
        assert_eq!(
            split::words("LazyLoad with XMLHttpRequest and snake_case"),
            ["Lazy", "Load", "with", "XML", "Http", "Request", "and", "snake", "case"]
        );
    }
    #[test]
    #[should_panic]
    fn split_words_panic() {
        assert_eq!(
            split::words("gravity can cross dimensions"),
            ["gravity1", "can", "cross", "dimensions"]
        );
    }

    #[test]
    fn split_to_graphemes() {
        assert_eq!(
            split::graphemes("a̐éö̲\r\n"),
            ["a̐", "é", "ö̲", "\r\n"]
        );
        assert_eq!(split::graphemes(""), [""]);
    }
    #[test]
    #[should_panic]
    fn split_to_graphemes_panic() {
        assert_eq!(split::graphemes("\r"), ["r"]);
    }

    /// voca_rs::query testing
    #[test]
    fn query_ends_with() {
        assert!(query::ends_with("the world is yours", "is yours"));
        assert!(query::ends_with("Zażółć gęślą jaźń", "jaźń"));
        assert!(query::ends_with("the world is yours", ""));
        assert!(query::ends_with("", ""), true);
    }
    #[test]
    #[should_panic]
    fn query_dont_ends_with() {
        assert!(query::ends_with("a b c", "b"));
    }
    #[test]
    fn query_includes() {
        assert!(query::includes("the world is yours", "the world", 0));
        assert!(query::includes("Zażółć gęślą jaźń", "gęślą", 7));
        assert!(query::includes("the world is yours", "", 0));
        assert!(query::includes("", "", 0), true);
    }
    #[test]
    #[should_panic]
    fn query_dont_includes() {
        assert!(query::includes("a b c", "d", 0));
    }
    #[test]
    fn query_is_blank() {
        assert!(query::is_blank(""), true);
        assert_eq!(query::is_blank("   "), true);
        assert_eq!(query::is_blank("\n\t\r"), true);
        assert_eq!(query::is_blank("Zażółć gęślą jaźń"), false);
    }
    #[test]
    fn query_is_empty() {
        assert!(query::is_empty(""), true);
        assert_eq!(query::is_empty("Zażółć gęślą jaźń"), false);
        assert_eq!(query::is_empty("the world is yours"), false);
    }
    #[test]
    fn query_is_lowercase() {
        assert!(query::is_lowercase(""), true);
        assert_eq!(query::is_lowercase("the world is yours"), true);
        assert_eq!(query::is_lowercase("Zażółć gęślą jaźń"), false);
        assert_eq!(query::is_lowercase("T1000"), false);
    }
    #[test]
    fn query_starts_with() {
        assert!(query::starts_with("the world is yours", "the world"));
        assert!(query::starts_with(
            "Zażółć gęślą jaźń",
            "Zażółć"
        ));
        assert!(query::starts_with("the world is yours", ""));
        assert!(query::starts_with("", ""), true);
    }
    #[test]
    #[should_panic]
    fn query_dont_starts_with() {
        assert!(query::starts_with("a b c", "b"));
    }

    /// voca_rs::case testing
    #[test]
    fn case_camel_case() {
        assert_eq!(case::camel_case("The World - IS Yours"), "theWorldIsYours");
        assert_eq!(
            case::camel_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółćGęśląJaźń"
        );
        assert_eq!(
            case::camel_case("say  ***    Hello\r\n   to--ME++"),
            "sayHelloToMe"
        );
        assert_eq!(case::camel_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_camel_case_panic() {
        assert_eq!(case::camel_case("ABC"), "ABC");
    }

    #[test]
    fn case_pascal_case() {
        assert_eq!(case::pascal_case("The World - IS Yours"), "TheWorldIsYours");
        assert_eq!(
            case::pascal_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZażółćGęśląJaźń"
        );
        assert_eq!(
            case::pascal_case("say  ***    Hello\r\n   to--ME++"),
            "SayHelloToMe"
        );
        assert_eq!(case::pascal_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_pascal_case_panic() {
        assert_eq!(case::pascal_case("ABC"), "ABC");
    }

    #[test]
    fn case_capitalize() {
        assert_eq!(
            case::capitalize("The World IS YourS", &true),
            "The world is yours"
        );
        assert_eq!(
            case::capitalize("ZAżółć GĘŚLĄ jAźń", &true),
            "Zażółć gęślą jaźń"
        );
        assert_eq!(
            case::capitalize("say Hello to ME", &false),
            "Say Hello to ME"
        );
        assert_eq!(case::capitalize("", &true), "");
    }
    #[test]
    #[should_panic]
    fn case_capitalize_panic() {
        assert_eq!(case::capitalize("ABC", &true), "ABC");
    }

    #[test]
    fn case_decapitalize() {
        assert_eq!(
            case::decapitalize("The World IS YourS", &true),
            "the world is yours"
        );
        assert_eq!(
            case::decapitalize("ZAżółć GĘŚLĄ jAźń", &true),
            "zażółć gęślą jaźń"
        );
        assert_eq!(
            case::decapitalize("Say Hello to ME", &false),
            "say Hello to ME"
        );
        assert_eq!(case::decapitalize("", &true), "");
    }
    #[test]
    #[should_panic]
    fn case_decapitalize_panic() {
        assert_eq!(case::decapitalize("ABC", &true), "ABC");
    }

    #[test]
    fn case_kebab_case() {
        assert_eq!(
            case::kebab_case("The World - IS Yours"),
            "the-world-is-yours"
        );
        assert_eq!(
            case::kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółć-gęślą-jaźń"
        );
        assert_eq!(
            case::kebab_case("say  ***    Hello\r\n   to--ME++"),
            "say-hello-to-me"
        );
        assert_eq!(case::kebab_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_kebab_case_panic() {
        assert_eq!(case::kebab_case("A B C"), "ABC");
    }

    #[test]
    fn case_shouty_kebab_case() {
        assert_eq!(
            case::shouty_kebab_case("The World - IS Yours"),
            "THE-WORLD-IS-YOURS"
        );
        assert_eq!(
            case::shouty_kebab_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZAŻÓŁĆ-GĘŚLĄ-JAŹŃ"
        );
        assert_eq!(
            case::shouty_kebab_case("say  ***    Hello\r\n   to--ME++"),
            "SAY-HELLO-TO-ME"
        );
        assert_eq!(case::shouty_kebab_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_shouty_kebab_case_panic() {
        assert_eq!(case::shouty_kebab_case("A B C"), "ABC");
    }

    #[test]
    fn case_lower_case() {
        assert_eq!(case::lower_case("The World IS YourS"), "the world is yours");
        assert_eq!(
            case::lower_case("Zażółć gęśLą jaźń"),
            "zażółć gęślą jaźń"
        );
        assert_eq!(case::lower_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_lower_case_panic() {
        assert_eq!(case::lower_case("ABC"), "ABC");
    }

    #[test]
    fn case_snake_case() {
        assert_eq!(
            case::snake_case("The World - IS Yours"),
            "the_world_is_yours"
        );
        assert_eq!(
            case::snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "zażółć_gęślą_jaźń"
        );
        assert_eq!(
            case::snake_case("say  ***    Hello\r\n   to--ME++"),
            "say_hello_to_me"
        );
        assert_eq!(case::snake_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_snake_case_panic() {
        assert_eq!(case::snake_case("A B C"), "ABC");
    }

    #[test]
    fn case_shouty_snake_case() {
        assert_eq!(
            case::shouty_snake_case("The World - IS Yours"),
            "THE_WORLD_IS_YOURS"
        );
        assert_eq!(
            case::shouty_snake_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "ZAŻÓŁĆ_GĘŚLĄ_JAŹŃ"
        );
        assert_eq!(
            case::shouty_snake_case("say  ***    Hello\r\n   to--ME++"),
            "SAY_HELLO_TO_ME"
        );
        assert_eq!(case::shouty_snake_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_shouty_snake_case_panic() {
        assert_eq!(case::shouty_snake_case("A B C"), "ABC");
    }

    #[test]
    fn case_swap_case() {
        assert_eq!(
            case::swap_case("The World - IS Yours"),
            "tHE wORLD - is yOURS"
        );
        assert_eq!(
            case::swap_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "_zAŻÓŁĆ-gęślą_JAŹŃ-"
        );
        assert_eq!(
            case::swap_case("say über Hello to--ME++"),
            "SAY ÜBER hELLO TO--me++"
        );
        assert_eq!(case::swap_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_swap_case_panic() {
        assert_eq!(case::swap_case("A B C"), "---");
    }

    #[test]
    fn case_upper_case() {
        assert_eq!(case::upper_case("The World IS YourS"), "THE WORLD IS YOURS");
        assert_eq!(
            case::upper_case("Zażółć gęślą jaźń"),
            "ZAŻÓŁĆ GĘŚLĄ JAŹŃ"
        );
        assert_eq!(case::upper_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_upper_case_panic() {
        assert_eq!(case::upper_case("ABC"), "abc");
    }

    #[test]
    fn case_title_case() {
        assert_eq!(
            case::title_case("The World - IS Yours"),
            "The World Is Yours"
        );
        assert_eq!(
            case::title_case("_Zażółć-GĘŚLĄ_jaźń-"),
            "Zażółć Gęślą Jaźń"
        );
        assert_eq!(
            case::title_case("say über Hello to--ME++"),
            "Say Über Hello To Me"
        );
        assert_eq!(case::title_case(""), "");
    }
    #[test]
    #[should_panic]
    fn case_title_case_panic() {
        assert_eq!(case::title_case("A B C"), "---");
    }

    // /// voca_rs::manipulate testing
    // #[test]
    // fn manipulate_trim() {
    //     assert_eq!(
    //         manipulate::trim("   The world - is yours\t   ", ''),
    //         "The world - is yours"
    //     );
    //     assert_eq!(
    //         manipulate::trim("--Zażółć gęślą jaźń---", '-'),
    //         "Zażółć gęślą jaźń"
    //     );
    //     assert_eq!(manipulate::trim("++--say über++", '+'), "--say über");
    //     assert_eq!(manipulate::trim("", ''), "");
    // }
    // #[test]
    // #[should_panic]
    // fn manipulate_trim_panic() {
    //     // assert_eq!(manipulate::trim("A B C", ""), "---");
    // }
}
