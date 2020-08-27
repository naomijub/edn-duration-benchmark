use std::time::{Instant};
use edn_rs;
use edn_derive::Deserialize;

fn main() {
    let edn = edn_str();

    let start = Instant::now();
    let _ : Entity = edn_rs::from_str(&edn).unwrap();
    let duration = start.elapsed();

    println!("Time elapsed in Deserializing into Entity structs is: {:?}", duration);
}

fn edn_str() -> String {
    "{
        :entity-type :human
        :first-name \"bench\"
        :last-name \"mark\"
        :age 13
        :version 0.13
        :associates [
            {
                :name :julia
                :role :adm
            }
            {
                :name :otavio
                :role :contributor
            }
            {
                :name :juxt
                :role :great-ideas
            }
        ]
    }".to_string()
}

#[derive(Deserialize, Debug, Clone)]
struct Entity {
    entity_type: String,
    first_name: String,
    last_name: String,
    age: usize,
    version: f64,
    associates: Vec<Person>
}

#[derive(Deserialize, Debug, Clone)]
struct Person {
    name: String,
    role: String
}