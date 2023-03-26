mod data {
    pub mod first_names;
    pub mod last_names;
}

use data::first_names::*;
use data::last_names::*;
use rand::prelude::*;

wai_bindgen_rust::export!("faker.wai");

fn get_random_string<'a>(strings: &'a [&'a str]) -> &'a str {
    let mut rng = thread_rng();
    strings.choose(&mut rng).unwrap()
}

pub struct Faker;

impl faker::Faker for Faker {
    fn first_name() -> String {
        String::from(get_random_string(&FIRST_NAMES))
    }

    fn last_name() -> String {
        String::from(get_random_string(&LAST_NAMES))
    }

    fn full_name() -> String {
        let first_name = Self::first_name();
        let last_name = Self::last_name();
        format!("{} {}", first_name, last_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use faker::Faker as _;

    #[test]
    fn first_name() {
        let result = Faker::first_name();
        assert!(FIRST_NAMES.contains(&result.as_str()));
    }

    #[test]
    fn last_name() {
        let result = Faker::last_name();
        assert!(LAST_NAMES.contains(&result.as_str()));
    }

    #[test]
    fn full_name() {
        let result = Faker::full_name();
        let split_result: Vec<&str> = result.split_whitespace().collect();

        assert_eq!(split_result.len(), 2);

        let first_name = split_result[0];
        let last_name = split_result[1];

        assert!(FIRST_NAMES.contains(&first_name));
        assert!(LAST_NAMES.contains(&last_name));
    }
}
