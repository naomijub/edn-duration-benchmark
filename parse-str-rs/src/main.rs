use std::time::{Instant};
use edn_rs;
use std::str::FromStr;

fn main() {
    let edn = edn_str();

    let start = Instant::now();
    let value = edn_rs::Edn::from_str(&edn);
    let duration = start.elapsed();

    if value.is_ok() {
        println!("Time elapsed in edn_rs::from_str is: {:?}", duration);

        let start_nav = Instant::now();
        let role = value.unwrap()[":associates"][0][":role"].clone();
        let duration_nav = start_nav.elapsed();

        println!("Time elapsed to navigate to role_0 \"{}\": {:?}", role, duration_nav);
    } else {
        println!("Parse failed. Duration {:?}", duration);
    }
}

fn edn_str() -> String {
    "{
        :type :human
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