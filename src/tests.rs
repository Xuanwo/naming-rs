use super::*;

macro_rules! split_via_special_chars_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, split_via_special_chars(&input.to_string()));
        }
    )*
    }
}

split_via_special_chars_tests! {
    test_split_via_special_chars_0: ("abc", vec!["abc"]),
    test_split_via_special_chars_1: ("a_bc", vec!["a", "bc"]),
    test_split_via_special_chars_2: ("a,bc", vec!["a", "bc"]),
    test_split_via_special_chars_3: ("ABC", vec!["ABC"]),
    test_split_via_special_chars_4: ("A-BC", vec!["A", "BC"]),
    test_split_via_special_chars_5: ("a--b", vec!["a", "b"]),
}

macro_rules! split_via_upper_chars_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, split_via_upper_chars(&input.to_string()));
        }
    )*
    }
}

split_via_upper_chars_tests! {
    split_via_upper_chars_0: ("abc", vec!["abc"]),
    split_via_upper_chars_1: ("Abc", vec!["Abc"]),
    split_via_upper_chars_2: ("AbC", vec!["Ab", "C"]),
    split_via_upper_chars_3: ("AbCd", vec!["Ab","Cd"]),
    split_via_upper_chars_4: ("AAA", vec!["AAA"]),
    split_via_upper_chars_5: ("AAABb", vec!["AAA", "Bb"]),
    split_via_upper_chars_6: ("MD5", vec!["MD5"]),
}