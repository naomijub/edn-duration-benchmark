use criterion::{criterion_group, criterion_main, Criterion};
use edn_rs;
use edn_derive::Deserialize;

fn criterion_benchmark(c: &mut Criterion) {
    let edn = edn_str();
    c.bench_function("deser", |b| b.iter(|| edn_rs::from_str::<Entity>(&edn)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

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