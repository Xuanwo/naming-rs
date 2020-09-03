use std::collections::{HashSet};
use std::iter::FromIterator as _;

use lazy_static::lazy_static;

lazy_static! {
    static ref COMMON_INITIALISM: HashSet<String> = {
        let mut s = HashSet::new();
        let x = vec![
            "acl",
            "api",
            "ascii",
            "cpu",
            "css",
            "dns",
            "eof",
            "guid",
            "html",
            "http",
            "https",
            "id",
            "ip",
            "json",
            "lhs",
            "qps",
            "ram",
            "rhs",
            "rpc",
            "sla",
            "smtp",
            "sql",
            "ssh",
            "tcp",
            "tls",
            "ttl",
            "udp",
            "ui",
            "uid",
            "uuid",
            "uri",
            "url",
            "utf8",
            "vm",
            "xml",
            "xmpp",
            "xsrf",
            "xss"
        ];

        for v in x {
            s.insert(v.to_string());
        }

        s
    };
}

enum State {
    Upper,
    Lower,
    Digit,
    Split,
}

impl From<&char> for State {
    fn from(b: &char) -> Self {
        match b {
            '0'..='9' => State::Digit,
            'a'..='z' => State::Lower,
            'A'..='Z' => State::Upper,
            _ => State::Split,
        }
    }
}

impl From<&u8> for State {
    fn from(b: &u8) -> Self {
        match *b as char {
            '0'..='9' => State::Digit,
            'a'..='z' => State::Lower,
            'A'..='Z' => State::Upper,
            _ => State::Split,
        }
    }
}


/// Split string via special chars.
///
/// Examples:
///   a_bc => [a, bc]
///   a-bc => [a, bc]
///   a bc => [a, bc]
///   a--b => [a, b]
#[inline]
fn split_via_special_chars(s: &str) -> Vec<String> {
    assert_ne!(s.len(), 0, "input string should not be empty");

    let mut idx = 0;
    let mut ans: Vec<String> = vec![String::new()];

    for v in s.as_bytes() {
        match State::from(v) {
            State::Split => {
                if !ans[idx].is_empty() {
                    idx += 1;
                    ans.push(String::new());
                }
            }
            _ => {
                ans[idx].push(*v as char)
            }
        }
    }

    ans
}

/// Split string via upper chars.
/// Examples:
///   abc => [abc]
///   Abc => [Abc]
///   AbC => [Ab, C]
///  AbCd => [Ab, Cd]
///   AAA => [AAA]
///   AAABb => [AAA, Bb]
///
///
/// All state: empty, upper, digit+lower
/// All action: add, append
///
/// prev, cur, next, action
/// empty, upper, empty, append
/// empty, upper, upper, append
/// empty, upper, lower, append
/// upper, upper, empty, append
/// upper, upper, upper, append
/// upper, upper, lower, add
/// lower, upper, empty, add
/// lower, upper, upper, add
/// lower, upper, lower, add
/// empty, lower, empty, append
/// empty, lower, upper, append
/// empty, lower, lower, append
/// upper, lower, empty, append
/// upper, lower, upper, append
/// upper, lower, lower, append
/// lower, lower, empty, append
/// lower, lower, upper, append
/// lower, lower, lower, append
fn split_via_upper_chars(s: &str) -> Vec<String> {
    assert_ne!(s.len(), 0, "input string should not be empty");

    let mut idx = 0;
    let mut ans: Vec<String> = vec![String::new()];

    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
        let ch = bytes[i] as char;

        // If prev is empty, we always add current into buf.
        // This case handles all prev == empty cases.
        if i == 0 {
            ans[idx].push(ch);
            continue;
        }

        // Handle all add cases.
        match State::from(&ch) {
            State::Upper => {
                match State::from(&bytes[i - 1]) {
                    State::Split => {
                        ans[idx].push(ch);
                    }
                    State::Lower | State::Digit => {
                        ans.push(String::new());
                        idx += 1;
                        ans[idx].push(ch);
                    }
                    State::Upper => {
                        if i != bytes.len() - 1 {
                            match State::from(&bytes[i + 1]) {
                                State::Lower => {
                                    ans.push(String::new());
                                    idx += 1;
                                    ans[idx].push(ch);
                                }
                                _ => {
                                    ans[idx].push(ch);
                                }
                            }
                        } else {
                            ans[idx].push(ch);
                        }
                    }
                }
            }
            _ => {
                ans[idx].push(ch)
            }
        }
    }

    ans
}

fn split_string_in_parts(s: &str) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();

    for v in split_via_special_chars(s) {
        if v.is_empty() {
            continue;
        }
        let mut x = split_via_upper_chars(&v);
        for i in x.iter_mut() {
            ans.push(i.to_lowercase())
        }
    }

    ans
}

/// Convert string to camel case.
pub fn to_camel(s: &str) -> String {
    let x = split_string_in_parts(s);
    let mut ans = String::from(&x[0]);

    ans.push_str(
        x.iter().skip(0)
            .map(|v| {
                if COMMON_INITIALISM.contains(v) {
                    to_upper(v)
                } else {
                    to_upper_first(v)
                }
            })
            .collect::<Vec<String>>()
            .join("").as_str()
    );

    ans
}

/// Convert string to pascal case.
pub fn to_pascal(s: &str) -> String {
    split_string_in_parts(s).iter().map(|v| {
        if COMMON_INITIALISM.contains(v) {
            to_upper(v)
        } else {
            to_upper_first(v)
        }
    }).collect::<Vec<String>>().join("")
}

/// Convert string to snack case.
pub fn to_snack(s: &str) -> String {
    split_string_in_parts(s).join("_")
}

/// Convert string to kebab case.
pub fn to_kebab(s: &str) -> String {
    split_string_in_parts(s).join("-")
}

/// Convert string to upper case.
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

/// Convert string to upper first case.
pub fn to_upper_first(s: &str) -> String {
    assert_ne!(s.len(), 0);

    let mut x: Vec<char> = Vec::from_iter(s.as_bytes().iter().map(|x| *x as char));
    x[0] = x[0].to_ascii_uppercase();

    String::from_iter(x)
}

macro_rules! declare_trait {
    ($name:ident, $value:ident) => {
        pub trait $name {
            fn $value(&self) -> String;
        }

        impl $name for String {
            fn $value(&self) -> String {
                $value(&self)
            }
        }
    }
}

declare_trait!(ToKebab, to_kebab);
declare_trait!(ToCamel, to_camel);
declare_trait!(ToPascal, to_pascal);
declare_trait!(ToSnack, to_snack);
declare_trait!(ToUpper, to_upper);
declare_trait!(ToUpperFirst, to_upper_first);

#[cfg(test)]
mod tests;
