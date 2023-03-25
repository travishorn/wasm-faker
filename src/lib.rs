wai_bindgen_rust::export!("faker.wai");

pub struct Faker;

impl faker::Faker for Faker {
    fn full_name() -> String {
        String::from("John Doe")
    }
}
