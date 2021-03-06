#![allow(dead_code)]

fn main() {}

#[allow(unnecessary_operation)]
fn starts_with() {
    "".chars().next() == Some(' ');
    Some(' ') != "".chars().next();
}

fn chars_cmp_with_unwrap() {
    let s = String::from("foo");
    if s.chars().next().unwrap() == 'f' { // s.starts_with('f')
        // Nothing here
    }
    if s.chars().next_back().unwrap() == 'o' { // s.ends_with('o')
        // Nothing here
    }
    if s.chars().last().unwrap() == 'o' { // s.ends_with('o')
        // Nothing here
    }
    if s.chars().next().unwrap() != 'f' { // !s.starts_with('f')
        // Nothing here
    }
    if s.chars().next_back().unwrap() != 'o' { // !s.ends_with('o')
        // Nothing here
    }
    if s.chars().last().unwrap() != 'o' { // !s.ends_with('o')
        // Nothing here
    }
}

#[allow(unnecessary_operation)]
fn ends_with() {
    "".chars().last() == Some(' ');
    Some(' ') != "".chars().last();
    "".chars().next_back() == Some(' ');
    Some(' ') != "".chars().next_back();
}
